use yew::prelude::*;
use std::rc::Rc;
use crate::services::state_service::{AppContext, AppAction};
use crate::plugins::PluginRegistry;
use material_yew::Icon;

#[derive(Properties, PartialEq)]
pub struct HudBottomLeftProps {
    #[prop_or_default]
    pub plugins: Rc<PluginRegistry>,
}

#[function_component(HudBottomLeft)]
pub fn hud_bottom_left(props: &HudBottomLeftProps) -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let collapsed = app_context.reducer.hud_bottom_left_collapsed;

    let plugins_html = props.plugins.get_all().iter().map(|plugin| {
        html! {
            <div class="drawer-item">
                if !collapsed {
                    <h4>{plugin.name()}</h4>
                }
                {plugin.view()}
            </div>
        }
    }).collect::<Html>();

    let ontoggle = {
        let dispatch = app_context.reducer.dispatcher().clone();
        Callback::from(move |_| {
            dispatch.dispatch(AppAction::ToggleHudBottomLeft);
        })
    };

    let class = classes!(
        "hud-region",
        "corner-region",
        "hud-bottom-left",
        if collapsed { "collapsed" } else { "expanded" }
    );

    html! {
        <div class={class}>
            <div class="drawer-header">
                <div onclick={ontoggle} class="drawer-toggle-icon">
                    <Icon icon={if collapsed { "menu" } else { "menu_open" }} />
                </div>
                if !collapsed {
                    <span class="drawer-title">{"Bottom Left HUD"}</span>
                }
            </div>
            if !collapsed {
                <div class="drawer-content">
                    { plugins_html }
                </div>
            }
        </div>
    }
}
