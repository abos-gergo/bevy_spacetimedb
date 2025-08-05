use bevy::{log::LogPlugin, prelude::*};
use bevy_spacetimedb::{
    ReadInsertEvent, ReadReducerEvent, ReducerResultEvent, RegisterToTableEvents,
    StdbConnectedEvent, StdbConnection, StdbPlugin, register_reducers, tables,
};
use spacetimedb_sdk::{ReducerEvent, Table};
use stdb::{DbConnection, Player, PlayersTableAccess, Reducer, player_register};

use crate::stdb::{PlanetsTableAccess, RemoteTables, gs_register};

mod stdb;

#[derive(Clone, Debug)]
pub struct RegisterPlayerEvent {
    pub event: ReducerEvent<Reducer>,
    pub id: u64,
}

#[derive(Clone, Debug)]
pub struct GsRegisterEvent {
    pub event: ReducerEvent<Reducer>,
    pub ip: String,
    pub port: u16,
}

pub fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(
            StdbPlugin::default()
                //Eliminated boilerplate here.
                .with_name("http://localhost:3000")
                .with_uri("chat")
                .with_token("Read this value from storage")
                .with_run_fn(DbConnection::run_threaded)
                // I want to eliminate boilerplate (whole with_events part), the commented line below almost works, I could not get the lifetimes to match.
                // Same approach can be made for reducers if this one works.
                // .add_table(RemoteTables::planets, RegisterToTableEvents::all())
                //Old functionality still works.
                .with_events(|plugin, app, db, reducers| {
                    tables!(players);

                    register_reducers!(
                        on_player_register(ctx, id) => RegisterPlayerEvent {
                            event: ctx.event.clone(),
                            id: *id
                        },
                        on_gs_register(ctx, ip, port) => GsRegisterEvent {
                            event: ctx.event.clone(),
                            ip: ip.clone(),
                            port: *port
                        }
                    );
                }),
        )
        .add_systems(
            Update,
            (on_connected, on_register_player, on_gs_register, on_player),
        )
        .run();
}

fn on_connected(
    mut events: EventReader<StdbConnectedEvent>,
    stdb: Res<StdbConnection<DbConnection>>,
) {
    for ev in events.read() {
        info!(
            "Connected to SpacetimeDB with identity: {}",
            ev.identity.to_hex()
        );

        // Call any reducers
        stdb.reducers().player_register(1).unwrap();

        // Subscribe to any tables
        stdb.subscribe()
            .on_applied(|_| info!("Subscription to players applied"))
            .on_error(|_, err| error!("Subscription to players failed for: {}", err))
            .subscribe("SELECT * FROM players");

        // Access your database cache (since it's not yet populated here this line might return 0)
        info!("Players count: {}", stdb.db().players().count());
    }
}

fn on_register_player(mut events: ReadReducerEvent<RegisterPlayerEvent>) {
    for event in events.read() {
        info!("Registered player: {:?}", event);
    }
}

fn on_gs_register(mut events: ReadReducerEvent<GsRegisterEvent>) {
    for event in events.read() {
        info!("Registered game server: {:?}", event);
    }
}

fn on_player(mut events: ReadInsertEvent<Player>) {
    for event in events.read() {
        info!("Player inserted: {:?}", event.row);
    }
}
