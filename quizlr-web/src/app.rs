use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <header>
                <h1>{"Quizlr"}</h1>
                <p>{"Adaptive Learning Platform"}</p>
            </header>

            <main>
                <div class="card">
                    <h2>{"Welcome to Quizlr!"}</h2>
                    <p>{"Get started by creating your first quiz or importing existing content."}</p>
                    <div style="margin-top: 1rem;">
                        <button class="primary">{"Create Quiz"}</button>
                        <button class="secondary" style="margin-left: 0.5rem;">{"Import Content"}</button>
                    </div>
                </div>
            </main>
        </div>
    }
}
