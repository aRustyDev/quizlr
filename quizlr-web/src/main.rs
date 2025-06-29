use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

mod app;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    // Initialize console logging
    _ = console_log::init();

    // Mount to the #app div instead of body
    let app_element = document()
        .get_element_by_id("app")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    leptos::mount::mount_to(app_element, || view! { <App/> }).forget();
}
