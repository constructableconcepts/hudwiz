use yew::prelude::*;
use crate::plugins::Plugin;
use crate::components::content_renderer::ContentRenderer;
use crate::services::state_service::ContentType;

#[derive(Debug)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn id(&self) -> &'static str {
        "map"
    }

    fn name(&self) -> &'static str {
        "Map"
    }

    fn view(&self) -> Html {
        let content_type = ContentType::IFrame("https://www.openstreetmap.org/export/embed.html?bbox=-0.222%2C51.469%2C0.000%2C51.531&layer=mapnik".to_string());
        html! {
            <ContentRenderer content_type={content_type} width={"150px".to_string()} height={"150px".to_string()} />
        }
    }
}
