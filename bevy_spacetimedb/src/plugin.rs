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

use crate::{StdbConnectedEvent, StdbConnection, StdbConnectionErrorEvent, StdbDisconnectedEvent};

///
pub struct StdbPlugin {}

impl StdbPlugin {
    ///
    pub fn builder<
        C: spacetimedb_sdk::__codegen::DbConnection<Module = M>,
        M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
    >() -> StdbPluginBuilder<C, M> {
        StdbPluginBuilder {
            name: Default::default(),
            uri: Default::default(),
            run_fn: None,
        }
    }
}

impl Plugin for StdbPlugin {
    fn build(&self, app: &mut App) {}
}

///
#[derive(Default)]
pub struct StdbPluginBuilder<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M>,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> {
    name: String,
    uri: String,
    run_fn: Option<fn(&C) -> JoinHandle<()>>,
}

///
impl<
    C: spacetimedb_sdk::__codegen::DbConnection<Module = M>,
    M: spacetimedb_sdk::__codegen::SpacetimeModule<DbConnection = C>,
> StdbPluginBuilder<C, M>
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
