use yew::prelude::*;
use crate::services::state_service::ContentType;
use crate::components::content::main_svg_content::MainSvgContent;

#[derive(Properties, PartialEq, Clone)]
pub struct ContentRendererProps {
    pub content_type: ContentType,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_default]
    pub height: Option<String>,
}

#[function_component(ContentRenderer)]
pub fn content_renderer(props: &ContentRendererProps) -> Html {
    let style = {
        let mut style = String::new();
        if let Some(width) = &props.width {
            style.push_str(&format!("width: {};", width));
        }
        if let Some(height) = &props.height {
            style.push_str(&format!("height: {};", height));
        }
        style
    };

    html! {
        <div class="content-renderer" style={style}>
            {
                match &props.content_type {
                    ContentType::Empty => html! {},
                    ContentType::TransparentSvg => html! { <MainSvgContent /> },
                    ContentType::IFrame(src) => html! {
                        <iframe class="content-iframe" src={src.clone()} />
                    },
                    ContentType::Video(src) => html! {
                        <video class="content-video" src={src.clone()} controls=true />
                    },
                    ContentType::WebGL(src) => html! {
                        <iframe class="content-iframe" src={src.clone()} />
                    },
                    ContentType::Image(src) => html! {
                        <img class="content-image" src={src.clone()} />
                    },
                    _ => html! {}
                }
            }
        </div>
    }
}
