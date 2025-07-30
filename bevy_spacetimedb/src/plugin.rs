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
use spacetimedb_sdk::{DbConnectionBuilder, DbContext};

use crate::{StdbConnectedEvent, StdbConnectionErrorEvent, StdbDisconnectedEvent};

///
pub struct StdbPlugin {}

impl StdbPlugin {
    ///
    pub fn connection() -> StdbPluginConnectionBuilder {
        StdbPluginConnectionBuilder {
            name: Default::default(),
            uri: Default::default(),
        }
    }
}

///
pub struct StdbPluginConnectionBuilder {
    name: String,
    uri: String,
}

///
impl StdbPluginConnectionBuilder {
    ///
    pub fn build<
        C: spacetimedb_sdk::__codegen::DbConnection<Module = M>,
        M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
    >(
        self,
        run_fn: fn(&C) -> JoinHandle<()>,
    ) {
        let (send_connected, recv_connected) = channel::<StdbConnectedEvent>();
        let (send_disconnected, recv_disconnected) = channel::<StdbDisconnectedEvent>();
        let (send_connect_error, recv_connect_error) = channel::<StdbConnectionErrorEvent>();

        let conn = &DbConnectionBuilder::<M>::new()
            .with_module_name(self.name)
            .with_uri(self.uri)
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
            .expect("Connection builder is not set");
        run_fn(conn);
    }

    /// .
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// .
    pub fn uri(mut self, uri: impl Into<String>) -> Self {
        self.uri = uri.into();
        self
    }
}
