use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainPanelProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MainPanel)]
pub fn main_panel(props: &MainPanelProps) -> Html {
    html! {
        <div class="hud-region main-panel">
            { for props.children.iter() }
        </div>
    }
}
