//! Library to used to access the Github API with Rust
#![feature(custom_derive, proc_macro)]
#![cfg_attr(feature = "dev", plugin(clippy))]
// This allows for better enforc
// unstable_features add one day when custom derive is stableed style and project features
#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts, unsafe_code, unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;

// Only here for error handling with hyper
extern crate solicit;
extern crate url;

// This has to be here so the macros are available everywhere
#[macro_use]
mod macros;
#[macro_use]
mod error;

mod activity;
mod enterprise;
mod gists;
pub mod github;
mod git_data;
mod issues;
mod json;
mod migration;
mod miscellaneous;
mod organizations;
mod pull_requests;
mod reactions;
mod repositories;
mod requests;
mod search;
mod types;
mod users;

