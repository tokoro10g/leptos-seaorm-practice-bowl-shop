pub mod app;
pub mod components {
    pub mod add_to_cart_popup;
    pub mod cart_item;
    pub mod cart_tooltip;
    pub mod menu_item;
    pub mod menu_list;
}
pub mod server;

#[cfg(feature = "ssr")]
pub mod db;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
