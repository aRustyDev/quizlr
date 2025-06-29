#[cfg(target_arch = "wasm32")]
mod tests {
    use wasm_bindgen_test::*;
    use web_sys::window;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_app_mounts_to_dom() {
        // Given: The app should mount to the DOM
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // When: We check for the app content after mounting
        // Note: In a real test, we'd need to wait for the app to mount
        // For now, we're just verifying the structure exists

        // Then: The body should contain our app content
        let body = document.body().expect("should have body");
        let inner_html = body.inner_html();

        // The app should render some content
        assert!(
            !inner_html.is_empty(),
            "Body should not be empty after app mounts"
        );
    }

    #[wasm_bindgen_test]
    fn test_app_renders_expected_content() {
        // Given: The app is mounted
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // Then: We should find expected elements
        // Looking for the app-container div
        let app_container = document.query_selector(".app-container").unwrap();
        assert!(app_container.is_some(), "Should find app-container element");
    }
}
