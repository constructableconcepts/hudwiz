pub mod auth;
pub mod components;
pub mod pages;
pub mod router;
pub mod services;
pub mod plugins;

use yew::prelude::*;
use yew_router::prelude::*;
use router::{switch, Route};
use crate::services::theme_service::ThemeProvider;
use crate::services::state_service::AppStateProvider;
use crate::services::api_service::ApiProvider;
use crate::services::auth_service::AuthProvider;
use crate::services::realtime_provider::RealtimeProvider;
use wasm_bindgen::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AppStateProvider>
            <ThemeProvider>
                <ApiProvider>
                    <AuthProvider>
                        <RealtimeProvider>
                            <BrowserRouter>
                                <Switch<Route> render={switch} />
                            </BrowserRouter>
                        </RealtimeProvider>
                    </AuthProvider>
                </ApiProvider>
            </ThemeProvider>
        </AppStateProvider>
    }
}

// This is the entry point for the web app
#[wasm_bindgen(start)]
pub fn run_app() {
    // This provides better error messages in the browser console
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    log::info!("App starting...");
    yew::Renderer::<App>::new().render();
}
