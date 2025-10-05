use yew::prelude::*;
use material_yew::Icon;

const ICONS: [&str; 10] = [
    "near_me",
    "create",
    "rectangle",
    "radio_button_unchecked",
    "timeline",
    "text_fields",
    "sticky_note_2",
    "comment",
    "aspect_ratio",
    "file_upload",
];

#[function_component(FloatingToolbar)]
pub fn floating_toolbar() -> Html {
    let icons_html = ICONS.iter().map(|icon_name| {
        html! {
            <div class="toolbar-item">
                <Icon icon={{*icon_name}} />
            </div>
        }
    }).collect::<Html>();

    html! {
        <div id="floating-toolbar">
            { icons_html }
        </div>
    }
}
