#![allow(warnings)]

mod dom;
mod loader;
use wasm_bindgen::prelude::*;
use shipyard::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging and panic hook only during debug builds
cfg_if::cfg_if! {
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

cfg_if::cfg_if! {
    if #[cfg(feature = "ts_test")] {
    } else {
        #[wasm_bindgen]
        pub fn init_app() -> Result<js_sys::Promise, JsValue> {
            init_log();
            dom::init(loader::load())
        }
    }
}



