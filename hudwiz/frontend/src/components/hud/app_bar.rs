use yew::prelude::*;
use crate::services::state_service::{AppContext};
use crate::components::app_bar::avatar_menu::AvatarMenu;
use crate::components::app_bar::app_bar_toolbar::AppBarToolbar;
use material_yew::Icon;

#[function_component(AppBar)]
pub fn app_bar() -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let menu_open_handle = use_state(|| false);

    let toggle_menu = {
        let menu_open_handle = menu_open_handle.clone();
        Callback::from(move |_| {
            menu_open_handle.set(!*menu_open_handle);
        })
    };

    let class = classes!(
        "hud-region",
        "app-bar",
        "expanded"
    );

    html! {
        <div class={class}>
            <div class="left-group">
                <Icon icon={{ "desktop_windows" }} />
            </div>
            <div class="center-group">
                <h1>{ app_context.reducer.title.clone() }</h1>
            </div>
            <div class="right-group">
                <AppBarToolbar />
                <div class="avatar-container" onclick={toggle_menu}>
                    <Icon icon={{ "account_circle" }} />
                    if *menu_open_handle {
                        <AvatarMenu />
                    }
                </div>
            </div>
        </div>
    }
}
