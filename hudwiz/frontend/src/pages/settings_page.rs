use yew::prelude::*;
use crate::services::state_service::{AppContext, AppAction};
use crate::components::headless::button::Button;

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    let app_context = use_context::<AppContext>().expect("No AppContext found");

    let set_theme = |theme_name: &str| {
        let dispatch = app_context.reducer.dispatcher().clone();
        let theme_name = theme_name.to_string();
        Callback::from(move |_| {
            dispatch.dispatch(AppAction::SetTheme(theme_name.clone()));
        })
    };

    let current_theme = app_context.reducer.theme.clone();

    html! {
        <div class="settings-page">
            <h2>{ "Settings" }</h2>
            <div class="theme-selector">
                <h3>{ "Theme" }</h3>
                <p>{ format!("Current Theme: {}", current_theme) }</p>
                <Button onclick={set_theme("light")} disabled={current_theme == "light"} class="button">
                    { "Light Theme" }
                </Button>
                <Button onclick={set_theme("dark")} disabled={current_theme == "dark"} class="button button-primary">
                    { "Dark Theme" }
                </Button>
                <Button onclick={set_theme("dracula")} disabled={current_theme == "dracula"} class="button button-primary">
                    { "Dracula" }
                </Button>
                <Button onclick={set_theme("nord")} disabled={current_theme == "nord"} class="button button-primary">
                    { "Nord" }
                </Button>
                <Button onclick={set_theme("github-light")} disabled={current_theme == "github-light"} class="button">
                    { "GitHub Light" }
                </Button>
                <Button onclick={set_theme("solarized-light")} disabled={current_theme == "solarized-light"} class="button">
                    { "Solarized Light" }
                </Button>
                <Button onclick={set_theme("monokai")} disabled={current_theme == "monokai"} class="button button-primary">
                    { "Monokai" }
                </Button>
                <Button onclick={set_theme("cyberpunk")} disabled={current_theme == "cyberpunk"} class="button button-primary">
                    { "Cyberpunk" }
                </Button>
            </div>
        </div>
    }
}
