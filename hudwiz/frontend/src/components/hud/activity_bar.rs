use yew::prelude::*;
use material_yew::Icon;
use crate::services::state_service::{AppContext, AppAction};
use crate::components::chat::chat_input_bar::ChatInputBar;

const ICONS: [&str; 8] = [
    "add", "mic", "sentiment_satisfied", "image",
    "replay", "stop_circle", "content_copy", "thumb_up",
];

#[function_component(ActivityBar)]
pub fn activity_bar() -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let collapsed = app_context.reducer.activity_bar_collapsed;

    let on_toggle_collapse = {
        let dispatch = app_context.reducer.dispatcher().clone();
        Callback::from(move |_| {
            dispatch.dispatch(AppAction::ToggleActivityBar);
        })
    };

    let icons_html = ICONS.iter().map(|icon_name| {
        html! {
            <div class="user-drawer-icon-item">
                <Icon icon={*icon_name} />
            </div>
        }
    }).collect::<Html>();

    let main_class = classes!(
        "hud-region",
        "activity-bar",
        if collapsed { "collapsed" } else { "expanded" }
    );

    html! {
        <div class={main_class}>
            if !collapsed {
                <div class="user-drawer-input-container">
                    <ChatInputBar />
                </div>
            }
            <div class="user-drawer-icon-row">
                <div class="user-drawer-left-icons">
                    <div class="user-drawer-icon-item" onclick={on_toggle_collapse}>
                        <Icon icon={"forum"} />
                    </div>
                    { icons_html }
                </div>
                <div class="user-drawer-right-icons">
                     <div class="user-drawer-icon-item">
                        <Icon icon={"settings"} />
                    </div>
                </div>
            </div>
        </div>
    }
}