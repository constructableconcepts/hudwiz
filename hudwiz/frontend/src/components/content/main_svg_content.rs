use yew::prelude::*;
use yew::virtual_dom::VNode;

const SVG_CONTENT: &str = r#"
<svg width="1200" height="900" viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg">
    <rect width="500" height="500" fill="rgba(100, 100, 255, 0.1)" />
    <circle cx="250" cy="250" r="100" fill="rgba(255, 100, 100, 0.3)" />
    <line x1="50" y1="50" x2="450" y2="450" stroke="rgba(255, 255, 255, 0.2)" stroke-width="2" />
    <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" font-family="sans-serif" font-size="24px" fill="rgba(255, 255, 255, 0.5)">
        {"Main SVG Content"}
    </text>
</svg>
"#;

#[function_component(MainSvgContent)]
pub fn main_svg_content() -> Html {
    let svg_vnode = VNode::from_html_unchecked(SVG_CONTENT.into());
    html! {
        { svg_vnode }
    }
}
