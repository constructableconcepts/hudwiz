use yew::prelude::*;
use material_yew::Icon;

#[derive(Clone, PartialEq, Default)]
pub enum TogglePosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl TogglePosition {
    fn to_class(&self) -> &'static str {
        match self {
            TogglePosition::TopRight => "toggle-top-right",
            TogglePosition::TopLeft => "toggle-top-left",
            TogglePosition::BottomRight => "toggle-bottom-right",
            TogglePosition::BottomLeft => "toggle-bottom-left",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub is_collapsible: bool,
    #[prop_or_default]
    pub is_collapsed: bool,
    #[prop_or_default]
    pub on_toggle: Callback<()>,
    #[prop_or_default]
    pub toggle_position: TogglePosition,
}

#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    let panel_class = classes!(
        "hud-region",
        "corner-region",
        props.class.clone(),
        props.toggle_position.to_class(),
        if props.is_collapsed { "collapsed" } else { "expanded" }
    );

    let on_toggle = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_| {
            on_toggle.emit(());
        })
    };

    html! {
        <div class={panel_class}>
            if props.is_collapsible {
                <div onclick={on_toggle} class="drawer-toggle-icon">
                    <Icon icon={{ if props.is_collapsed { "menu" } else { "menu_open" } }} />
                </div>
            }
            if !props.is_collapsed {
                <>
                    <span class="panel-title">{ &props.title }</span>
                    { for props.children.iter() }
                </>
            }
        </div>
    }
}
