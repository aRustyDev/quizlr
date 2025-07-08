use leptos::prelude::*;
use wasm_bindgen::JsCast;

mod app;
use app::App;

fn main() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize console logging
    _ = console_log::init();

    // Get the app div element and mount our app
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");

    let app_div = document
        .get_element_by_id("app")
        .expect("Could not find #app element")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("app element should be an HtmlElement");

    mount_to(app_div, || view! { <App/> }).forget();
}
