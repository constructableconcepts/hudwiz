use yew::prelude::*;
use material_yew::Icon;

#[function_component(AppBarToolbar)]
pub fn app_bar_toolbar() -> Html {
    html! {
        <div class="app-bar-toolbar">
            <Icon icon={"notifications"} />
            <Icon icon={"palette"} />
            <Icon icon={"design_services"} />
        </div>
    }
}
