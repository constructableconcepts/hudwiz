use yew::prelude::*;
use web_sys::Element;
use crate::services::animation_service;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub aria_label: Option<String>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let node_ref = use_node_ref();

    let handle_click = {
        let onclick = props.onclick.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(element) = node_ref.cast::<Element>() {
                animation_service::apply_animation(element, "pulse-animation", 200);
            }
            onclick.emit(e);
        })
    };

    let mut classes = props.class.clone();
    classes.push("headless-button");

    html! {
        <button
            ref={node_ref}
            class={classes}
            onclick={handle_click}
            disabled={props.disabled}
            aria-label={props.aria_label.clone()}
        >
            { for props.children.iter() }
        </button>
    }
}