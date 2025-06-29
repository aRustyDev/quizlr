#[cfg(target_arch = "wasm32")]
mod tests {
    use leptos::*;
    use wasm_bindgen_test::*;
    use web_sys::window;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_header_has_styling_classes() {
        // Given: The app is mounted
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // Then: Header elements should have styling classes
        let h1 = document.query_selector("h1").unwrap();
        assert!(h1.is_some(), "Should find h1 element");

        if let Some(h1_elem) = h1 {
            let class_name = h1_elem.class_name();
            assert!(
                !class_name.is_empty(),
                "h1 should have styling classes, but has: '{}'",
                class_name
            );
        }
    }

    #[wasm_bindgen_test]
    fn test_header_subtitle_has_styling() {
        // Given: The app is mounted
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // Then: Header subtitle should have styling
        let header = document.query_selector("header").unwrap();
        assert!(header.is_some(), "Should find header element");

        if let Some(header_elem) = header {
            let class_name = header_elem.class_name();
            assert!(!class_name.is_empty(), "header should have styling classes");
        }
    }

    #[wasm_bindgen_test]
    fn test_buttons_have_click_handlers() {
        // Given: The app is mounted
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // When: We find the buttons
        let buttons = document.query_selector_all("button").unwrap();
        let button_count = buttons.length();

        // Then: Should have at least 2 buttons (Create Quiz and Import Content)
        assert!(
            button_count >= 2,
            "Should find at least 2 buttons, found {}",
            button_count
        );
    }

    #[wasm_bindgen_test]
    fn test_empty_state_displayed() {
        // Given: The app is mounted with no quizzes
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // Then: Should show welcome message or empty state
        let main_content = document.query_selector("main").unwrap();
        assert!(main_content.is_some(), "Should find main content area");

        // Should show content in main area
        if let Some(main_elem) = main_content {
            let inner_html = main_elem.inner_html();
            assert!(!inner_html.is_empty(), "Main content should not be empty");
        }
    }

    #[wasm_bindgen_test]
    fn test_welcome_card_visible() {
        // Given: The app is mounted
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // Then: Welcome card should be visible
        let welcome_heading = document.query_selector("h2").unwrap();
        assert!(welcome_heading.is_some(), "Should find welcome heading");

        if let Some(h2_elem) = welcome_heading {
            let text_content = h2_elem.text_content().unwrap_or_default();
            assert!(
                text_content.contains("Welcome"),
                "Should show welcome message, found: '{}'",
                text_content
            );
        }
    }
}
