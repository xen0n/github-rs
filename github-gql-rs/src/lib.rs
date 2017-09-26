//! Library to used to access the Github API with Rust
#![feature(associated_type_defaults)]
#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate hyper_rustls;
extern crate futures;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;
extern crate git2;
extern crate chrono;

pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error)
                #[doc = "`std::io::Error` converted to an error-chain type"];
            Serde(::serde_json::Error)
                #[doc = "`serde_json::Error` converted to an error-chain type"];
            Hyper(::hyper::Error)
                #[doc = "`hyper::Error` converted to an error-chain type"];
        }
    }
}

pub mod client;
pub mod query;
pub mod mutation;

pub mod connections;
pub mod edges;
pub mod objects;
pub mod interfaces;
pub mod enums;
pub mod unions;
pub mod input_objects;
pub mod scalars;

pub use hyper::{Headers, StatusCode};

use errors::Result;
pub trait IntoGithubRequest {
    fn into_github_req(&self, token: &str) -> Result<hyper::Request>;
}
