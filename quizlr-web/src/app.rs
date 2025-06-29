use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // State for showing different views
    let (current_view, set_current_view) = signal("home".to_string());
    let (quizzes, _set_quizzes) = signal(Vec::<String>::new());

    // Event handlers
    let on_create_quiz = move |_| {
        log!("Create Quiz clicked");
        set_current_view.set("create".to_string());
        // TODO: Navigate to quiz creation view
    };

    let on_import_content = move |_| {
        log!("Import Content clicked");
        set_current_view.set("import".to_string());
        // TODO: Navigate to import view
    };

    view! {
        <div class="min-h-screen bg-gray-50">
            <header class="bg-white shadow-sm border-b border-gray-200 px-6 py-4">
                <h1 class="text-3xl font-bold text-gray-900">{"Quizlr"}</h1>
                <p class="text-gray-600 mt-1">{"Adaptive Learning Platform"}</p>
            </header>

            <main class="container mx-auto px-6 py-8">
                {move || match current_view.get().as_str() {
                    "create" => view! {
                        <div class="bg-white rounded-lg shadow-md p-6">
                            <h2 class="text-2xl font-semibold mb-4">{"Create New Quiz"}</h2>
                            <p class="text-gray-600 mb-4">{"Quiz creation coming soon..."}</p>
                            <button
                                class="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300 transition-colors"
                                on:click=move |_| set_current_view.set("home".to_string())
                            >
                                {"Back to Home"}
                            </button>
                        </div>
                    }.into_any(),
                    "import" => view! {
                        <div class="bg-white rounded-lg shadow-md p-6">
                            <h2 class="text-2xl font-semibold mb-4">{"Import Content"}</h2>
                            <p class="text-gray-600 mb-4">{"Content import coming soon..."}</p>
                            <button
                                class="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300 transition-colors"
                                on:click=move |_| set_current_view.set("home".to_string())
                            >
                                {"Back to Home"}
                            </button>
                        </div>
                    }.into_any(),
                    _ => view! {
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

                        // Show quiz list or empty state
                        <div class="mt-8">
                            {move || if quizzes.get().is_empty() {
                                view! {
                                    <div class="text-center py-12">
                                        <p class="text-gray-500 text-lg">
                                            {"No quizzes yet. Create your first quiz to get started!"}
                                        </p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="quiz-list grid gap-4">
                                        {quizzes.get().iter().map(|quiz| {
                                            view! {
                                                <div class="bg-white rounded-lg shadow-md p-4 hover:shadow-lg transition-shadow">
                                                    <h3 class="font-semibold">{quiz.clone()}</h3>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }}
                        </div>
                    }.into_any()
                }}
            </main>
        </div>
    }
}
