use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    log!("App component mounting");

    // State for showing different views
    let (current_view, set_current_view) = signal("home".to_string());
    let (quizzes, _set_quizzes) = signal(Vec::<String>::new());

    log!("Initial view: {}", current_view.get());

    // Event handlers
    let on_create_quiz = move |_| {
        log!("Create Quiz clicked");
        set_current_view.set("create".to_string());
    };

    let on_import_content = move |_| {
        log!("Import Content clicked");
        set_current_view.set("import".to_string());
    };

    let on_go_home = move |_| {
        set_current_view.set("home".to_string());
    };

    view! {
        <div class="min-h-screen bg-gray-50">
            <header class="bg-white shadow-sm border-b border-gray-200 px-6 py-4">
                <h1 class="text-3xl font-bold text-gray-900">{"Quizlr"}</h1>
                <p class="text-gray-600 mt-1">{"Adaptive Learning Platform"}</p>
            </header>

            <main class="container mx-auto px-6 py-8">
                <Show
                    when=move || current_view.get() == "create"
                    fallback=move || view! {
                        <Show
                            when=move || current_view.get() == "import"
                            fallback=move || view! {
                                // Home view
                                <>
                                    <div class="bg-white rounded-lg shadow-md p-6">
                                        <h2 class="text-2xl font-semibold mb-4">{"Welcome to Quizlr!"}</h2>
                                        <p class="text-gray-600 mb-6">
                                            {"Get started by creating your first quiz or importing existing content."}
                                        </p>
                                        <div class="flex gap-3">
                                            <button
                                                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                                                on:click=on_create_quiz
                                            >
                                                {"Create Quiz"}
                                            </button>
                                            <button
                                                class="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300 transition-colors"
                                                on:click=on_import_content
                                            >
                                                {"Import Content"}
                                            </button>
                                        </div>
                                    </div>

                                    <div class="mt-8">
                                        <Show
                                            when=move || quizzes.get().is_empty()
                                            fallback=move || view! {
                                                <div class="quiz-list grid gap-4">
                                                    <For
                                                        each=move || quizzes.get().into_iter()
                                                        key=|quiz| quiz.clone()
                                                        children=move |quiz| view! {
                                                            <div class="bg-white rounded-lg shadow-md p-4 hover:shadow-lg transition-shadow">
                                                                <h3 class="font-semibold">{quiz}</h3>
                                                            </div>
                                                        }
                                                    />
                                                </div>
                                            }
                                        >
                                            <div class="text-center py-12">
                                                <p class="text-gray-500 text-lg">
                                                    {"No quizzes yet. Create your first quiz to get started!"}
                                                </p>
                                            </div>
                                        </Show>
                                    </div>
                                </>
                            }
                        >
                            // Import view
                            <div class="bg-white rounded-lg shadow-md p-6">
                                <h2 class="text-2xl font-semibold mb-4">{"Import Content"}</h2>
                                <p class="text-gray-600 mb-4">{"Content import coming soon..."}</p>
                                <button
                                    class="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300 transition-colors"
                                    on:click=on_go_home
                                >
                                    {"Back to Home"}
                                </button>
                            </div>
                        </Show>
                    }
                >
                    // Create view
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <h2 class="text-2xl font-semibold mb-4">{"Create New Quiz"}</h2>
                        <p class="text-gray-600 mb-4">{"Quiz creation coming soon..."}</p>
                        <button
                            class="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300 transition-colors"
                            on:click=on_go_home
                        >
                            {"Back to Home"}
                        </button>
                    </div>
                </Show>
            </main>
        </div>
    }
}
