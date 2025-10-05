use yew::prelude::*;
use crate::services::state_service::{AppContext, AppAction};
use material_yew::Icon;

#[function_component(HudTopRight)]
pub fn hud_top_right() -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let collapsed = app_context.reducer.hud_top_right_collapsed;

    let ontoggle = {
        let dispatch = app_context.reducer.dispatcher().clone();
        Callback::from(move |_| {
            dispatch.dispatch(AppAction::ToggleHudTopRight);
        })
    };

    let class = classes!(
        "hud-region",
        "corner-region",
        "hud-top-right",
        if collapsed { "collapsed" } else { "expanded" }
    );

    html! {
        <div class={class}>
            <div onclick={ontoggle} class="drawer-toggle-icon">
                <Icon icon={{ if collapsed { "menu" } else { "menu_open" } }} />
            </div>
            if !collapsed {
                <div>{ "Top Right HUD Content" }</div>
            }
        </div>
    }
}
