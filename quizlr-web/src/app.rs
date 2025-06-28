use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
// use leptos_router::components::Outlet;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Quizlr - Adaptive Learning Platform"/>
        <MetaTags/>
        <Stylesheet id="leptos" href="/pkg/quizlr.css"/>
        
        <div class="min-h-screen bg-gray-50">
            <header class="bg-white shadow">
                <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                    <h1 class="text-3xl font-bold text-gray-900">
                        "Quizlr"
                    </h1>
                </div>
            </header>
            
            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <div class="px-4 py-6 sm:px-0">
                    <div>"Welcome to Quizlr!"</div>
                </div>
            </main>
        </div>
    }
}