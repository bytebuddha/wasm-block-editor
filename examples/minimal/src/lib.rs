use wasm_bindgen::prelude::*;

mod app;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}
