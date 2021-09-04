pub mod bertrand;
pub mod error;
pub mod fetch;
pub mod state;

use bertrand::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    yew::start_app::<App>();

    Ok(())
}
