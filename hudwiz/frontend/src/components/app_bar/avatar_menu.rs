use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use material_yew::Icon;
use crate::services::animation_service;
use web_sys::Element;

#[function_component(AvatarMenu)]
pub fn avatar_menu() -> Html {
    let node_ref = use_node_ref();

    {
        let node_ref = node_ref.clone();
        use_effect_with((), move |_| {
            if let Some(element) = node_ref.cast::<Element>() {
                animation_service::apply_animation(element, "slide-up", 400);
            }
            || ()
        });
    }

    html! {
        <div ref={node_ref} class="avatar-menu">
            <ul>
                <li>
                    <Link<Route> to={Route::Settings} classes="menu-item-link">
                        <div class="menu-item-content">
                            <Icon icon={"settings"} />
                            <span>{ "Settings" }</span>
                        </div>
                    </Link<Route>>
                </li>
                <li>
                    <button class="menu-item-button">
                        <div class="menu-item-content">
                            <Icon icon={"logout"} />
                            <span>{ "Log out" }</span>
                        </div>
                    </button>
                </li>
            </ul>
        </div>
    }
}