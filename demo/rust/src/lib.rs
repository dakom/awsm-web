// see https://github.com/rust-lang/rust/issues/70070
#![cfg_attr(feature = "quiet", allow(warnings))]

mod menu;
mod router;
mod scenes;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging and panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
        fn init_log() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn init_log() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

// enable panic hook only during debug builds
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        fn init_panic() {
            console_error_panic_hook::set_once();
        }
    } else {
        fn init_panic() {}
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    init_panic();
    init_log();

    log::info!("logging enabled!");

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    router::start_router(window, document)
}
