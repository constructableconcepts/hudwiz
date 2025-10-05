use yew::prelude::*;
use web_sys::Element;
use log::info;
use crate::services::state_service::AppContext;

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let app_context = use_context::<AppContext>().expect("No AppContext found");
    let theme = app_context.reducer.theme.clone();
    let node_ref = use_node_ref();

    {
        let node_ref = node_ref.clone();
        let theme = theme.clone();
        use_effect_with(theme, move |theme| {
            if let Some(element) = node_ref.cast::<Element>() {
                info!("Setting data-theme to: {}", theme);
                element.set_attribute("data-theme", &theme).unwrap();
            }
            || ()
        });
    }

    html! {
        <div ref={node_ref} class="theme-provider-root">
            { for props.children.iter() }
        </div>
    }
}
