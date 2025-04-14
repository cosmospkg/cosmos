pub mod config;
pub mod error;
pub mod constellation;
pub mod installer;
pub mod galaxy;
pub mod star;
pub mod universe;
pub mod resolver;

#[cfg(feature = "ffi")]
pub mod ffi;