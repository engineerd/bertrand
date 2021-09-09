pub mod bertrand;
pub mod error;
pub mod fetch;
pub mod post;
pub mod state;
pub mod switch;

use bertrand::App;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    yew::start_app::<App>();

    Ok(())
}
