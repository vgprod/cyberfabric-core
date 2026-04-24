//! User Info SDK
//!
//! This crate provides the public API for the `user_info` module:
//! - `UsersInfoClientV1` trait — feature `odata` (enabled by default)
//! - `UsersStreamingClientV1` / `CitiesStreamingClientV1` /
//!   `AddressesStreamingClientV1` streaming facades — feature `odata`
//!   (enabled by default)
//! - Model types for users, addresses and cities
//! - Error type (`UsersInfoError`)
//! - `OData` filter field definitions — feature `odata` (enabled by default)
//!
//! The `odata` feature is on by default; consumers that set
//! `default-features = false` need to re-enable it explicitly to access the
//! client and streaming facades.
//!
//! ## Usage
//!
//! Consumers obtain the client from `ClientHub` (requires the `odata` feature):
//! ```ignore
//! // Cargo.toml:
//! //   users-info-sdk = { workspace = true }            # default features incl. "odata"
//! //   # or, if opted out:
//! //   users-info-sdk = { workspace = true, default-features = false, features = ["odata"] }
//!
//! use user_info_sdk::UsersInfoClientV1;
//!
//! // Get the client from ClientHub
//! let client = hub.get::<dyn UsersInfoClientV1>()?;
//!
//! // Use the API
//! let user = client.get_user(&ctx, user_id).await?;
//! let users = client.list_users(&ctx, query).await?;
//! ```
//!
//! ## `OData` Support
//!
//! The `odata` feature (enabled by default) exposes filter field definitions:
//! ```ignore
//! use user_info_sdk::odata::{UserFilterField, CityFilterField};
//! ```

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

#[cfg(feature = "odata")]
pub mod client;
pub mod errors;
pub mod models;

// OData filter field definitions (feature-gated)
#[cfg(feature = "odata")]
pub mod odata;

// Re-export main types at crate root for convenience
#[cfg(feature = "odata")]
pub use client::{
    AddressesStreamingClientV1, CitiesStreamingClientV1, UsersInfoClientV1, UsersStreamingClientV1,
};
pub use errors::UsersInfoError;
pub use models::{
    Address, AddressPatch, City, CityPatch, NewAddress, NewCity, NewUser, UpdateAddressRequest,
    UpdateCityRequest, UpdateUserRequest, User, UserFull, UserPatch,
};
