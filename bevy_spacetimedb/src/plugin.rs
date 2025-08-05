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
use spacetimedb_sdk::{DbConnectionBuilder, DbContext};

use crate::{
    AddEventChannelAppExtensions, StdbConnectedEvent, StdbConnection, StdbConnectionErrorEvent,
    StdbDisconnectedEvent,
};

///
pub struct StdbPlugin<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M>,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> {
    name: String,
    uri: String,
    run_fn: Option<fn(&C) -> JoinHandle<()>>,
}

impl<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M>,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> Default for StdbPlugin<C, M>
{
    fn default() -> Self {
        Self {
            name: Default::default(),
            uri: Default::default(),
            run_fn: None,
        }
    }
}

///
impl<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M>,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> StdbPlugin<C, M>
{
    ///
    pub fn with_run_fn(mut self, run_fn: fn(&C) -> JoinHandle<()>) -> Self {
        self.run_fn = Some(run_fn);
        self
    }

    /// .
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// .
    pub fn with_uri(mut self, uri: impl Into<String>) -> Self {
        self.uri = uri.into();
        self
    }

    ///
    pub fn with_tables() {
        tables!();
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

        if let Some(run_fn) = self.run_fn {
            run_fn(&conn);
        }

        app.insert_resource(StdbConnection::new(conn));
    }
}
