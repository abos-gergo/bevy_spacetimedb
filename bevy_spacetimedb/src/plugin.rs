use std::{
    any::{Any, TypeId},
    sync::{
        Mutex,
        mpsc::{Sender, channel},
    },
    thread::JoinHandle,
};

use bevy::{
    app::{App, Plugin},
    platform::collections::HashMap,
};
use bevy_spacetimedb_macros::tables;
use spacetimedb_sdk::{DbConnectionBuilder, DbContext, Table, TableWithPrimaryKey};

use crate::{
    AddEventChannelAppExtensions, DeleteEvent, InsertEvent, InsertUpdateEvent, StdbConnectedEvent,
    StdbConnection, StdbConnectionErrorEvent, StdbDisconnectedEvent, UpdateEvent, events,
};

pub type FnRegisterCallbacks<C, M> = fn(&StdbPlugin<C, M>, &mut App, &<C as DbContext>::DbView);

pub struct StdbPlugin<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M> + DbContext,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> {
    name: String,
    uri: String,
    run_fn: Option<fn(&C) -> JoinHandle<()>>,
    event_senders: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    register_event: Option<FnRegisterCallbacks<C, M>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StdbEvents {
    insert: bool,
    delete: bool,
    update: bool,
    insert_update: bool,
}

impl StdbEvents {
    pub fn all() -> Self {
        Self {
            insert: true,
            delete: true,
            update: true,
            insert_update: true,
        }
    }

    pub fn no_update() -> Self {
        Self {
            insert: true,
            delete: true,
            update: false,
            insert_update: false,
        }
    }
}

impl<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M> + DbContext,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> Default for StdbPlugin<C, M>
{
    fn default() -> Self {
        Self {
            name: Default::default(),
            uri: Default::default(),
            run_fn: None,
            event_senders: Mutex::new(HashMap::new()),
            register_event: None,
        }
    }
}

impl<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M> + DbContext,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> StdbPlugin<C, M>
{
    pub fn with_run_fn(mut self, run_fn: fn(&C) -> JoinHandle<()>) -> Self {
        self.run_fn = Some(run_fn);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_uri(mut self, uri: impl Into<String>) -> Self {
        self.uri = uri.into();
        self
    }

    pub fn with_tables() {
        tables!();
    }

    pub fn add_table<TRow, TTable, F>(mut self, accessor: F, events: StdbEvents) -> Self
    where
        TRow: Send + Sync + Clone + 'static,
        TTable: Table<Row = TRow> + TableWithPrimaryKey<Row = TRow>,
        F: 'static + Fn(&'static C::DbView) -> TTable,
    {
        //Store the accessor inside the struct
        // register_events: Vec<fn(&Self, &mut App, &'static C::DbView)>,
        let register = move |plugin: &Self, app: &mut App, db: &'static C::DbView| {
            let table = accessor(db);
            if events.insert {
                plugin.on_insert(app, &table);
            }
            if events.delete {
                plugin.on_delete(app, &table);
            }
            if events.update {
                plugin.on_update(app, &table);
            }
            if events.insert_update {
                plugin.on_insert_update(app, &table);
            }
        };

        // self.register_events
        //     .lock()
        //     .unwrap()
        //     .push(Box::new(register));

        self
    }

    /// Register a Bevy event of type InsertEvent<TRow> for the `on_insert` event on the provided table.
    fn on_insert<TRow>(&self, app: &mut App, table: &impl Table<Row = TRow>) -> &Self
    where
        TRow: Send + Sync + Clone + 'static,
    {
        let type_id = TypeId::of::<InsertEvent<TRow>>();

        let mut map = self.event_senders.lock().unwrap();

        let sender = map
            .entry(type_id)
            .or_insert_with(|| {
                let (send, recv) = channel::<InsertEvent<TRow>>();
                app.add_event_channel(recv);
                Box::new(send)
            })
            .downcast_ref::<Sender<InsertEvent<TRow>>>()
            .expect("Sender type mismatch")
            .clone();

        table.on_insert(move |_ctx, row| {
            let event = InsertEvent { row: row.clone() };
            let _ = sender.send(event);
        });

        self
    }

    /// Register a Bevy event of type DeleteEvent<TRow> for the `on_delete` event on the provided table.
    fn on_delete<TRow>(&self, app: &mut App, table: &impl Table<Row = TRow>) -> &Self
    where
        TRow: Send + Sync + Clone + 'static,
    {
        let type_id = TypeId::of::<DeleteEvent<TRow>>();

        let mut map = self.event_senders.lock().unwrap();
        let sender = map
            .entry(type_id)
            .or_insert_with(|| {
                let (send, recv) = channel::<DeleteEvent<TRow>>();
                app.add_event_channel(recv);
                Box::new(send)
            })
            .downcast_ref::<Sender<DeleteEvent<TRow>>>()
            .expect("Sender type mismatch")
            .clone();

        table.on_delete(move |_ctx, row| {
            let event = DeleteEvent { row: row.clone() };
            let _ = sender.send(event);
        });

        self
    }

    /// Register a Bevy event of type UpdateEvent<TRow> for the `on_update` event on the provided table.
    fn on_update<TRow, TTable>(&self, app: &mut App, table: &TTable) -> &Self
    where
        TRow: Send + Sync + Clone + 'static,
        TTable: Table<Row = TRow> + TableWithPrimaryKey<Row = TRow>,
    {
        let type_id = TypeId::of::<UpdateEvent<TRow>>();

        let mut map = self.event_senders.lock().unwrap();
        let sender = map
            .entry(type_id)
            .or_insert_with(|| {
                let (send, recv) = channel::<UpdateEvent<TRow>>();
                app.add_event_channel(recv);
                Box::new(send)
            })
            .downcast_ref::<Sender<UpdateEvent<TRow>>>()
            .expect("Sender type mismatch")
            .clone();

        table.on_update(move |_ctx, old, new| {
            let event = UpdateEvent {
                old: old.clone(),
                new: new.clone(),
            };
            let _ = sender.send(event);
        });

        self
    }

    /// Register a Bevy event of type InsertUpdateEvent<TRow> for the `on_insert` and `on_update` events on the provided table.
    fn on_insert_update<TRow, TTable>(&self, app: &mut App, table: &TTable) -> &Self
    where
        TRow: Send + Sync + Clone + 'static,
        TTable: Table<Row = TRow> + TableWithPrimaryKey<Row = TRow>,
    {
        let type_id = TypeId::of::<InsertUpdateEvent<TRow>>();

        let mut map = self.event_senders.lock().unwrap();
        let send = map
            .entry(type_id)
            .or_insert_with(|| {
                let (send, recv) = channel::<InsertUpdateEvent<TRow>>();
                app.add_event_channel(recv);
                Box::new(send)
            })
            .downcast_ref::<Sender<InsertUpdateEvent<TRow>>>()
            .expect("Sender type mismatch")
            .clone();

        let send_update = send.clone();
        table.on_update(move |_ctx, old, new| {
            let event = InsertUpdateEvent {
                old: Some(old.clone()),
                new: new.clone(),
            };
            let _ = send_update.send(event);
        });

        table.on_insert(move |_ctx, row| {
            let event = InsertUpdateEvent {
                old: None,
                new: row.clone(),
            };
            let _ = send.send(event);
        });

        self
    }
}

impl<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M> + DbContext + Sync,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> Plugin for StdbPlugin<C, M>
{
    fn build(&self, app: &mut App) {
        let (send_connected, recv_connected) = channel::<StdbConnectedEvent>();
        let (send_disconnected, recv_disconnected) = channel::<StdbDisconnectedEvent>();
        let (send_connect_error, recv_connect_error) = channel::<StdbConnectionErrorEvent>();

        app.add_event_channel::<StdbConnectionErrorEvent>(recv_connect_error)
            .add_event_channel::<StdbConnectedEvent>(recv_connected)
            .add_event_channel::<StdbDisconnectedEvent>(recv_disconnected);

        let conn = DbConnectionBuilder::<M>::new()
            .with_module_name(self.name.clone())
            .with_uri(self.uri.clone())
            .on_connect_error(move |_ctx, err| {
                send_connect_error
                    .send(StdbConnectionErrorEvent { err })
                    .unwrap();
            })
            .on_disconnect(move |_ctx, err| {
                send_disconnected
                    .send(StdbDisconnectedEvent { err })
                    .unwrap();
            })
            .on_connect(move |_ctx, id, token| {
                send_connected
                    .send(StdbConnectedEvent {
                        identity: id,
                        access_token: token.to_string(),
                    })
                    .unwrap();
            })
            .build()
            .expect("SpacetimeDB connection failed");

        let run_fn = self.run_fn.expect("No run function specified!");
        run_fn(&conn);

        app.insert_resource(StdbConnection::new(conn));
    }
}
