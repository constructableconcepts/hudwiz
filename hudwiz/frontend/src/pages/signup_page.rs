use yew::prelude::*;
use yew_router::prelude::*;
use yew::virtual_dom::{VNode, AttrValue};
use crate::router::Route;
use material_yew::Icon;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use web_sys::HtmlInputElement;
use serde_json::json;

const GOOGLE_ICON_SVG: &str = r##"<svg class="auth-button-icon" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" fill="#4285F4"/><path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853"/><path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/><path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/><path d="M1 1h22v22H1z" fill="none"/></svg>"##;
const MICROSOFT_ICON_SVG: &str = r##"<svg class="auth-button-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 23 23"><path fill="#f3f3f3" d="M0 0h23v23H0z"/><path fill="#f35325" d="M1 1h10v10H1z"/><path fill="#81bc06" d="M12 1h10v10H12z"/><path fill="#05a6f0" d="M1 12h10v10H1z"/><path fill="#ffba08" d="M12 12h10v10H12z"/></svg>"##;
const APPLE_ICON_SVG: &str = r##"<svg class="auth-button-icon" xmlns="http://www.w3.org/2000/svg" xml:space="preserve" viewBox="0 0 814 1000"><path fill="black" d="M788.1 340.9c-5.8 4.5-108.2 62.2-108.2 190.5 0 148.4 130.3 200.9 134.2 202.2-.6 3.2-20.7 71.9-68.7 141.9-42.8 61.6-87.5 123.1-155.5 123.1s-85.5-39.5-164-39.5c-76.5 0-103.7 40.8-165.9 40.8s-105.6-57-155.5-127C46.7 790.7 0 663 0 541.8c0-194.4 126.4-297.5 250.8-297.5 66.1 0 121.2 43.4 162.7 43.4 39.5 0 101.1-46 176.3-46 28.5 0 130.9 2.6 198.3 99.2zm-234-181.5c31.1-36.9 53.1-88.1 53.1-139.3 0-7.1-.6-14.3-1.9-20.1-50.6 1.9-110.8 33.7-147.1 75.8-28.5 32.4-55.1 83.6-55.1 135.5 0 7.8 1.3 15.6 1.9 18.1 3.2.6 8.4 1.3 13.6 1.3 45.4 0 102.5-30.4 135.5-71.3z"/></svg>"##;


#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let navigator = use_navigator().unwrap();
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let error_state = use_state(|| None::<String>);

    let onsubmit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        let navigator = navigator.clone();
        let error_state = error_state.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let navigator = navigator.clone();
            let error_state = error_state.clone();

            spawn_local(async move {
                let client = Client::new();
                let location = web_sys::window().unwrap().location();
                let base_url = format!("{}//{}", location.protocol().unwrap(), location.host().unwrap());
                let res = client
                    .post(format!("{}/api/auth/signup", base_url))
                    .json(&json!({ "username": username, "password": password }))
                    .send()
                    .await;

                match res {
                    Ok(response) => {
                        if response.status().is_success() {
                            navigator.push(&Route::Login);
                        } else {
                            let error_msg = response.text().await.unwrap_or_else(|_| "An unknown error occurred.".to_string());
                            error_state.set(Some(format!("Signup failed: {}", error_msg)));
                        }
                    }
                    Err(e) => {
                        error_state.set(Some(format!("An error occurred: {}", e)));
                    }
                }
            });
        })
    };

    let google_icon = VNode::from_html_unchecked(AttrValue::from(GOOGLE_ICON_SVG));
    let microsoft_icon = VNode::from_html_unchecked(AttrValue::from(MICROSOFT_ICON_SVG));
    let apple_icon = VNode::from_html_unchecked(AttrValue::from(APPLE_ICON_SVG));

    html! {
        <div class="auth-page">
            <div class="auth-container">
                <div class="auth-icon">
                    <Icon icon={{ "desktop_windows" }} />
                </div>
                <h2>{ "Create an Account" }</h2>
                <form onsubmit={onsubmit}>
                    {
                        if let Some(error) = &*error_state {
                            html!{ <p class="error">{error}</p> }
                        } else {
                            html!{}
                        }
                    }
                    <div class="form-field">
                        <label for="email">{ "Email" }</label>
                        <input type="email" id="email" required=true ref={username_ref} />
                    </div>
                    <div class="form-field">
                        <label for="password">{ "Password" }</label>
                        <input type="password" id="password" required=true ref={password_ref} />
                    </div>
                    <div class="form-field">
                        <label for="confirm-password">{ "Confirm Password" }</label>
                        <input type="password" id="confirm-password" required=true />
                    </div>
                    <button type="submit" class="auth-button">{ "Sign Up" }</button>
                </form>
                <div style="margin-top: 24px; margin-bottom: 16px; text-align: center; color: #888;">{ "or" }</div>
                <button class="auth-button google-blue">
                    { google_icon }
                    <span>{ "Sign up with Google" }</span>
                </button>
                <button class="auth-button microsoft-blue">
                    { microsoft_icon }
                    <span>{ "Sign up with Microsoft" }</span>
                </button>
                <button class="auth-button apple-black">
                    { apple_icon }
                    <span>{ "Sign up with Apple" }</span>
                </button>
                <div class="auth-links">
                    <Link<Route> to={Route::Login}>{ "Already have an account? Log in" }</Link<Route>>
                </div>
            </div>
        </div>
    }
}
