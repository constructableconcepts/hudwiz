use yew::prelude::*;
use crate::plugins::Plugin;
use crate::components::content_renderer::ContentRenderer;
use crate::services::state_service::ContentType;

#[derive(Debug)]
pub struct VideoPlugin;

impl Plugin for VideoPlugin {
    fn id(&self) -> &'static str {
        "video"
    }

    fn name(&self) -> &'static str {
        "Video"
    }

    fn view(&self) -> Html {
        let content_type = ContentType::Video("http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_string());
        html! {
            <ContentRenderer content_type={content_type} width={"150px".to_string()} height={"150px".to_string()} />
        }
    }
}
