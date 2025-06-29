#[cfg(target_arch = "wasm32")]
mod tests {
    use leptos::*;
    use wasm_bindgen_test::*;
    use web_sys::{window, HtmlElement};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_main_content_is_visible() {
        // Given: The app is mounted
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // When: We check the main element
        let main = document.query_selector("main").unwrap();
        assert!(main.is_some(), "Should find main element");

        if let Some(main_elem) = main {
            let main_html = main_elem.dyn_into::<HtmlElement>().unwrap();
            let computed_style = window.get_computed_style(&main_html).unwrap().unwrap();

            // Then: Main should be visible
            let display = computed_style.get_property_value("display").unwrap();
            assert_ne!(display, "none", "Main element should not be display:none");

            let visibility = computed_style.get_property_value("visibility").unwrap();
            assert_ne!(
                visibility, "hidden",
                "Main element should not be visibility:hidden"
            );

            let opacity = computed_style.get_property_value("opacity").unwrap();
            assert_ne!(opacity, "0", "Main element should not have opacity:0");
        }
    }

    #[wasm_bindgen_test]
    fn test_content_has_proper_dimensions() {
        // Given: The app is mounted
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // When: We check content elements
        let cards = document.query_selector_all(".bg-white").unwrap();
        assert!(cards.length() > 0, "Should find content cards");

        // Then: Content should have dimensions
        for i in 0..cards.length() {
            if let Some(card) = cards.item(i) {
                if let Ok(card_elem) = card.dyn_into::<HtmlElement>() {
                    let rect = card_elem.get_bounding_client_rect();
                    assert!(rect.width() > 0.0, "Card should have width");
                    assert!(rect.height() > 0.0, "Card should have height");
                }
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_no_css_conflicts() {
        // Given: The app is mounted with Tailwind
        let window = window().expect("should have window");
        let document = window.document().expect("should have document");

        // When: We check for global reset issues
        let body = document.body().expect("should have body");
        let computed_style = window.get_computed_style(&body).unwrap().unwrap();

        // Then: Body should have proper styling (not reset to 0)
        let bg_color = computed_style
            .get_property_value("background-color")
            .unwrap();
        assert!(!bg_color.is_empty(), "Body should have background color");
    }
}
