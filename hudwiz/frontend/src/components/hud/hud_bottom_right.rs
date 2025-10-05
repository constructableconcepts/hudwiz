use yew::prelude::*;
use std::rc::Rc;
use crate::services::state_service::{AppContext, AppAction};
use crate::plugins::PluginRegistry;
use material_yew::Icon;

#[derive(Properties, PartialEq)]
pub struct HudBottomRightProps {
    #[prop_or_default]
    pub plugins: Rc<PluginRegistry>,
}

#[function_component(HudBottomRight)]
pub fn hud_bottom_right(props: &HudBottomRightProps) -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let collapsed = app_context.reducer.hud_bottom_right_collapsed;

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
            dispatch.dispatch(AppAction::ToggleHudBottomRight);
        })
    };

    let class = classes!(
        "hud-region",
        "corner-region",
        "hud-bottom-right",
        if collapsed { "collapsed" } else { "expanded" }
    );

    html! {
        <div class={class}>
            <div class="drawer-header">
                if !collapsed {
                    <span class="drawer-title">{"Bottom Right HUD"}</span>
                }
                <div onclick={ontoggle} class="drawer-toggle-icon">
                    <Icon icon={if collapsed { "menu" } else { "menu_open" }} />
                </div>
            </div>
            if !collapsed {
                <div class="drawer-content">
                    { plugins_html }
                </div>
            }
        </div>
    }
}
