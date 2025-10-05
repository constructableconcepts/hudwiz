use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use gloo_console::log;
use material_yew::Icon;
use crate::services::auth_service::AuthContext;
use web_sys::HtmlInputElement;

use yew::virtual_dom::VNode;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    log!("LoginPage mounted");
    let google_logo_svg = r##"<svg class="sso-logo" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" fill="#4285F4"/><path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853"/><path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/><path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/><path d="M1 1h22v22H1z" fill="none"/></svg>"##;
    let microsoft_logo_svg = r##"<svg class="sso-logo" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 21 21"><path fill="#f35325" d="M0 0h10v10H0z"/><path fill="#81bc06" d="M11 0h10v10H11z"/><path fill="#05a6f0" d="M0 11h10v10H0z"/><path fill="#ffba08" d="M11 11h10v10H11z"/></svg>"##;

    let google_logo = VNode::from_html_unchecked(google_logo_svg.into());
    let microsoft_logo = VNode::from_html_unchecked(microsoft_logo_svg.into());

    let navigator = use_navigator().unwrap();
    let auth_context = use_context::<AuthContext>().expect("no auth context found");

    let username_ref = use_node_ref();
    let password_ref = use_node_ref();

    let onsubmit = {
        let login = auth_context.login.clone();
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            log!(format!("LoginPage submitting login for user: {}", username));
            login.emit((username, password));
        })
    };

    {
        let user = auth_context.user.clone();
        let navigator = navigator.clone();
        use_effect_with(user.clone(), move |user| {
            log!(format!("LoginPage use_effect_with running, user: {:?}", user));
            if user.is_some() {
                log!("LoginPage redirecting to /app");
                navigator.push(&Route::App);
            }
            || ()
        });
    }

    let on_google_signin = Callback::from(|_| {
        web_sys::window().unwrap().location().set_href("/api/auth/sso/login/google").unwrap();
    });
    let on_microsoft_signin = Callback::from(|_| {
        web_sys::window().unwrap().location().set_href("/api/auth/sso/login/microsoft").unwrap();
    });

    html! {
        <div class="auth-page">
            <div class="auth-container">
                <div class="auth-icon">
                    <Icon icon={{ "desktop_windows" }} />
                </div>
                <h2>{ "Login to hudwiz" }</h2>
                <form onsubmit={onsubmit}>
                    <div class="form-field">
                        <label for="email">{ "Email" }</label>
                        <input type="email" id="email" required=true ref={username_ref} />
                    </div>
                    <div class="form-field">
                        <label for="password">{ "Password" }</label>
                        <input type="password" id="password" required=true ref={password_ref} />
                    </div>
                    <button type="submit" class="auth-button">{ "Login" }</button>
                </form>
                <div class="auth-links">
                    <Link<Route> to={Route::Signup}>{ "Don't have an account? Sign up" }</Link<Route>>
                </div>
                <hr class="sso-divider" />
                <div class="sso-buttons">
                    <button onclick={on_google_signin} class="auth-button google">
                        { google_logo }
                        { "Sign in with Google" }
                    </button>
                    <button onclick={on_microsoft_signin} class="auth-button microsoft">
                        { microsoft_logo }
                        { "Sign in with Microsoft" }
                    </button>
                </div>
            </div>
        </div>
    }
}
