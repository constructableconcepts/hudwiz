use yew::prelude::*;
use material_yew::Icon;
use crate::services::state_service::{AppContext, AppAction};
use crate::services::i18n_service::use_translation;

const ICONS: [&str; 10] = [
    "info",
    "help",
    "history",
    "toc",
    "schedule",
    "cloud",
    "attachment",
    "api",
    "build",
    "code",
];

#[function_component(SidebarRight)]
pub fn sidebar_right() -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let i18n = use_translation();
    let collapsed = app_context.reducer.sidebar_right_collapsed;

    let icons_html = ICONS.iter().map(|icon_name| {
        html! {
            <div class="drawer-item">
                <Icon icon={{*icon_name}} />
                if !collapsed {
                    <span>{*icon_name}</span>
                }
            </div>
        }
    }).collect::<Html>();

    let ontoggle = {
        let dispatch = app_context.reducer.dispatcher().clone();
        Callback::from(move |_| {
            dispatch.dispatch(AppAction::ToggleSidebarRight);
        })
    };

    let class = classes!(
        "hud-region",
        "sidebar-right",
        "drawer",
        if collapsed { "collapsed" } else { "expanded" }
    );

    let mut toggle_icon_class = classes!("drawer-toggle-icon");
    if !collapsed {
        toggle_icon_class.push("mirrored-icon");
    }

    html! {
        <div class={class}>
            <div class="drawer-header">
                <div onclick={ontoggle} class={toggle_icon_class}>
                    <Icon icon={{ if collapsed { "chevron_left" } else { "chevron_right" } }} />
                </div>
                if !collapsed {
                    <span class="drawer-title">{i18n.tr("details-title")}</span>
                }
            </div>
            <div class="drawer-content">
                { icons_html }
            </div>
        </div>
    }
}