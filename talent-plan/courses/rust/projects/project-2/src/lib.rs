#![deny(missing_docs)]
//! A simple key/value store.

// pub use error_example::{KvsError, Result};
pub use error::Result;
pub use kv::KvStore;
mod error;
mod error_example;
mod kv;
mod kv_example;
