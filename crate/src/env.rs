use wasm_bindgen::prelude::*;
use std::env::VarError;

#[wasm_bindgen(inline_js = "export function process_env_var(key) { const value = process.env[key]; return value == undefined ? '' : value; }")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn process_env_var(key:&str) -> Result<String, JsValue>;
}

pub fn env_var(key: &str) -> Result<String, VarError> {
    unsafe {
        process_env_var(key)
            .map_err(|_| {
                VarError::NotPresent
            })
            .and_then(|var| if var.is_empty() { Err(VarError::NotPresent) } else { Ok(var) })
    }
}
