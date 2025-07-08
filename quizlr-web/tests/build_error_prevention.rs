/// Tests to prevent common Leptos build errors
/// Run with: cargo test --target wasm32-unknown-unknown
#[cfg(test)]
mod tests {
    use leptos::prelude::*;

    #[test]
    fn test_if_else_view_types() {
        // This test ensures if/else branches return compatible types
        let (condition, _) = signal(true);

        // This should compile - all branches return the same structure
        let _view = move || {
            if condition.get() {
                view! { <div>"True"</div> }
            } else {
                view! { <div>"False"</div> }
            }
        };
    }

    #[test]
    fn test_show_component_usage() {
        // Prefer Show components over if/else for conditional rendering
        let (show, _) = signal(true);

        let _view = view! {
            <Show
                when=move || show.get()
                fallback=|| view! { <div>"Hidden"</div> }
            >
                <div>"Visible"</div>
            </Show>
        };
    }

    #[test]
    fn test_closure_captures() {
        // Test proper closure capture patterns
        let (count, set_count) = signal(0);

        // Good: Using signals properly
        let _increment = move |_: leptos::ev::MouseEvent| {
            set_count.set(count.get() + 1);
        };

        // Good: Cloning before capture
        let data = vec!["a", "b", "c"];
        let data_clone = data.clone();
        let _view = move || {
            data_clone
                .iter()
                .map(|&item| {
                    view! { <span>{item}</span> }
                })
                .collect::<Vec<_>>()
        };
    }

    #[test]
    fn test_consistent_view_structure() {
        // Ensure consistent HTML structure across conditional branches
        let (mode, _) = signal(0);

        let _view = move || match mode.get() {
            0 => view! { <div class="content"><p>"Mode 0"</p></div> },
            1 => view! { <div class="content"><p>"Mode 1"</p></div> },
            _ => view! { <div class="content"><p>"Default"</p></div> },
        };
    }
}
