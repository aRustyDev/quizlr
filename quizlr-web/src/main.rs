use leptos::prelude::*;
use leptos_router::components::Router;

mod app;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    // Initialize console logging
    _ = console_log::init();
    
    leptos::mount::mount_to_body(|| {
        view! {
            <Router>
                <App/>
            </Router>
        }
    });
}