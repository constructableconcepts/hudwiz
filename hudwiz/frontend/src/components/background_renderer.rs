use yew::prelude::*;
use crate::services::state_service::ContentType;
use crate::components::hud::background_canvas::BackgroundCanvas;

#[derive(Properties, PartialEq, Clone)]
pub struct BackgroundRendererProps {
    pub content_type: ContentType,
}

#[function_component(BackgroundRenderer)]
pub fn background_renderer(props: &BackgroundRendererProps) -> Html {
    html! {
        <div class="background-renderer">
            {
                match &props.content_type {
                    ContentType::AnimatedSvg => html! { <BackgroundCanvas /> },
                    ContentType::SolidColor(color) => html! {
                        <div style={format!("background-color: {}; width: 100%; height: 100%;", color)} />
                    },
                    // Render nothing for other content types
                    _ => html! {}
                }
            }
        </div>
    }
}
