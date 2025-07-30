#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use crate::stdb::Reducer;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_spacetimedb::{
    ReducerResultEvent, StdbConnectedEvent, StdbConnection, StdbPlugin,
    register_reducers, tables,
};
use spacetimedb_sdk::{DbConnectionBuilder, ReducerEvent};
use crate::stdb::{
    DbConnection, LobbyTableAccess, UserTableAccess, create_lobby, set_name,
};
use crate::stdb::RemoteModule;
mod stdb {
    #![allow(unused, clippy::all)]
    use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
    pub mod client_connected_reducer {
        #![allow(unused, clippy::all)]
        use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
        #[sats(crate = __lib)]
        pub(super) struct ClientConnectedArgs {}
        impl __lib::ser::Serialize for ClientConnectedArgs {
            fn serialize<S: __lib::ser::Serializer>(
                &self,
                __serializer: S,
            ) -> Result<S::Ok, S::Error> {
                let mut __prod = __serializer.serialize_named_product(0usize)?;
                __lib::ser::SerializeNamedProduct::end(__prod)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::all)]
        const _: () = {
            impl<'de> __lib::de::Deserialize<'de> for ClientConnectedArgs {
                fn deserialize<D: __lib::de::Deserializer<'de>>(
                    deserializer: D,
                ) -> Result<Self, D::Error> {
                    deserializer
                        .deserialize_product(__ProductVisitor {
                            _marker: std::marker::PhantomData::<
                                fn() -> ClientConnectedArgs,
                            >,
                        })
                }
            }
            struct __ProductVisitor {
                _marker: std::marker::PhantomData<fn() -> ClientConnectedArgs>,
            }
            impl<'de> __lib::de::ProductVisitor<'de> for __ProductVisitor {
                type Output = ClientConnectedArgs;
                fn product_name(&self) -> Option<&str> {
                    Some("ClientConnectedArgs")
                }
                fn product_len(&self) -> usize {
                    0usize
                }
                fn visit_seq_product<A: __lib::de::SeqProductAccess<'de>>(
                    self,
                    mut tup: A,
                ) -> Result<Self::Output, A::Error> {
                    Ok(ClientConnectedArgs {})
                }
                fn visit_named_product<A: __lib::de::NamedProductAccess<'de>>(
                    self,
                    mut __prod: A,
                ) -> Result<Self::Output, A::Error> {
                    while let Some(__field) = __lib::de::NamedProductAccess::get_field_ident(
                        &mut __prod,
                        Self {
                            _marker: std::marker::PhantomData,
                        },
                    )? {
                        match __field {}
                    }
                    Ok(ClientConnectedArgs {})
                }
            }
            impl<'de> __lib::de::FieldNameVisitor<'de> for __ProductVisitor {
                type Output = __ProductFieldIdent;
                fn field_names(&self, names: &mut dyn __lib::de::ValidNames) {
                    names.extend::<&[&str]>(&[])
                }
                fn visit<__E: __lib::de::Error>(
                    self,
                    name: &str,
                ) -> Result<Self::Output, __E> {
                    match name {
                        _ => Err(__lib::de::Error::unknown_field_name(name, &self)),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            enum __ProductFieldIdent {}
        };
        #[automatically_derived]
        impl ::core::clone::Clone for ClientConnectedArgs {
            #[inline]
            fn clone(&self) -> ClientConnectedArgs {
                ClientConnectedArgs {}
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ClientConnectedArgs {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ClientConnectedArgs {
            #[inline]
            fn eq(&self, other: &ClientConnectedArgs) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ClientConnectedArgs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "ClientConnectedArgs")
            }
        }
        impl From<ClientConnectedArgs> for super::Reducer {
            fn from(args: ClientConnectedArgs) -> Self {
                Self::ClientConnected
            }
        }
        impl __sdk::InModule for ClientConnectedArgs {
            type Module = super::RemoteModule;
        }
        pub struct ClientConnectedCallbackId(__sdk::CallbackId);
        #[allow(non_camel_case_types)]
        /// Extension trait for access to the reducer `client_connected`.
        ///
        /// Implemented for [`super::RemoteReducers`].
        pub trait client_connected {
            /// Request that the remote module invoke the reducer `client_connected` to run as soon as possible.
            ///
            /// This method returns immediately, and errors only if we are unable to send the request.
            /// The reducer will run asynchronously in the future,
            ///  and its status can be observed by listening for [`Self::on_client_connected`] callbacks.
            fn client_connected(&self) -> __sdk::Result<()>;
            /// Register a callback to run whenever we are notified of an invocation of the reducer `client_connected`.
            ///
            /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
            /// to determine the reducer's status.
            ///
            /// The returned [`ClientConnectedCallbackId`] can be passed to [`Self::remove_on_client_connected`]
            /// to cancel the callback.
            fn on_client_connected(
                &self,
                callback: impl FnMut(&super::ReducerEventContext) + Send + 'static,
            ) -> ClientConnectedCallbackId;
            /// Cancel a callback previously registered by [`Self::on_client_connected`],
            /// causing it not to run in the future.
            fn remove_on_client_connected(&self, callback: ClientConnectedCallbackId);
        }
        impl client_connected for super::RemoteReducers {
            fn client_connected(&self) -> __sdk::Result<()> {
                self.imp.call_reducer("client_connected", ClientConnectedArgs {})
            }
            fn on_client_connected(
                &self,
                mut callback: impl FnMut(&super::ReducerEventContext) + Send + 'static,
            ) -> ClientConnectedCallbackId {
                ClientConnectedCallbackId(
                    self
                        .imp
                        .on_reducer(
                            "client_connected",
                            Box::new(move |ctx: &super::ReducerEventContext| {
                                let super::ReducerEventContext {
                                    event: __sdk::ReducerEvent {
                                        reducer: super::Reducer::ClientConnected {},
                                        ..
                                    },
                                    ..
                                } = ctx else {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                };
                                callback(ctx)
                            }),
                        ),
                )
            }
            fn remove_on_client_connected(&self, callback: ClientConnectedCallbackId) {
                self.imp.remove_on_reducer("client_connected", callback.0)
            }
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        /// Extension trait for setting the call-flags for the reducer `client_connected`.
        ///
        /// Implemented for [`super::SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub trait set_flags_for_client_connected {
            /// Set the call-reducer flags for the reducer `client_connected` to `flags`.
            ///
            /// This type is currently unstable and may be removed without a major version bump.
            fn client_connected(&self, flags: __ws::CallReducerFlags);
        }
        impl set_flags_for_client_connected for super::SetReducerFlags {
            fn client_connected(&self, flags: __ws::CallReducerFlags) {
                self.imp.set_call_reducer_flags("client_connected", flags);
            }
        }
    }
    pub mod create_lobby_reducer {
        #![allow(unused, clippy::all)]
        use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
        #[sats(crate = __lib)]
        pub(super) struct CreateLobbyArgs {}
        impl __lib::ser::Serialize for CreateLobbyArgs {
            fn serialize<S: __lib::ser::Serializer>(
                &self,
                __serializer: S,
            ) -> Result<S::Ok, S::Error> {
                let mut __prod = __serializer.serialize_named_product(0usize)?;
                __lib::ser::SerializeNamedProduct::end(__prod)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::all)]
        const _: () = {
            impl<'de> __lib::de::Deserialize<'de> for CreateLobbyArgs {
                fn deserialize<D: __lib::de::Deserializer<'de>>(
                    deserializer: D,
                ) -> Result<Self, D::Error> {
                    deserializer
                        .deserialize_product(__ProductVisitor {
                            _marker: std::marker::PhantomData::<fn() -> CreateLobbyArgs>,
                        })
                }
            }
            struct __ProductVisitor {
                _marker: std::marker::PhantomData<fn() -> CreateLobbyArgs>,
            }
            impl<'de> __lib::de::ProductVisitor<'de> for __ProductVisitor {
                type Output = CreateLobbyArgs;
                fn product_name(&self) -> Option<&str> {
                    Some("CreateLobbyArgs")
                }
                fn product_len(&self) -> usize {
                    0usize
                }
                fn visit_seq_product<A: __lib::de::SeqProductAccess<'de>>(
                    self,
                    mut tup: A,
                ) -> Result<Self::Output, A::Error> {
                    Ok(CreateLobbyArgs {})
                }
                fn visit_named_product<A: __lib::de::NamedProductAccess<'de>>(
                    self,
                    mut __prod: A,
                ) -> Result<Self::Output, A::Error> {
                    while let Some(__field) = __lib::de::NamedProductAccess::get_field_ident(
                        &mut __prod,
                        Self {
                            _marker: std::marker::PhantomData,
                        },
                    )? {
                        match __field {}
                    }
                    Ok(CreateLobbyArgs {})
                }
            }
            impl<'de> __lib::de::FieldNameVisitor<'de> for __ProductVisitor {
                type Output = __ProductFieldIdent;
                fn field_names(&self, names: &mut dyn __lib::de::ValidNames) {
                    names.extend::<&[&str]>(&[])
                }
                fn visit<__E: __lib::de::Error>(
                    self,
                    name: &str,
                ) -> Result<Self::Output, __E> {
                    match name {
                        _ => Err(__lib::de::Error::unknown_field_name(name, &self)),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            enum __ProductFieldIdent {}
        };
        #[automatically_derived]
        impl ::core::clone::Clone for CreateLobbyArgs {
            #[inline]
            fn clone(&self) -> CreateLobbyArgs {
                CreateLobbyArgs {}
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CreateLobbyArgs {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CreateLobbyArgs {
            #[inline]
            fn eq(&self, other: &CreateLobbyArgs) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreateLobbyArgs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "CreateLobbyArgs")
            }
        }
        impl From<CreateLobbyArgs> for super::Reducer {
            fn from(args: CreateLobbyArgs) -> Self {
                Self::CreateLobby
            }
        }
        impl __sdk::InModule for CreateLobbyArgs {
            type Module = super::RemoteModule;
        }
        pub struct CreateLobbyCallbackId(__sdk::CallbackId);
        #[allow(non_camel_case_types)]
        /// Extension trait for access to the reducer `create_lobby`.
        ///
        /// Implemented for [`super::RemoteReducers`].
        pub trait create_lobby {
            /// Request that the remote module invoke the reducer `create_lobby` to run as soon as possible.
            ///
            /// This method returns immediately, and errors only if we are unable to send the request.
            /// The reducer will run asynchronously in the future,
            ///  and its status can be observed by listening for [`Self::on_create_lobby`] callbacks.
            fn create_lobby(&self) -> __sdk::Result<()>;
            /// Register a callback to run whenever we are notified of an invocation of the reducer `create_lobby`.
            ///
            /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
            /// to determine the reducer's status.
            ///
            /// The returned [`CreateLobbyCallbackId`] can be passed to [`Self::remove_on_create_lobby`]
            /// to cancel the callback.
            fn on_create_lobby(
                &self,
                callback: impl FnMut(&super::ReducerEventContext) + Send + 'static,
            ) -> CreateLobbyCallbackId;
            /// Cancel a callback previously registered by [`Self::on_create_lobby`],
            /// causing it not to run in the future.
            fn remove_on_create_lobby(&self, callback: CreateLobbyCallbackId);
        }
        impl create_lobby for super::RemoteReducers {
            fn create_lobby(&self) -> __sdk::Result<()> {
                self.imp.call_reducer("create_lobby", CreateLobbyArgs {})
            }
            fn on_create_lobby(
                &self,
                mut callback: impl FnMut(&super::ReducerEventContext) + Send + 'static,
            ) -> CreateLobbyCallbackId {
                CreateLobbyCallbackId(
                    self
                        .imp
                        .on_reducer(
                            "create_lobby",
                            Box::new(move |ctx: &super::ReducerEventContext| {
                                let super::ReducerEventContext {
                                    event: __sdk::ReducerEvent {
                                        reducer: super::Reducer::CreateLobby {},
                                        ..
                                    },
                                    ..
                                } = ctx else {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                };
                                callback(ctx)
                            }),
                        ),
                )
            }
            fn remove_on_create_lobby(&self, callback: CreateLobbyCallbackId) {
                self.imp.remove_on_reducer("create_lobby", callback.0)
            }
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        /// Extension trait for setting the call-flags for the reducer `create_lobby`.
        ///
        /// Implemented for [`super::SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub trait set_flags_for_create_lobby {
            /// Set the call-reducer flags for the reducer `create_lobby` to `flags`.
            ///
            /// This type is currently unstable and may be removed without a major version bump.
            fn create_lobby(&self, flags: __ws::CallReducerFlags);
        }
        impl set_flags_for_create_lobby for super::SetReducerFlags {
            fn create_lobby(&self, flags: __ws::CallReducerFlags) {
                self.imp.set_call_reducer_flags("create_lobby", flags);
            }
        }
    }
    pub mod identity_disconnected_reducer {
        #![allow(unused, clippy::all)]
        use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
        #[sats(crate = __lib)]
        pub(super) struct IdentityDisconnectedArgs {}
        impl __lib::ser::Serialize for IdentityDisconnectedArgs {
            fn serialize<S: __lib::ser::Serializer>(
                &self,
                __serializer: S,
            ) -> Result<S::Ok, S::Error> {
                let mut __prod = __serializer.serialize_named_product(0usize)?;
                __lib::ser::SerializeNamedProduct::end(__prod)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::all)]
        const _: () = {
            impl<'de> __lib::de::Deserialize<'de> for IdentityDisconnectedArgs {
                fn deserialize<D: __lib::de::Deserializer<'de>>(
                    deserializer: D,
                ) -> Result<Self, D::Error> {
                    deserializer
                        .deserialize_product(__ProductVisitor {
                            _marker: std::marker::PhantomData::<
                                fn() -> IdentityDisconnectedArgs,
                            >,
                        })
                }
            }
            struct __ProductVisitor {
                _marker: std::marker::PhantomData<fn() -> IdentityDisconnectedArgs>,
            }
            impl<'de> __lib::de::ProductVisitor<'de> for __ProductVisitor {
                type Output = IdentityDisconnectedArgs;
                fn product_name(&self) -> Option<&str> {
                    Some("IdentityDisconnectedArgs")
                }
                fn product_len(&self) -> usize {
                    0usize
                }
                fn visit_seq_product<A: __lib::de::SeqProductAccess<'de>>(
                    self,
                    mut tup: A,
                ) -> Result<Self::Output, A::Error> {
                    Ok(IdentityDisconnectedArgs {})
                }
                fn visit_named_product<A: __lib::de::NamedProductAccess<'de>>(
                    self,
                    mut __prod: A,
                ) -> Result<Self::Output, A::Error> {
                    while let Some(__field) = __lib::de::NamedProductAccess::get_field_ident(
                        &mut __prod,
                        Self {
                            _marker: std::marker::PhantomData,
                        },
                    )? {
                        match __field {}
                    }
                    Ok(IdentityDisconnectedArgs {})
                }
            }
            impl<'de> __lib::de::FieldNameVisitor<'de> for __ProductVisitor {
                type Output = __ProductFieldIdent;
                fn field_names(&self, names: &mut dyn __lib::de::ValidNames) {
                    names.extend::<&[&str]>(&[])
                }
                fn visit<__E: __lib::de::Error>(
                    self,
                    name: &str,
                ) -> Result<Self::Output, __E> {
                    match name {
                        _ => Err(__lib::de::Error::unknown_field_name(name, &self)),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            enum __ProductFieldIdent {}
        };
        #[automatically_derived]
        impl ::core::clone::Clone for IdentityDisconnectedArgs {
            #[inline]
            fn clone(&self) -> IdentityDisconnectedArgs {
                IdentityDisconnectedArgs {}
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for IdentityDisconnectedArgs {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for IdentityDisconnectedArgs {
            #[inline]
            fn eq(&self, other: &IdentityDisconnectedArgs) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdentityDisconnectedArgs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdentityDisconnectedArgs")
            }
        }
        impl From<IdentityDisconnectedArgs> for super::Reducer {
            fn from(args: IdentityDisconnectedArgs) -> Self {
                Self::IdentityDisconnected
            }
        }
        impl __sdk::InModule for IdentityDisconnectedArgs {
            type Module = super::RemoteModule;
        }
        pub struct IdentityDisconnectedCallbackId(__sdk::CallbackId);
        #[allow(non_camel_case_types)]
        /// Extension trait for access to the reducer `identity_disconnected`.
        ///
        /// Implemented for [`super::RemoteReducers`].
        pub trait identity_disconnected {
            /// Request that the remote module invoke the reducer `identity_disconnected` to run as soon as possible.
            ///
            /// This method returns immediately, and errors only if we are unable to send the request.
            /// The reducer will run asynchronously in the future,
            ///  and its status can be observed by listening for [`Self::on_identity_disconnected`] callbacks.
            fn identity_disconnected(&self) -> __sdk::Result<()>;
            /// Register a callback to run whenever we are notified of an invocation of the reducer `identity_disconnected`.
            ///
            /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
            /// to determine the reducer's status.
            ///
            /// The returned [`IdentityDisconnectedCallbackId`] can be passed to [`Self::remove_on_identity_disconnected`]
            /// to cancel the callback.
            fn on_identity_disconnected(
                &self,
                callback: impl FnMut(&super::ReducerEventContext) + Send + 'static,
            ) -> IdentityDisconnectedCallbackId;
            /// Cancel a callback previously registered by [`Self::on_identity_disconnected`],
            /// causing it not to run in the future.
            fn remove_on_identity_disconnected(
                &self,
                callback: IdentityDisconnectedCallbackId,
            );
        }
        impl identity_disconnected for super::RemoteReducers {
            fn identity_disconnected(&self) -> __sdk::Result<()> {
                self.imp
                    .call_reducer("identity_disconnected", IdentityDisconnectedArgs {})
            }
            fn on_identity_disconnected(
                &self,
                mut callback: impl FnMut(&super::ReducerEventContext) + Send + 'static,
            ) -> IdentityDisconnectedCallbackId {
                IdentityDisconnectedCallbackId(
                    self
                        .imp
                        .on_reducer(
                            "identity_disconnected",
                            Box::new(move |ctx: &super::ReducerEventContext| {
                                let super::ReducerEventContext {
                                    event: __sdk::ReducerEvent {
                                        reducer: super::Reducer::IdentityDisconnected {},
                                        ..
                                    },
                                    ..
                                } = ctx else {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                };
                                callback(ctx)
                            }),
                        ),
                )
            }
            fn remove_on_identity_disconnected(
                &self,
                callback: IdentityDisconnectedCallbackId,
            ) {
                self.imp.remove_on_reducer("identity_disconnected", callback.0)
            }
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        /// Extension trait for setting the call-flags for the reducer `identity_disconnected`.
        ///
        /// Implemented for [`super::SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub trait set_flags_for_identity_disconnected {
            /// Set the call-reducer flags for the reducer `identity_disconnected` to `flags`.
            ///
            /// This type is currently unstable and may be removed without a major version bump.
            fn identity_disconnected(&self, flags: __ws::CallReducerFlags);
        }
        impl set_flags_for_identity_disconnected for super::SetReducerFlags {
            fn identity_disconnected(&self, flags: __ws::CallReducerFlags) {
                self.imp.set_call_reducer_flags("identity_disconnected", flags);
            }
        }
    }
    pub mod lobby_table {
        #![allow(unused, clippy::all)]
        use super::lobby_type::Lobby;
        use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
        /// Table handle for the table `lobby`.
        ///
        /// Obtain a handle from the [`LobbyTableAccess::lobby`] method on [`super::RemoteTables`],
        /// like `ctx.db.lobby()`.
        ///
        /// Users are encouraged not to explicitly reference this type,
        /// but to directly chain method calls,
        /// like `ctx.db.lobby().on_insert(...)`.
        pub struct LobbyTableHandle<'ctx> {
            imp: __sdk::TableHandle<Lobby>,
            ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
        }
        #[allow(non_camel_case_types)]
        /// Extension trait for access to the table `lobby`.
        ///
        /// Implemented for [`super::RemoteTables`].
        pub trait LobbyTableAccess {
            #[allow(non_snake_case)]
            /// Obtain a [`LobbyTableHandle`], which mediates access to the table `lobby`.
            fn lobby(&self) -> LobbyTableHandle<'_>;
        }
        impl LobbyTableAccess for super::RemoteTables {
            fn lobby(&self) -> LobbyTableHandle<'_> {
                LobbyTableHandle {
                    imp: self.imp.get_table::<Lobby>("lobby"),
                    ctx: std::marker::PhantomData,
                }
            }
        }
        pub struct LobbyInsertCallbackId(__sdk::CallbackId);
        pub struct LobbyDeleteCallbackId(__sdk::CallbackId);
        impl<'ctx> __sdk::Table for LobbyTableHandle<'ctx> {
            type Row = Lobby;
            type EventContext = super::EventContext;
            fn count(&self) -> u64 {
                self.imp.count()
            }
            fn iter(&self) -> impl Iterator<Item = Lobby> + '_ {
                self.imp.iter()
            }
            type InsertCallbackId = LobbyInsertCallbackId;
            fn on_insert(
                &self,
                callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
            ) -> LobbyInsertCallbackId {
                LobbyInsertCallbackId(self.imp.on_insert(Box::new(callback)))
            }
            fn remove_on_insert(&self, callback: LobbyInsertCallbackId) {
                self.imp.remove_on_insert(callback.0)
            }
            type DeleteCallbackId = LobbyDeleteCallbackId;
            fn on_delete(
                &self,
                callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
            ) -> LobbyDeleteCallbackId {
                LobbyDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
            }
            fn remove_on_delete(&self, callback: LobbyDeleteCallbackId) {
                self.imp.remove_on_delete(callback.0)
            }
        }
        #[doc(hidden)]
        pub(super) fn register_table(
            client_cache: &mut __sdk::ClientCache<super::RemoteModule>,
        ) {
            let _table = client_cache.get_or_make_table::<Lobby>("lobby");
            _table.add_unique_constraint::<u64>("id", |row| &row.id);
            _table.add_unique_constraint::<__sdk::Identity>("host", |row| &row.host);
        }
        pub struct LobbyUpdateCallbackId(__sdk::CallbackId);
        impl<'ctx> __sdk::TableWithPrimaryKey for LobbyTableHandle<'ctx> {
            type UpdateCallbackId = LobbyUpdateCallbackId;
            fn on_update(
                &self,
                callback: impl FnMut(
                    &Self::EventContext,
                    &Self::Row,
                    &Self::Row,
                ) + Send + 'static,
            ) -> LobbyUpdateCallbackId {
                LobbyUpdateCallbackId(self.imp.on_update(Box::new(callback)))
            }
            fn remove_on_update(&self, callback: LobbyUpdateCallbackId) {
                self.imp.remove_on_update(callback.0)
            }
        }
        #[doc(hidden)]
        pub(super) fn parse_table_update(
            raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
        ) -> __sdk::Result<__sdk::TableUpdate<Lobby>> {
            __sdk::TableUpdate::parse_table_update(raw_updates)
                .map_err(|e| {
                    __sdk::InternalError::failed_parse(
                            "TableUpdate<Lobby>",
                            "TableUpdate",
                        )
                        .with_cause(e)
                        .into()
                })
        }
        /// Access to the `id` unique index on the table `lobby`,
        /// which allows point queries on the field of the same name
        /// via the [`LobbyIdUnique::find`] method.
        ///
        /// Users are encouraged not to explicitly reference this type,
        /// but to directly chain method calls,
        /// like `ctx.db.lobby().id().find(...)`.
        pub struct LobbyIdUnique<'ctx> {
            imp: __sdk::UniqueConstraintHandle<Lobby, u64>,
            phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
        }
        impl<'ctx> LobbyTableHandle<'ctx> {
            /// Get a handle on the `id` unique index on the table `lobby`.
            pub fn id(&self) -> LobbyIdUnique<'ctx> {
                LobbyIdUnique {
                    imp: self.imp.get_unique_constraint::<u64>("id"),
                    phantom: std::marker::PhantomData,
                }
            }
        }
        impl<'ctx> LobbyIdUnique<'ctx> {
            /// Find the subscribed row whose `id` column value is equal to `col_val`,
            /// if such a row is present in the client cache.
            pub fn find(&self, col_val: &u64) -> Option<Lobby> {
                self.imp.find(col_val)
            }
        }
        /// Access to the `host` unique index on the table `lobby`,
        /// which allows point queries on the field of the same name
        /// via the [`LobbyHostUnique::find`] method.
        ///
        /// Users are encouraged not to explicitly reference this type,
        /// but to directly chain method calls,
        /// like `ctx.db.lobby().host().find(...)`.
        pub struct LobbyHostUnique<'ctx> {
            imp: __sdk::UniqueConstraintHandle<Lobby, __sdk::Identity>,
            phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
        }
        impl<'ctx> LobbyTableHandle<'ctx> {
            /// Get a handle on the `host` unique index on the table `lobby`.
            pub fn host(&self) -> LobbyHostUnique<'ctx> {
                LobbyHostUnique {
                    imp: self.imp.get_unique_constraint::<__sdk::Identity>("host"),
                    phantom: std::marker::PhantomData,
                }
            }
        }
        impl<'ctx> LobbyHostUnique<'ctx> {
            /// Find the subscribed row whose `host` column value is equal to `col_val`,
            /// if such a row is present in the client cache.
            pub fn find(&self, col_val: &__sdk::Identity) -> Option<Lobby> {
                self.imp.find(col_val)
            }
        }
    }
    pub mod lobby_type {
        #![allow(unused, clippy::all)]
        use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
        #[sats(crate = __lib)]
        pub struct Lobby {
            pub id: u64,
            pub host: __sdk::Identity,
            pub guest: Option<__sdk::Identity>,
            pub created_at: __sdk::Timestamp,
        }
        impl __lib::ser::Serialize for Lobby {
            fn serialize<S: __lib::ser::Serializer>(
                &self,
                __serializer: S,
            ) -> Result<S::Ok, S::Error> {
                let mut __prod = __serializer.serialize_named_product(4usize)?;
                __lib::ser::SerializeNamedProduct::serialize_element::<
                    u64,
                >(&mut __prod, Some("id"), &self.id)?;
                __lib::ser::SerializeNamedProduct::serialize_element::<
                    __sdk::Identity,
                >(&mut __prod, Some("host"), &self.host)?;
                __lib::ser::SerializeNamedProduct::serialize_element::<
                    Option<__sdk::Identity>,
                >(&mut __prod, Some("guest"), &self.guest)?;
                __lib::ser::SerializeNamedProduct::serialize_element::<
                    __sdk::Timestamp,
                >(&mut __prod, Some("created_at"), &self.created_at)?;
                __lib::ser::SerializeNamedProduct::end(__prod)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::all)]
        const _: () = {
            impl<'de> __lib::de::Deserialize<'de> for Lobby {
                fn deserialize<D: __lib::de::Deserializer<'de>>(
                    deserializer: D,
                ) -> Result<Self, D::Error> {
                    deserializer
                        .deserialize_product(__ProductVisitor {
                            _marker: std::marker::PhantomData::<fn() -> Lobby>,
                        })
                }
            }
            struct __ProductVisitor {
                _marker: std::marker::PhantomData<fn() -> Lobby>,
            }
            impl<'de> __lib::de::ProductVisitor<'de> for __ProductVisitor {
                type Output = Lobby;
                fn product_name(&self) -> Option<&str> {
                    Some("Lobby")
                }
                fn product_len(&self) -> usize {
                    4usize
                }
                fn visit_seq_product<A: __lib::de::SeqProductAccess<'de>>(
                    self,
                    mut tup: A,
                ) -> Result<Self::Output, A::Error> {
                    Ok(Lobby {
                        id: tup
                            .next_element::<u64>()?
                            .ok_or_else(|| __lib::de::Error::invalid_product_length(
                                0usize,
                                &self,
                            ))?,
                        host: tup
                            .next_element::<__sdk::Identity>()?
                            .ok_or_else(|| __lib::de::Error::invalid_product_length(
                                1usize,
                                &self,
                            ))?,
                        guest: tup
                            .next_element::<Option<__sdk::Identity>>()?
                            .ok_or_else(|| __lib::de::Error::invalid_product_length(
                                2usize,
                                &self,
                            ))?,
                        created_at: tup
                            .next_element::<__sdk::Timestamp>()?
                            .ok_or_else(|| __lib::de::Error::invalid_product_length(
                                3usize,
                                &self,
                            ))?,
                    })
                }
                fn visit_named_product<A: __lib::de::NamedProductAccess<'de>>(
                    self,
                    mut __prod: A,
                ) -> Result<Self::Output, A::Error> {
                    let mut id = None;
                    let mut host = None;
                    let mut guest = None;
                    let mut created_at = None;
                    while let Some(__field) = __lib::de::NamedProductAccess::get_field_ident(
                        &mut __prod,
                        Self {
                            _marker: std::marker::PhantomData,
                        },
                    )? {
                        match __field {
                            __ProductFieldIdent::id => {
                                if id.is_some() {
                                    return Err(
                                        __lib::de::Error::duplicate_field(0usize, Some("id"), &self),
                                    );
                                }
                                id = Some(
                                    __lib::de::NamedProductAccess::get_field_value::<
                                        u64,
                                    >(&mut __prod)?,
                                );
                            }
                            __ProductFieldIdent::host => {
                                if host.is_some() {
                                    return Err(
                                        __lib::de::Error::duplicate_field(
                                            1usize,
                                            Some("host"),
                                            &self,
                                        ),
                                    );
                                }
                                host = Some(
                                    __lib::de::NamedProductAccess::get_field_value::<
                                        __sdk::Identity,
                                    >(&mut __prod)?,
                                );
                            }
                            __ProductFieldIdent::guest => {
                                if guest.is_some() {
                                    return Err(
                                        __lib::de::Error::duplicate_field(
                                            2usize,
                                            Some("guest"),
                                            &self,
                                        ),
                                    );
                                }
                                guest = Some(
                                    __lib::de::NamedProductAccess::get_field_value::<
                                        Option<__sdk::Identity>,
                                    >(&mut __prod)?,
                                );
                            }
                            __ProductFieldIdent::created_at => {
                                if created_at.is_some() {
                                    return Err(
                                        __lib::de::Error::duplicate_field(
                                            3usize,
                                            Some("created_at"),
                                            &self,
                                        ),
                                    );
                                }
                                created_at = Some(
                                    __lib::de::NamedProductAccess::get_field_value::<
                                        __sdk::Timestamp,
                                    >(&mut __prod)?,
                                );
                            }
                        }
                    }
                    Ok(Lobby {
                        id: id
                            .ok_or_else(|| __lib::de::Error::missing_field(
                                0usize,
                                Some("id"),
                                &self,
                            ))?,
                        host: host
                            .ok_or_else(|| __lib::de::Error::missing_field(
                                1usize,
                                Some("host"),
                                &self,
                            ))?,
                        guest: guest
                            .ok_or_else(|| __lib::de::Error::missing_field(
                                2usize,
                                Some("guest"),
                                &self,
                            ))?,
                        created_at: created_at
                            .ok_or_else(|| __lib::de::Error::missing_field(
                                3usize,
                                Some("created_at"),
                                &self,
                            ))?,
                    })
                }
            }
            impl<'de> __lib::de::FieldNameVisitor<'de> for __ProductVisitor {
                type Output = __ProductFieldIdent;
                fn field_names(&self, names: &mut dyn __lib::de::ValidNames) {
                    names.extend::<&[&str]>(&["id", "host", "guest", "created_at"])
                }
                fn visit<__E: __lib::de::Error>(
                    self,
                    name: &str,
                ) -> Result<Self::Output, __E> {
                    match name {
                        "id" => Ok(__ProductFieldIdent::id),
                        "host" => Ok(__ProductFieldIdent::host),
                        "guest" => Ok(__ProductFieldIdent::guest),
                        "created_at" => Ok(__ProductFieldIdent::created_at),
                        _ => Err(__lib::de::Error::unknown_field_name(name, &self)),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            enum __ProductFieldIdent {
                id,
                host,
                guest,
                created_at,
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for Lobby {
            #[inline]
            fn clone(&self) -> Lobby {
                Lobby {
                    id: ::core::clone::Clone::clone(&self.id),
                    host: ::core::clone::Clone::clone(&self.host),
                    guest: ::core::clone::Clone::clone(&self.guest),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Lobby {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Lobby {
            #[inline]
            fn eq(&self, other: &Lobby) -> bool {
                self.id == other.id && self.host == other.host
                    && self.guest == other.guest && self.created_at == other.created_at
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Lobby {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Lobby",
                    "id",
                    &self.id,
                    "host",
                    &self.host,
                    "guest",
                    &self.guest,
                    "created_at",
                    &&self.created_at,
                )
            }
        }
        impl __sdk::InModule for Lobby {
            type Module = super::RemoteModule;
        }
    }
    pub mod set_name_reducer {
        #![allow(unused, clippy::all)]
        use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
        #[sats(crate = __lib)]
        pub(super) struct SetNameArgs {
            pub name: String,
        }
        impl __lib::ser::Serialize for SetNameArgs {
            fn serialize<S: __lib::ser::Serializer>(
                &self,
                __serializer: S,
            ) -> Result<S::Ok, S::Error> {
                let mut __prod = __serializer.serialize_named_product(1usize)?;
                __lib::ser::SerializeNamedProduct::serialize_element::<
                    String,
                >(&mut __prod, Some("name"), &self.name)?;
                __lib::ser::SerializeNamedProduct::end(__prod)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::all)]
        const _: () = {
            impl<'de> __lib::de::Deserialize<'de> for SetNameArgs {
                fn deserialize<D: __lib::de::Deserializer<'de>>(
                    deserializer: D,
                ) -> Result<Self, D::Error> {
                    deserializer
                        .deserialize_product(__ProductVisitor {
                            _marker: std::marker::PhantomData::<fn() -> SetNameArgs>,
                        })
                }
            }
            struct __ProductVisitor {
                _marker: std::marker::PhantomData<fn() -> SetNameArgs>,
            }
            impl<'de> __lib::de::ProductVisitor<'de> for __ProductVisitor {
                type Output = SetNameArgs;
                fn product_name(&self) -> Option<&str> {
                    Some("SetNameArgs")
                }
                fn product_len(&self) -> usize {
                    1usize
                }
                fn visit_seq_product<A: __lib::de::SeqProductAccess<'de>>(
                    self,
                    mut tup: A,
                ) -> Result<Self::Output, A::Error> {
                    Ok(SetNameArgs {
                        name: tup
                            .next_element::<String>()?
                            .ok_or_else(|| __lib::de::Error::invalid_product_length(
                                0usize,
                                &self,
                            ))?,
                    })
                }
                fn visit_named_product<A: __lib::de::NamedProductAccess<'de>>(
                    self,
                    mut __prod: A,
                ) -> Result<Self::Output, A::Error> {
                    let mut name = None;
                    while let Some(__field) = __lib::de::NamedProductAccess::get_field_ident(
                        &mut __prod,
                        Self {
                            _marker: std::marker::PhantomData,
                        },
                    )? {
                        match __field {
                            __ProductFieldIdent::name => {
                                if name.is_some() {
                                    return Err(
                                        __lib::de::Error::duplicate_field(
                                            0usize,
                                            Some("name"),
                                            &self,
                                        ),
                                    );
                                }
                                name = Some(
                                    __lib::de::NamedProductAccess::get_field_value::<
                                        String,
                                    >(&mut __prod)?,
                                );
                            }
                        }
                    }
                    Ok(SetNameArgs {
                        name: name
                            .ok_or_else(|| __lib::de::Error::missing_field(
                                0usize,
                                Some("name"),
                                &self,
                            ))?,
                    })
                }
            }
            impl<'de> __lib::de::FieldNameVisitor<'de> for __ProductVisitor {
                type Output = __ProductFieldIdent;
                fn field_names(&self, names: &mut dyn __lib::de::ValidNames) {
                    names.extend::<&[&str]>(&["name"])
                }
                fn visit<__E: __lib::de::Error>(
                    self,
                    name: &str,
                ) -> Result<Self::Output, __E> {
                    match name {
                        "name" => Ok(__ProductFieldIdent::name),
                        _ => Err(__lib::de::Error::unknown_field_name(name, &self)),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            enum __ProductFieldIdent {
                name,
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for SetNameArgs {
            #[inline]
            fn clone(&self) -> SetNameArgs {
                SetNameArgs {
                    name: ::core::clone::Clone::clone(&self.name),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SetNameArgs {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SetNameArgs {
            #[inline]
            fn eq(&self, other: &SetNameArgs) -> bool {
                self.name == other.name
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SetNameArgs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SetNameArgs",
                    "name",
                    &&self.name,
                )
            }
        }
        impl From<SetNameArgs> for super::Reducer {
            fn from(args: SetNameArgs) -> Self {
                Self::SetName { name: args.name }
            }
        }
        impl __sdk::InModule for SetNameArgs {
            type Module = super::RemoteModule;
        }
        pub struct SetNameCallbackId(__sdk::CallbackId);
        #[allow(non_camel_case_types)]
        /// Extension trait for access to the reducer `set_name`.
        ///
        /// Implemented for [`super::RemoteReducers`].
        pub trait set_name {
            /// Request that the remote module invoke the reducer `set_name` to run as soon as possible.
            ///
            /// This method returns immediately, and errors only if we are unable to send the request.
            /// The reducer will run asynchronously in the future,
            ///  and its status can be observed by listening for [`Self::on_set_name`] callbacks.
            fn set_name(&self, name: String) -> __sdk::Result<()>;
            /// Register a callback to run whenever we are notified of an invocation of the reducer `set_name`.
            ///
            /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
            /// to determine the reducer's status.
            ///
            /// The returned [`SetNameCallbackId`] can be passed to [`Self::remove_on_set_name`]
            /// to cancel the callback.
            fn on_set_name(
                &self,
                callback: impl FnMut(
                    &super::ReducerEventContext,
                    &String,
                ) + Send + 'static,
            ) -> SetNameCallbackId;
            /// Cancel a callback previously registered by [`Self::on_set_name`],
            /// causing it not to run in the future.
            fn remove_on_set_name(&self, callback: SetNameCallbackId);
        }
        impl set_name for super::RemoteReducers {
            fn set_name(&self, name: String) -> __sdk::Result<()> {
                self.imp.call_reducer("set_name", SetNameArgs { name })
            }
            fn on_set_name(
                &self,
                mut callback: impl FnMut(
                    &super::ReducerEventContext,
                    &String,
                ) + Send + 'static,
            ) -> SetNameCallbackId {
                SetNameCallbackId(
                    self
                        .imp
                        .on_reducer(
                            "set_name",
                            Box::new(move |ctx: &super::ReducerEventContext| {
                                let super::ReducerEventContext {
                                    event: __sdk::ReducerEvent {
                                        reducer: super::Reducer::SetName { name },
                                        ..
                                    },
                                    ..
                                } = ctx else {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                };
                                callback(ctx, name)
                            }),
                        ),
                )
            }
            fn remove_on_set_name(&self, callback: SetNameCallbackId) {
                self.imp.remove_on_reducer("set_name", callback.0)
            }
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        /// Extension trait for setting the call-flags for the reducer `set_name`.
        ///
        /// Implemented for [`super::SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub trait set_flags_for_set_name {
            /// Set the call-reducer flags for the reducer `set_name` to `flags`.
            ///
            /// This type is currently unstable and may be removed without a major version bump.
            fn set_name(&self, flags: __ws::CallReducerFlags);
        }
        impl set_flags_for_set_name for super::SetReducerFlags {
            fn set_name(&self, flags: __ws::CallReducerFlags) {
                self.imp.set_call_reducer_flags("set_name", flags);
            }
        }
    }
    pub mod user_table {
        #![allow(unused, clippy::all)]
        use super::user_type::User;
        use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
        /// Table handle for the table `user`.
        ///
        /// Obtain a handle from the [`UserTableAccess::user`] method on [`super::RemoteTables`],
        /// like `ctx.db.user()`.
        ///
        /// Users are encouraged not to explicitly reference this type,
        /// but to directly chain method calls,
        /// like `ctx.db.user().on_insert(...)`.
        pub struct UserTableHandle<'ctx> {
            imp: __sdk::TableHandle<User>,
            ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
        }
        #[allow(non_camel_case_types)]
        /// Extension trait for access to the table `user`.
        ///
        /// Implemented for [`super::RemoteTables`].
        pub trait UserTableAccess {
            #[allow(non_snake_case)]
            /// Obtain a [`UserTableHandle`], which mediates access to the table `user`.
            fn user(&self) -> UserTableHandle<'_>;
        }
        impl UserTableAccess for super::RemoteTables {
            fn user(&self) -> UserTableHandle<'_> {
                UserTableHandle {
                    imp: self.imp.get_table::<User>("user"),
                    ctx: std::marker::PhantomData,
                }
            }
        }
        pub struct UserInsertCallbackId(__sdk::CallbackId);
        pub struct UserDeleteCallbackId(__sdk::CallbackId);
        impl<'ctx> __sdk::Table for UserTableHandle<'ctx> {
            type Row = User;
            type EventContext = super::EventContext;
            fn count(&self) -> u64 {
                self.imp.count()
            }
            fn iter(&self) -> impl Iterator<Item = User> + '_ {
                self.imp.iter()
            }
            type InsertCallbackId = UserInsertCallbackId;
            fn on_insert(
                &self,
                callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
            ) -> UserInsertCallbackId {
                UserInsertCallbackId(self.imp.on_insert(Box::new(callback)))
            }
            fn remove_on_insert(&self, callback: UserInsertCallbackId) {
                self.imp.remove_on_insert(callback.0)
            }
            type DeleteCallbackId = UserDeleteCallbackId;
            fn on_delete(
                &self,
                callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
            ) -> UserDeleteCallbackId {
                UserDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
            }
            fn remove_on_delete(&self, callback: UserDeleteCallbackId) {
                self.imp.remove_on_delete(callback.0)
            }
        }
        #[doc(hidden)]
        pub(super) fn register_table(
            client_cache: &mut __sdk::ClientCache<super::RemoteModule>,
        ) {
            let _table = client_cache.get_or_make_table::<User>("user");
            _table
                .add_unique_constraint::<
                    __sdk::Identity,
                >("identity", |row| &row.identity);
        }
        pub struct UserUpdateCallbackId(__sdk::CallbackId);
        impl<'ctx> __sdk::TableWithPrimaryKey for UserTableHandle<'ctx> {
            type UpdateCallbackId = UserUpdateCallbackId;
            fn on_update(
                &self,
                callback: impl FnMut(
                    &Self::EventContext,
                    &Self::Row,
                    &Self::Row,
                ) + Send + 'static,
            ) -> UserUpdateCallbackId {
                UserUpdateCallbackId(self.imp.on_update(Box::new(callback)))
            }
            fn remove_on_update(&self, callback: UserUpdateCallbackId) {
                self.imp.remove_on_update(callback.0)
            }
        }
        #[doc(hidden)]
        pub(super) fn parse_table_update(
            raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
        ) -> __sdk::Result<__sdk::TableUpdate<User>> {
            __sdk::TableUpdate::parse_table_update(raw_updates)
                .map_err(|e| {
                    __sdk::InternalError::failed_parse(
                            "TableUpdate<User>",
                            "TableUpdate",
                        )
                        .with_cause(e)
                        .into()
                })
        }
        /// Access to the `identity` unique index on the table `user`,
        /// which allows point queries on the field of the same name
        /// via the [`UserIdentityUnique::find`] method.
        ///
        /// Users are encouraged not to explicitly reference this type,
        /// but to directly chain method calls,
        /// like `ctx.db.user().identity().find(...)`.
        pub struct UserIdentityUnique<'ctx> {
            imp: __sdk::UniqueConstraintHandle<User, __sdk::Identity>,
            phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
        }
        impl<'ctx> UserTableHandle<'ctx> {
            /// Get a handle on the `identity` unique index on the table `user`.
            pub fn identity(&self) -> UserIdentityUnique<'ctx> {
                UserIdentityUnique {
                    imp: self.imp.get_unique_constraint::<__sdk::Identity>("identity"),
                    phantom: std::marker::PhantomData,
                }
            }
        }
        impl<'ctx> UserIdentityUnique<'ctx> {
            /// Find the subscribed row whose `identity` column value is equal to `col_val`,
            /// if such a row is present in the client cache.
            pub fn find(&self, col_val: &__sdk::Identity) -> Option<User> {
                self.imp.find(col_val)
            }
        }
    }
    pub mod user_type {
        #![allow(unused, clippy::all)]
        use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
        #[sats(crate = __lib)]
        pub struct User {
            pub identity: __sdk::Identity,
            pub name: Option<String>,
            pub online: bool,
        }
        impl __lib::ser::Serialize for User {
            fn serialize<S: __lib::ser::Serializer>(
                &self,
                __serializer: S,
            ) -> Result<S::Ok, S::Error> {
                let mut __prod = __serializer.serialize_named_product(3usize)?;
                __lib::ser::SerializeNamedProduct::serialize_element::<
                    __sdk::Identity,
                >(&mut __prod, Some("identity"), &self.identity)?;
                __lib::ser::SerializeNamedProduct::serialize_element::<
                    Option<String>,
                >(&mut __prod, Some("name"), &self.name)?;
                __lib::ser::SerializeNamedProduct::serialize_element::<
                    bool,
                >(&mut __prod, Some("online"), &self.online)?;
                __lib::ser::SerializeNamedProduct::end(__prod)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::all)]
        const _: () = {
            impl<'de> __lib::de::Deserialize<'de> for User {
                fn deserialize<D: __lib::de::Deserializer<'de>>(
                    deserializer: D,
                ) -> Result<Self, D::Error> {
                    deserializer
                        .deserialize_product(__ProductVisitor {
                            _marker: std::marker::PhantomData::<fn() -> User>,
                        })
                }
            }
            struct __ProductVisitor {
                _marker: std::marker::PhantomData<fn() -> User>,
            }
            impl<'de> __lib::de::ProductVisitor<'de> for __ProductVisitor {
                type Output = User;
                fn product_name(&self) -> Option<&str> {
                    Some("User")
                }
                fn product_len(&self) -> usize {
                    3usize
                }
                fn visit_seq_product<A: __lib::de::SeqProductAccess<'de>>(
                    self,
                    mut tup: A,
                ) -> Result<Self::Output, A::Error> {
                    Ok(User {
                        identity: tup
                            .next_element::<__sdk::Identity>()?
                            .ok_or_else(|| __lib::de::Error::invalid_product_length(
                                0usize,
                                &self,
                            ))?,
                        name: tup
                            .next_element::<Option<String>>()?
                            .ok_or_else(|| __lib::de::Error::invalid_product_length(
                                1usize,
                                &self,
                            ))?,
                        online: tup
                            .next_element::<bool>()?
                            .ok_or_else(|| __lib::de::Error::invalid_product_length(
                                2usize,
                                &self,
                            ))?,
                    })
                }
                fn visit_named_product<A: __lib::de::NamedProductAccess<'de>>(
                    self,
                    mut __prod: A,
                ) -> Result<Self::Output, A::Error> {
                    let mut identity = None;
                    let mut name = None;
                    let mut online = None;
                    while let Some(__field) = __lib::de::NamedProductAccess::get_field_ident(
                        &mut __prod,
                        Self {
                            _marker: std::marker::PhantomData,
                        },
                    )? {
                        match __field {
                            __ProductFieldIdent::identity => {
                                if identity.is_some() {
                                    return Err(
                                        __lib::de::Error::duplicate_field(
                                            0usize,
                                            Some("identity"),
                                            &self,
                                        ),
                                    );
                                }
                                identity = Some(
                                    __lib::de::NamedProductAccess::get_field_value::<
                                        __sdk::Identity,
                                    >(&mut __prod)?,
                                );
                            }
                            __ProductFieldIdent::name => {
                                if name.is_some() {
                                    return Err(
                                        __lib::de::Error::duplicate_field(
                                            1usize,
                                            Some("name"),
                                            &self,
                                        ),
                                    );
                                }
                                name = Some(
                                    __lib::de::NamedProductAccess::get_field_value::<
                                        Option<String>,
                                    >(&mut __prod)?,
                                );
                            }
                            __ProductFieldIdent::online => {
                                if online.is_some() {
                                    return Err(
                                        __lib::de::Error::duplicate_field(
                                            2usize,
                                            Some("online"),
                                            &self,
                                        ),
                                    );
                                }
                                online = Some(
                                    __lib::de::NamedProductAccess::get_field_value::<
                                        bool,
                                    >(&mut __prod)?,
                                );
                            }
                        }
                    }
                    Ok(User {
                        identity: identity
                            .ok_or_else(|| __lib::de::Error::missing_field(
                                0usize,
                                Some("identity"),
                                &self,
                            ))?,
                        name: name
                            .ok_or_else(|| __lib::de::Error::missing_field(
                                1usize,
                                Some("name"),
                                &self,
                            ))?,
                        online: online
                            .ok_or_else(|| __lib::de::Error::missing_field(
                                2usize,
                                Some("online"),
                                &self,
                            ))?,
                    })
                }
            }
            impl<'de> __lib::de::FieldNameVisitor<'de> for __ProductVisitor {
                type Output = __ProductFieldIdent;
                fn field_names(&self, names: &mut dyn __lib::de::ValidNames) {
                    names.extend::<&[&str]>(&["identity", "name", "online"])
                }
                fn visit<__E: __lib::de::Error>(
                    self,
                    name: &str,
                ) -> Result<Self::Output, __E> {
                    match name {
                        "identity" => Ok(__ProductFieldIdent::identity),
                        "name" => Ok(__ProductFieldIdent::name),
                        "online" => Ok(__ProductFieldIdent::online),
                        _ => Err(__lib::de::Error::unknown_field_name(name, &self)),
                    }
                }
            }
            #[allow(non_camel_case_types)]
            enum __ProductFieldIdent {
                identity,
                name,
                online,
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for User {
            #[inline]
            fn clone(&self) -> User {
                User {
                    identity: ::core::clone::Clone::clone(&self.identity),
                    name: ::core::clone::Clone::clone(&self.name),
                    online: ::core::clone::Clone::clone(&self.online),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for User {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for User {
            #[inline]
            fn eq(&self, other: &User) -> bool {
                self.identity == other.identity && self.name == other.name
                    && self.online == other.online
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for User {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "User",
                    "identity",
                    &self.identity,
                    "name",
                    &self.name,
                    "online",
                    &&self.online,
                )
            }
        }
        impl __sdk::InModule for User {
            type Module = super::RemoteModule;
        }
    }
    pub use client_connected_reducer::{
        client_connected, set_flags_for_client_connected, ClientConnectedCallbackId,
    };
    pub use create_lobby_reducer::{
        create_lobby, set_flags_for_create_lobby, CreateLobbyCallbackId,
    };
    pub use identity_disconnected_reducer::{
        identity_disconnected, set_flags_for_identity_disconnected,
        IdentityDisconnectedCallbackId,
    };
    pub use lobby_table::*;
    pub use lobby_type::Lobby;
    pub use set_name_reducer::{set_flags_for_set_name, set_name, SetNameCallbackId};
    pub use user_table::*;
    pub use user_type::User;
    /// One of the reducers defined by this module.
    ///
    /// Contained within a [`__sdk::ReducerEvent`] in [`EventContext`]s for reducer events
    /// to indicate which reducer caused the event.
    pub enum Reducer {
        ClientConnected,
        CreateLobby,
        IdentityDisconnected,
        SetName { name: String },
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Reducer {
        #[inline]
        fn clone(&self) -> Reducer {
            match self {
                Reducer::ClientConnected => Reducer::ClientConnected,
                Reducer::CreateLobby => Reducer::CreateLobby,
                Reducer::IdentityDisconnected => Reducer::IdentityDisconnected,
                Reducer::SetName { name: __self_0 } => {
                    Reducer::SetName {
                        name: ::core::clone::Clone::clone(__self_0),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Reducer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Reducer {
        #[inline]
        fn eq(&self, other: &Reducer) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        Reducer::SetName { name: __self_0 },
                        Reducer::SetName { name: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Reducer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Reducer::ClientConnected => {
                    ::core::fmt::Formatter::write_str(f, "ClientConnected")
                }
                Reducer::CreateLobby => {
                    ::core::fmt::Formatter::write_str(f, "CreateLobby")
                }
                Reducer::IdentityDisconnected => {
                    ::core::fmt::Formatter::write_str(f, "IdentityDisconnected")
                }
                Reducer::SetName { name: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SetName",
                        "name",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl __sdk::InModule for Reducer {
        type Module = RemoteModule;
    }
    impl __sdk::Reducer for Reducer {
        fn reducer_name(&self) -> &'static str {
            match self {
                Reducer::ClientConnected => "client_connected",
                Reducer::CreateLobby => "create_lobby",
                Reducer::IdentityDisconnected => "identity_disconnected",
                Reducer::SetName { .. } => "set_name",
            }
        }
    }
    impl TryFrom<__ws::ReducerCallInfo<__ws::BsatnFormat>> for Reducer {
        type Error = __sdk::Error;
        fn try_from(
            value: __ws::ReducerCallInfo<__ws::BsatnFormat>,
        ) -> __sdk::Result<Self> {
            match &value.reducer_name[..] {
                "client_connected" => {
                    Ok(
                        __sdk::parse_reducer_args::<
                            client_connected_reducer::ClientConnectedArgs,
                        >("client_connected", &value.args)?
                            .into(),
                    )
                }
                "create_lobby" => {
                    Ok(
                        __sdk::parse_reducer_args::<
                            create_lobby_reducer::CreateLobbyArgs,
                        >("create_lobby", &value.args)?
                            .into(),
                    )
                }
                "identity_disconnected" => {
                    Ok(
                        __sdk::parse_reducer_args::<
                            identity_disconnected_reducer::IdentityDisconnectedArgs,
                        >("identity_disconnected", &value.args)?
                            .into(),
                    )
                }
                "set_name" => {
                    Ok(
                        __sdk::parse_reducer_args::<
                            set_name_reducer::SetNameArgs,
                        >("set_name", &value.args)?
                            .into(),
                    )
                }
                unknown => {
                    Err(
                        __sdk::InternalError::unknown_name(
                                "reducer",
                                unknown,
                                "ReducerCallInfo",
                            )
                            .into(),
                    )
                }
            }
        }
    }
    #[allow(non_snake_case)]
    #[doc(hidden)]
    pub struct DbUpdate {
        lobby: __sdk::TableUpdate<Lobby>,
        user: __sdk::TableUpdate<User>,
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    impl ::core::default::Default for DbUpdate {
        #[inline]
        fn default() -> DbUpdate {
            DbUpdate {
                lobby: ::core::default::Default::default(),
                user: ::core::default::Default::default(),
            }
        }
    }
    impl TryFrom<__ws::DatabaseUpdate<__ws::BsatnFormat>> for DbUpdate {
        type Error = __sdk::Error;
        fn try_from(
            raw: __ws::DatabaseUpdate<__ws::BsatnFormat>,
        ) -> Result<Self, Self::Error> {
            let mut db_update = DbUpdate::default();
            for table_update in raw.tables {
                match &table_update.table_name[..] {
                    "lobby" => {
                        db_update
                            .lobby
                            .append(lobby_table::parse_table_update(table_update)?)
                    }
                    "user" => {
                        db_update
                            .user
                            .append(user_table::parse_table_update(table_update)?)
                    }
                    unknown => {
                        return Err(
                            __sdk::InternalError::unknown_name(
                                    "table",
                                    unknown,
                                    "DatabaseUpdate",
                                )
                                .into(),
                        );
                    }
                }
            }
            Ok(db_update)
        }
    }
    impl __sdk::InModule for DbUpdate {
        type Module = RemoteModule;
    }
    impl __sdk::DbUpdate for DbUpdate {
        fn apply_to_client_cache(
            &self,
            cache: &mut __sdk::ClientCache<RemoteModule>,
        ) -> AppliedDiff<'_> {
            let mut diff = AppliedDiff::default();
            diff.lobby = cache
                .apply_diff_to_table::<Lobby>("lobby", &self.lobby)
                .with_updates_by_pk(|row| &row.id);
            diff.user = cache
                .apply_diff_to_table::<User>("user", &self.user)
                .with_updates_by_pk(|row| &row.identity);
            diff
        }
    }
    #[allow(non_snake_case)]
    #[doc(hidden)]
    pub struct AppliedDiff<'r> {
        lobby: __sdk::TableAppliedDiff<'r, Lobby>,
        user: __sdk::TableAppliedDiff<'r, User>,
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    impl<'r> ::core::default::Default for AppliedDiff<'r> {
        #[inline]
        fn default() -> AppliedDiff<'r> {
            AppliedDiff {
                lobby: ::core::default::Default::default(),
                user: ::core::default::Default::default(),
            }
        }
    }
    impl __sdk::InModule for AppliedDiff<'_> {
        type Module = RemoteModule;
    }
    impl<'r> __sdk::AppliedDiff<'r> for AppliedDiff<'r> {
        fn invoke_row_callbacks(
            &self,
            event: &EventContext,
            callbacks: &mut __sdk::DbCallbacks<RemoteModule>,
        ) {
            callbacks.invoke_table_row_callbacks::<Lobby>("lobby", &self.lobby, event);
            callbacks.invoke_table_row_callbacks::<User>("user", &self.user, event);
        }
    }
    #[doc(hidden)]
    pub struct RemoteModule;
    impl __sdk::InModule for RemoteModule {
        type Module = Self;
    }
    /// The `reducers` field of [`EventContext`] and [`DbConnection`],
    /// with methods provided by extension traits for each reducer defined by the module.
    pub struct RemoteReducers {
        imp: __sdk::DbContextImpl<RemoteModule>,
    }
    impl __sdk::InModule for RemoteReducers {
        type Module = RemoteModule;
    }
    #[doc(hidden)]
    /// The `set_reducer_flags` field of [`DbConnection`],
    /// with methods provided by extension traits for each reducer defined by the module.
    /// Each method sets the flags for the reducer with the same name.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub struct SetReducerFlags {
        imp: __sdk::DbContextImpl<RemoteModule>,
    }
    impl __sdk::InModule for SetReducerFlags {
        type Module = RemoteModule;
    }
    /// The `db` field of [`EventContext`] and [`DbConnection`],
    /// with methods provided by extension traits for each table defined by the module.
    pub struct RemoteTables {
        imp: __sdk::DbContextImpl<RemoteModule>,
    }
    impl __sdk::InModule for RemoteTables {
        type Module = RemoteModule;
    }
    /// A connection to a remote module, including a materialized view of a subset of the database.
    ///
    /// Connect to a remote module by calling [`DbConnection::builder`]
    /// and using the [`__sdk::DbConnectionBuilder`] builder-pattern constructor.
    ///
    /// You must explicitly advance the connection by calling any one of:
    ///
    /// - [`DbConnection::frame_tick`].
    /// - [`DbConnection::run_threaded`].
    /// - [`DbConnection::run_async`].
    /// - [`DbConnection::advance_one_message`].
    /// - [`DbConnection::advance_one_message_blocking`].
    /// - [`DbConnection::advance_one_message_async`].
    ///
    /// Which of these methods you should call depends on the specific needs of your application,
    /// but you must call one of them, or else the connection will never progress.
    pub struct DbConnection {
        /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
        pub db: RemoteTables,
        /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
        pub reducers: RemoteReducers,
        #[doc(hidden)]
        /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
        /// via extension traits implemented for [`SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub set_reducer_flags: SetReducerFlags,
        imp: __sdk::DbContextImpl<RemoteModule>,
    }
    impl __sdk::InModule for DbConnection {
        type Module = RemoteModule;
    }
    impl __sdk::DbContext for DbConnection {
        type DbView = RemoteTables;
        type Reducers = RemoteReducers;
        type SetReducerFlags = SetReducerFlags;
        fn db(&self) -> &Self::DbView {
            &self.db
        }
        fn reducers(&self) -> &Self::Reducers {
            &self.reducers
        }
        fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
            &self.set_reducer_flags
        }
        fn is_active(&self) -> bool {
            self.imp.is_active()
        }
        fn disconnect(&self) -> __sdk::Result<()> {
            self.imp.disconnect()
        }
        type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;
        fn subscription_builder(&self) -> Self::SubscriptionBuilder {
            __sdk::SubscriptionBuilder::new(&self.imp)
        }
        fn try_identity(&self) -> Option<__sdk::Identity> {
            self.imp.try_identity()
        }
        fn connection_id(&self) -> __sdk::ConnectionId {
            self.imp.connection_id()
        }
    }
    impl DbConnection {
        /// Builder-pattern constructor for a connection to a remote module.
        ///
        /// See [`__sdk::DbConnectionBuilder`] for required and optional configuration for the new connection.
        pub fn builder() -> __sdk::DbConnectionBuilder<RemoteModule> {
            __sdk::DbConnectionBuilder::new()
        }
        /// If any WebSocket messages are waiting, process one of them.
        ///
        /// Returns `true` if a message was processed, or `false` if the queue is empty.
        /// Callers should invoke this message in a loop until it returns `false`
        /// or for as much time is available to process messages.
        ///
        /// Returns an error if the connection is disconnected.
        /// If the disconnection in question was normal,
        ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
        /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
        ///
        /// This is a low-level primitive exposed for power users who need significant control over scheduling.
        /// Most applications should call [`Self::frame_tick`] each frame
        /// to fully exhaust the queue whenever time is available.
        pub fn advance_one_message(&self) -> __sdk::Result<bool> {
            self.imp.advance_one_message()
        }
        /// Process one WebSocket message, potentially blocking the current thread until one is received.
        ///
        /// Returns an error if the connection is disconnected.
        /// If the disconnection in question was normal,
        ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
        /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
        ///
        /// This is a low-level primitive exposed for power users who need significant control over scheduling.
        /// Most applications should call [`Self::run_threaded`] to spawn a thread
        /// which advances the connection automatically.
        pub fn advance_one_message_blocking(&self) -> __sdk::Result<()> {
            self.imp.advance_one_message_blocking()
        }
        /// Process one WebSocket message, `await`ing until one is received.
        ///
        /// Returns an error if the connection is disconnected.
        /// If the disconnection in question was normal,
        ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
        /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
        ///
        /// This is a low-level primitive exposed for power users who need significant control over scheduling.
        /// Most applications should call [`Self::run_async`] to run an `async` loop
        /// which advances the connection when polled.
        pub async fn advance_one_message_async(&self) -> __sdk::Result<()> {
            self.imp.advance_one_message_async().await
        }
        /// Process all WebSocket messages waiting in the queue,
        /// then return without `await`ing or blocking the current thread.
        pub fn frame_tick(&self) -> __sdk::Result<()> {
            self.imp.frame_tick()
        }
        /// Spawn a thread which processes WebSocket messages as they are received.
        pub fn run_threaded(&self) -> std::thread::JoinHandle<()> {
            self.imp.run_threaded()
        }
        /// Run an `async` loop which processes WebSocket messages when polled.
        pub async fn run_async(&self) -> __sdk::Result<()> {
            self.imp.run_async().await
        }
    }
    impl __sdk::DbConnection for DbConnection {
        fn new(imp: __sdk::DbContextImpl<RemoteModule>) -> Self {
            Self {
                db: RemoteTables { imp: imp.clone() },
                reducers: RemoteReducers { imp: imp.clone() },
                set_reducer_flags: SetReducerFlags {
                    imp: imp.clone(),
                },
                imp,
            }
        }
    }
    /// A handle on a subscribed query.
    pub struct SubscriptionHandle {
        imp: __sdk::SubscriptionHandleImpl<RemoteModule>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SubscriptionHandle {
        #[inline]
        fn clone(&self) -> SubscriptionHandle {
            SubscriptionHandle {
                imp: ::core::clone::Clone::clone(&self.imp),
            }
        }
    }
    impl __sdk::InModule for SubscriptionHandle {
        type Module = RemoteModule;
    }
    impl __sdk::SubscriptionHandle for SubscriptionHandle {
        fn new(imp: __sdk::SubscriptionHandleImpl<RemoteModule>) -> Self {
            Self { imp }
        }
        /// Returns true if this subscription has been terminated due to an unsubscribe call or an error.
        fn is_ended(&self) -> bool {
            self.imp.is_ended()
        }
        /// Returns true if this subscription has been applied and has not yet been unsubscribed.
        fn is_active(&self) -> bool {
            self.imp.is_active()
        }
        /// Unsubscribe from the query controlled by this `SubscriptionHandle`,
        /// then run `on_end` when its rows are removed from the client cache.
        fn unsubscribe_then(
            self,
            on_end: __sdk::OnEndedCallback<RemoteModule>,
        ) -> __sdk::Result<()> {
            self.imp.unsubscribe_then(Some(on_end))
        }
        fn unsubscribe(self) -> __sdk::Result<()> {
            self.imp.unsubscribe_then(None)
        }
    }
    /// Alias trait for a [`__sdk::DbContext`] connected to this module,
    /// with that trait's associated types bounded to this module's concrete types.
    ///
    /// Users can use this trait as a boundary on definitions which should accept
    /// either a [`DbConnection`] or an [`EventContext`] and operate on either.
    pub trait RemoteDbContext: __sdk::DbContext<
            DbView = RemoteTables,
            Reducers = RemoteReducers,
            SetReducerFlags = SetReducerFlags,
            SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>,
        > {}
    impl<
        Ctx: __sdk::DbContext<
                DbView = RemoteTables,
                Reducers = RemoteReducers,
                SetReducerFlags = SetReducerFlags,
                SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>,
            >,
    > RemoteDbContext for Ctx {}
    /// An [`__sdk::DbContext`] augmented with a [`__sdk::Event`],
    /// passed to [`__sdk::Table::on_insert`], [`__sdk::Table::on_delete`] and [`__sdk::TableWithPrimaryKey::on_update`] callbacks.
    pub struct EventContext {
        /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
        pub db: RemoteTables,
        /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
        pub reducers: RemoteReducers,
        /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
        /// via extension traits implemented for [`SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub set_reducer_flags: SetReducerFlags,
        /// The event which caused these callbacks to run.
        pub event: __sdk::Event<Reducer>,
        imp: __sdk::DbContextImpl<RemoteModule>,
    }
    impl __sdk::AbstractEventContext for EventContext {
        type Event = __sdk::Event<Reducer>;
        fn event(&self) -> &Self::Event {
            &self.event
        }
        fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
            Self {
                db: RemoteTables { imp: imp.clone() },
                reducers: RemoteReducers { imp: imp.clone() },
                set_reducer_flags: SetReducerFlags {
                    imp: imp.clone(),
                },
                event,
                imp,
            }
        }
    }
    impl __sdk::InModule for EventContext {
        type Module = RemoteModule;
    }
    impl __sdk::DbContext for EventContext {
        type DbView = RemoteTables;
        type Reducers = RemoteReducers;
        type SetReducerFlags = SetReducerFlags;
        fn db(&self) -> &Self::DbView {
            &self.db
        }
        fn reducers(&self) -> &Self::Reducers {
            &self.reducers
        }
        fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
            &self.set_reducer_flags
        }
        fn is_active(&self) -> bool {
            self.imp.is_active()
        }
        fn disconnect(&self) -> __sdk::Result<()> {
            self.imp.disconnect()
        }
        type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;
        fn subscription_builder(&self) -> Self::SubscriptionBuilder {
            __sdk::SubscriptionBuilder::new(&self.imp)
        }
        fn try_identity(&self) -> Option<__sdk::Identity> {
            self.imp.try_identity()
        }
        fn connection_id(&self) -> __sdk::ConnectionId {
            self.imp.connection_id()
        }
    }
    impl __sdk::EventContext for EventContext {}
    /// An [`__sdk::DbContext`] augmented with a [`__sdk::ReducerEvent`],
    /// passed to on-reducer callbacks.
    pub struct ReducerEventContext {
        /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
        pub db: RemoteTables,
        /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
        pub reducers: RemoteReducers,
        /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
        /// via extension traits implemented for [`SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub set_reducer_flags: SetReducerFlags,
        /// The event which caused these callbacks to run.
        pub event: __sdk::ReducerEvent<Reducer>,
        imp: __sdk::DbContextImpl<RemoteModule>,
    }
    impl __sdk::AbstractEventContext for ReducerEventContext {
        type Event = __sdk::ReducerEvent<Reducer>;
        fn event(&self) -> &Self::Event {
            &self.event
        }
        fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
            Self {
                db: RemoteTables { imp: imp.clone() },
                reducers: RemoteReducers { imp: imp.clone() },
                set_reducer_flags: SetReducerFlags {
                    imp: imp.clone(),
                },
                event,
                imp,
            }
        }
    }
    impl __sdk::InModule for ReducerEventContext {
        type Module = RemoteModule;
    }
    impl __sdk::DbContext for ReducerEventContext {
        type DbView = RemoteTables;
        type Reducers = RemoteReducers;
        type SetReducerFlags = SetReducerFlags;
        fn db(&self) -> &Self::DbView {
            &self.db
        }
        fn reducers(&self) -> &Self::Reducers {
            &self.reducers
        }
        fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
            &self.set_reducer_flags
        }
        fn is_active(&self) -> bool {
            self.imp.is_active()
        }
        fn disconnect(&self) -> __sdk::Result<()> {
            self.imp.disconnect()
        }
        type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;
        fn subscription_builder(&self) -> Self::SubscriptionBuilder {
            __sdk::SubscriptionBuilder::new(&self.imp)
        }
        fn try_identity(&self) -> Option<__sdk::Identity> {
            self.imp.try_identity()
        }
        fn connection_id(&self) -> __sdk::ConnectionId {
            self.imp.connection_id()
        }
    }
    impl __sdk::ReducerEventContext for ReducerEventContext {}
    /// An [`__sdk::DbContext`] passed to [`__sdk::SubscriptionBuilder::on_applied`] and [`SubscriptionHandle::unsubscribe_then`] callbacks.
    pub struct SubscriptionEventContext {
        /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
        pub db: RemoteTables,
        /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
        pub reducers: RemoteReducers,
        /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
        /// via extension traits implemented for [`SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub set_reducer_flags: SetReducerFlags,
        imp: __sdk::DbContextImpl<RemoteModule>,
    }
    impl __sdk::AbstractEventContext for SubscriptionEventContext {
        type Event = ();
        fn event(&self) -> &Self::Event {
            &()
        }
        fn new(imp: __sdk::DbContextImpl<RemoteModule>, _event: Self::Event) -> Self {
            Self {
                db: RemoteTables { imp: imp.clone() },
                reducers: RemoteReducers { imp: imp.clone() },
                set_reducer_flags: SetReducerFlags {
                    imp: imp.clone(),
                },
                imp,
            }
        }
    }
    impl __sdk::InModule for SubscriptionEventContext {
        type Module = RemoteModule;
    }
    impl __sdk::DbContext for SubscriptionEventContext {
        type DbView = RemoteTables;
        type Reducers = RemoteReducers;
        type SetReducerFlags = SetReducerFlags;
        fn db(&self) -> &Self::DbView {
            &self.db
        }
        fn reducers(&self) -> &Self::Reducers {
            &self.reducers
        }
        fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
            &self.set_reducer_flags
        }
        fn is_active(&self) -> bool {
            self.imp.is_active()
        }
        fn disconnect(&self) -> __sdk::Result<()> {
            self.imp.disconnect()
        }
        type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;
        fn subscription_builder(&self) -> Self::SubscriptionBuilder {
            __sdk::SubscriptionBuilder::new(&self.imp)
        }
        fn try_identity(&self) -> Option<__sdk::Identity> {
            self.imp.try_identity()
        }
        fn connection_id(&self) -> __sdk::ConnectionId {
            self.imp.connection_id()
        }
    }
    impl __sdk::SubscriptionEventContext for SubscriptionEventContext {}
    /// An [`__sdk::DbContext`] augmented with a [`__sdk::Error`],
    /// passed to [`__sdk::DbConnectionBuilder::on_disconnect`], [`__sdk::DbConnectionBuilder::on_connect_error`] and [`__sdk::SubscriptionBuilder::on_error`] callbacks.
    pub struct ErrorContext {
        /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
        pub db: RemoteTables,
        /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
        pub reducers: RemoteReducers,
        /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
        /// via extension traits implemented for [`SetReducerFlags`].
        ///
        /// This type is currently unstable and may be removed without a major version bump.
        pub set_reducer_flags: SetReducerFlags,
        /// The event which caused these callbacks to run.
        pub event: Option<__sdk::Error>,
        imp: __sdk::DbContextImpl<RemoteModule>,
    }
    impl __sdk::AbstractEventContext for ErrorContext {
        type Event = Option<__sdk::Error>;
        fn event(&self) -> &Self::Event {
            &self.event
        }
        fn new(imp: __sdk::DbContextImpl<RemoteModule>, event: Self::Event) -> Self {
            Self {
                db: RemoteTables { imp: imp.clone() },
                reducers: RemoteReducers { imp: imp.clone() },
                set_reducer_flags: SetReducerFlags {
                    imp: imp.clone(),
                },
                event,
                imp,
            }
        }
    }
    impl __sdk::InModule for ErrorContext {
        type Module = RemoteModule;
    }
    impl __sdk::DbContext for ErrorContext {
        type DbView = RemoteTables;
        type Reducers = RemoteReducers;
        type SetReducerFlags = SetReducerFlags;
        fn db(&self) -> &Self::DbView {
            &self.db
        }
        fn reducers(&self) -> &Self::Reducers {
            &self.reducers
        }
        fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
            &self.set_reducer_flags
        }
        fn is_active(&self) -> bool {
            self.imp.is_active()
        }
        fn disconnect(&self) -> __sdk::Result<()> {
            self.imp.disconnect()
        }
        type SubscriptionBuilder = __sdk::SubscriptionBuilder<RemoteModule>;
        fn subscription_builder(&self) -> Self::SubscriptionBuilder {
            __sdk::SubscriptionBuilder::new(&self.imp)
        }
        fn try_identity(&self) -> Option<__sdk::Identity> {
            self.imp.try_identity()
        }
        fn connection_id(&self) -> __sdk::ConnectionId {
            self.imp.connection_id()
        }
    }
    impl __sdk::ErrorContext for ErrorContext {}
    impl __sdk::SpacetimeModule for RemoteModule {
        type DbConnection = DbConnection;
        type EventContext = EventContext;
        type ReducerEventContext = ReducerEventContext;
        type SubscriptionEventContext = SubscriptionEventContext;
        type ErrorContext = ErrorContext;
        type Reducer = Reducer;
        type DbView = RemoteTables;
        type Reducers = RemoteReducers;
        type SetReducerFlags = SetReducerFlags;
        type DbUpdate = DbUpdate;
        type AppliedDiff<'r> = AppliedDiff<'r>;
        type SubscriptionHandle = SubscriptionHandle;
        fn register_tables(client_cache: &mut __sdk::ClientCache<Self>) {
            lobby_table::register_table(client_cache);
            user_table::register_table(client_cache);
        }
    }
}
pub struct RegisterPlayerEvent {
    pub event: ReducerEvent<Reducer>,
    pub id: u64,
}
#[automatically_derived]
impl ::core::clone::Clone for RegisterPlayerEvent {
    #[inline]
    fn clone(&self) -> RegisterPlayerEvent {
        RegisterPlayerEvent {
            event: ::core::clone::Clone::clone(&self.event),
            id: ::core::clone::Clone::clone(&self.id),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for RegisterPlayerEvent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "RegisterPlayerEvent",
            "event",
            &self.event,
            "id",
            &&self.id,
        )
    }
}
pub struct GsRegisterEvent {
    pub event: ReducerEvent<Reducer>,
    pub ip: String,
    pub port: u16,
}
#[automatically_derived]
impl ::core::clone::Clone for GsRegisterEvent {
    #[inline]
    fn clone(&self) -> GsRegisterEvent {
        GsRegisterEvent {
            event: ::core::clone::Clone::clone(&self.event),
            ip: ::core::clone::Clone::clone(&self.ip),
            port: ::core::clone::Clone::clone(&self.port),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for GsRegisterEvent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "GsRegisterEvent",
            "event",
            &self.event,
            "ip",
            &self.ip,
            "port",
            &&self.port,
        )
    }
}
pub fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(
            StdbPlugin::default()
                .with_build_fn(DbConnectionBuilder::build)
                .with_run_fn(DbConnection::run_threaded)
                .with_events(|
                    plugin: &StdbPlugin<DbConnection, RemoteModule>,
                    app,
                    db,
                    reducers|
                {
                    plugin.on_insert(app, db.user());
                    plugin.on_delete(app, db.user());
                    plugin.on_update(app, db.user());
                    plugin.on_insert_update(app, db.user());
                    plugin.on_insert(app, db.lobby());
                    plugin.on_delete(app, db.lobby());
                    plugin.on_update(app, db.lobby());
                    plugin.on_insert_update(app, db.lobby());
                    let send_createlobbyevent = plugin
                        .reducer_event::<CreateLobbyEvent>(app);
                    reducers
                        .on_create_lobby(move |ctx| {
                            send_createlobbyevent
                                .send(
                                    ReducerResultEvent::new(CreateLobbyEvent {
                                        event: ctx.event.clone(),
                                    }),
                                )
                                .unwrap();
                        });
                    let send_setnameevent = plugin.reducer_event::<SetNameEvent>(app);
                    reducers
                        .on_set_name(move |ctx, name| {
                            send_setnameevent
                                .send(
                                    ReducerResultEvent::new(SetNameEvent {
                                        event: ctx.event.clone(),
                                        name: name.clone(),
                                    }),
                                )
                                .unwrap();
                        });
                }),
        )
        .add_systems(Update, on_connected)
        .run();
}
fn on_connected(
    mut events: EventReader<StdbConnectedEvent>,
    stdb: Res<StdbConnection<DbConnection>>,
) {
    for _ in events.read() {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event example_app\\src\\main.rs:64",
                        "example_app",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some(
                            "example_app\\src\\main.rs",
                        ),
                        ::tracing_core::__macro_support::Option::Some(64u32),
                        ::tracing_core::__macro_support::Option::Some("example_app"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Connected to SpacetimeDB") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        stdb.subscribe()
            .on_applied(|_| {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event example_app\\src\\main.rs:67",
                            "example_app",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "example_app\\src\\main.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(67u32),
                            ::tracing_core::__macro_support::Option::Some("example_app"),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::INFO
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("Subscription to lobby applied") as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            })
            .on_error(|_, err| {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event example_app\\src\\main.rs:68",
                            "example_app",
                            ::tracing::Level::ERROR,
                            ::tracing_core::__macro_support::Option::Some(
                                "example_app\\src\\main.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(68u32),
                            ::tracing_core::__macro_support::Option::Some("example_app"),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::ERROR
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::ERROR
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("Subscription to lobby failed for: {0}", err)
                                                as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            })
            .subscribe("SELECT * FROM lobby");
        stdb.subscribe()
            .on_applied(|_| {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event example_app\\src\\main.rs:72",
                            "example_app",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "example_app\\src\\main.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(72u32),
                            ::tracing_core::__macro_support::Option::Some("example_app"),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::INFO
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("Subscription to user applied") as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            })
            .on_error(|_, err| {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event example_app\\src\\main.rs:73",
                            "example_app",
                            ::tracing::Level::ERROR,
                            ::tracing_core::__macro_support::Option::Some(
                                "example_app\\src\\main.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(73u32),
                            ::tracing_core::__macro_support::Option::Some("example_app"),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::ERROR
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::ERROR
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("Subscription to user failed for: {0}", err)
                                                as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            })
            .subscribe("SELECT * FROM user");
    }
}
pub struct CreateLobbyEvent {
    pub event: ReducerEvent<Reducer>,
}
#[automatically_derived]
impl ::core::clone::Clone for CreateLobbyEvent {
    #[inline]
    fn clone(&self) -> CreateLobbyEvent {
        CreateLobbyEvent {
            event: ::core::clone::Clone::clone(&self.event),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CreateLobbyEvent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "CreateLobbyEvent",
            "event",
            &&self.event,
        )
    }
}
pub struct SetNameEvent {
    pub event: ReducerEvent<Reducer>,
    pub name: String,
}
#[automatically_derived]
impl ::core::clone::Clone for SetNameEvent {
    #[inline]
    fn clone(&self) -> SetNameEvent {
        SetNameEvent {
            event: ::core::clone::Clone::clone(&self.event),
            name: ::core::clone::Clone::clone(&self.name),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for SetNameEvent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SetNameEvent",
            "event",
            &self.event,
            "name",
            &&self.name,
        )
    }
}
