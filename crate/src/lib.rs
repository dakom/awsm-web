#![allow(warnings)]
//these aren't worth putting behind features
pub mod errors;
pub mod prelude;
pub mod env;
pub(crate) mod unwrap; // exported through prelude

//each of these can be enabled/disabled as needed
#[cfg(feature = "dom")]
pub mod dom;
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "canvas")]
pub mod canvas;
#[cfg(feature = "data")]
pub mod data;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "loaders")]
pub mod loaders;
#[cfg(feature = "tick")]
pub mod tick;
#[cfg(feature = "webgl")]
pub mod webgl;
#[cfg(feature = "workers")]
pub mod workers;
#[cfg(feature = "window")]
pub mod window;
#[cfg(all(feature = "window", feature="workers"))]
pub mod global;
#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "file")]
pub mod file;