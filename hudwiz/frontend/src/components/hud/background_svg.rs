use yew::prelude::*;

#[function_component(BackgroundSvg)]
pub fn background_svg() -> Html {
    let svg_content = r#"
<svg width="100%" height="100%" preserveAspectRatio="xMidYMid slice" viewBox="0 0 1440 560" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <filter id="blur-filter" x="-50%" y="-50%" width="200%" height="200%">
            <feGaussianBlur in="SourceGraphic" stdDeviation="5" />
        </filter>
    </defs>

    <rect width="1440" height="560" fill="#0a0a23" />

    <g filter="url(#blur-filter)">
        <circle cx="150" cy="150" r="40" fill="rgba(0, 255, 0, 0.2)">
            <animate attributeName="r" values="35;45;35" dur="7s" repeatCount="indefinite" />
            <animate attributeName="cx" values="150;250;150" dur="20s" repeatCount="indefinite" />
        </circle>
        <circle cx="1100" cy="400" r="60" fill="rgba(0, 0, 255, 0.2)">
            <animate attributeName="r" values="55;65;55" dur="8s" repeatCount="indefinite" />
            <animate attributeName="cy" values="400;300;400" dur="18s" repeatCount="indefinite" />
        </circle>
        <circle cx="800" cy="100" r="25" fill="rgba(255, 255, 255, 0.2)">
            <animate attributeName="r" values="20;30;20" dur="6s" repeatCount="indefinite" />
            <animate attributeName="cx" values="800;900;800" dur="15s" repeatCount="indefinite" />
        </circle>
        <circle cx="400" cy="500" r="50" fill="rgba(255, 0, 0, 0.2)">
            <animate attributeName="r" values="45;55;45" dur="9s" repeatCount="indefinite" />
            <animate attributeName="cy" values="500;450;500" dur="12s" repeatCount="indefinite" />
        </circle>
        <circle cx="600" cy="250" r="30" fill="rgba(204, 204, 255, 0.2)">
            <animate attributeName="r" values="25;35;25" dur="5s" repeatCount="indefinite" />
            <animate attributeName="cx" values="600;500;600" dur="17s" repeatCount="indefinite" />
        </circle>
         <circle cx="1300" cy="200" r="45" fill="rgba(0, 255, 255, 0.2)">
            <animate attributeName="r" values="40;50;40" dur="10s" repeatCount="indefinite" />
            <animate attributeName="cy" values="200;300;200" dur="22s" repeatCount="indefinite" />
        </circle>
        <circle cx="50" cy="450" r="35" fill="rgba(255, 0, 255, 0.2)">
            <animate attributeName="r" values="30;40;30" dur="8s" repeatCount="indefinite" />
            <animate attributeName="cx" values="50;100;50" dur="14s" repeatCount="indefinite" />
        </circle>
    </g>
</svg>
"#;

    let div = web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
    div.set_inner_html(svg_content);
    Html::VRef(div.into())
}
