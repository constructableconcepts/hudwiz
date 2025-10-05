use yew::prelude::*;
use crate::services::state_service::{AppContext, AppAction, ContentType};
use material_yew::Icon;
use log::info;
use crate::services::i18n_service::use_translation;

#[function_component(SidebarLeft)]
pub fn sidebar_left() -> Html {
    let app_context = use_context::<AppContext>().expect("No AppContext found");
    let i18n = use_translation();
    let collapsed = app_context.reducer.sidebar_left_collapsed;
    let dispatch = app_context.reducer.dispatcher();

    let ontoggle = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            info!("Sidebar left toggle clicked!");
            dispatch.dispatch(AppAction::ToggleSidebarLeft);
            info!("New collapsed state: {}", !collapsed);
        })
    };

    let mut drawer_class = classes!("hud-region", "sidebar-left", "drawer");
    if collapsed {
        drawer_class.push("collapsed");
    } else {
        drawer_class.push("expanded");
    }

    html! {
        <div id="sidebar-left-drawer" class={drawer_class}>
            <div class="drawer-header">
                if !collapsed {
                    <span class="drawer-title">{i18n.tr("controls-title")}</span>
                }
                <div onclick={ontoggle} class="drawer-toggle-icon">
                    <Icon icon={{ if collapsed { "chevron_right" } else { "chevron_left" } }} />
                </div>
            </div>
            <div class="drawer-content">
                if !collapsed {
                    <span class="drawer-title">{"Language"}</span>
                }
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetLanguage("en-US".to_string()))
                }>
                    <Icon icon={{"language"}} />
                    if !collapsed {
                        <span>{"English"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetLanguage("es-ES".to_string()))
                }>
                    <Icon icon={{"language"}} />
                    if !collapsed {
                        <span>{"Español"}</span>
                    }
                </div>
                <hr />
                if !collapsed {
                    <span class="drawer-title">{"Background"}</span>
                }
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetBackgroundContent(ContentType::AnimatedSvg))
                }>
                    <Icon icon={{"animation"}} />
                    if !collapsed {
                        <span>{"Animated SVG"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetBackgroundContent(ContentType::SolidColor("grey".to_string())))
                }>
                    <Icon icon={{"color_lens"}} />
                    if !collapsed {
                        <span>{"Solid Color"}</span>
                    }
                </div>
                <hr />
                if !collapsed {
                    <span class="drawer-title">{"Main Content"}</span>
                }
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetMainContent(ContentType::TransparentSvg))
                }>
                    <Icon icon={{"wallpaper"}} />
                    if !collapsed {
                        <span>{"Transparent SVG"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetMainContent(ContentType::IFrame("https://www.openstreetmap.org/export/embed.html?bbox=-0.004017949104309083%2C51.47612752342468%2C0.004017949104309083%2C51.47812752342468&layer=mapnik".to_string())))
                }>
                    <Icon icon={{"map"}} />
                    if !collapsed {
                        <span>{"IFrame"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetMainContent(ContentType::Video("http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_string())))
                }>
                    <Icon icon={{"movie"}} />
                    if !collapsed {
                        <span>{"Video"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetMainContent(ContentType::WebGL("https://codepen.io/arqinotes/embed/GRaNyPw/?theme-id=modal".to_string())))
                }>
                    <Icon icon={{"view_in_ar"}} />
                    if !collapsed {
                        <span>{"WebGL"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetMainContent(ContentType::Empty))
                }>
                    <Icon icon={{"crop_square"}} />
                    if !collapsed {
                        <span>{"Empty"}</span>
                    }
                </div>
                <hr />
                if !collapsed {
                    <span class="drawer-title">{"Themes"}</span>
                }
                // Dark Themes
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetTheme("dracula".to_string()))
                }>
                    <Icon icon={{"brightness_3"}} />
                    if !collapsed {
                        <span>{"Dracula"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetTheme("nord".to_string()))
                }>
                    <Icon icon={{"ac_unit"}} />
                    if !collapsed {
                        <span>{"Nord"}</span>
                    }
                </div>
                // Light Themes
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetTheme("github-light".to_string()))
                }>
                    <Icon icon={{"light_mode"}} />
                    if !collapsed {
                        <span>{"GitHub Light"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetTheme("solarized-light".to_string()))
                }>
                    <Icon icon={{"wb_sunny"}} />
                    if !collapsed {
                        <span>{"Solarized Light"}</span>
                    }
                </div>
                // Custom Themes
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetTheme("monokai".to_string()))
                }>
                    <Icon icon={{"palette"}} />
                    if !collapsed {
                        <span>{"Monokai"}</span>
                    }
                </div>
                <div class="drawer-item" onclick={
                    let dispatch = dispatch.clone();
                    move |_| dispatch.dispatch(AppAction::SetTheme("cyberpunk".to_string()))
                }>
                    <Icon icon={{"electric_bolt"}} />
                    if !collapsed {
                        <span>{"Cyberpunk"}</span>
                    }
                </div>
            </div>
        </div>
    }
}
