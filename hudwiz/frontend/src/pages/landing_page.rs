use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="landing-page">
            <div class="landing-nav">
                <div class="nav-left">
                    // Logo could go here
                </div>
                <div class="nav-right">
                    <Link<Route> to={Route::Login} classes="button button-secondary">{ "Sign In" }</Link<Route>>
                    <Link<Route> to={Route::Signup} classes="button button-primary">{ "Sign Up" }</Link<Route>>
                </div>
            </div>
            <main class="landing-main">
                <div class="hero-section">
                    <div class="hero-content">
                        <h1 class="hero-headline">{ "The AI-Native Development Environment" }</h1>
                        <p class="hero-subheadline">
                            { "hudwiz is a new type of development environment. One that understands your code, helps you build, and works with you, not against you." }
                        </p>
                        <div class="signup-form">
                            <input type="email" placeholder="email@example.com" class="signup-input" />
                            <Link<Route> to={Route::Signup} classes="button button-primary signup-button">{ "Sign Up" }</Link<Route>>
                        </div>
                    </div>
                    <div class="hero-image-container">
                        <img src="/static/images/hero-image.png" alt="hudwiz application screenshot" class="hero-image" />
                    </div>
                </div>
            </main>
            <footer class="landing-footer">
                <p>{ "© 2025 hudwiz, Inc." }</p>
            </footer>
        </div>
    }
}
