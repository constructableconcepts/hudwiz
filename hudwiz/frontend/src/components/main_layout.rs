use yew::prelude::*;
use crate::services::state_service::AppContext;
use crate::services::config_service::ComponentType;
use crate::components::status_bar::StatusBar;
use crate::components::content_renderer::ContentRenderer;
use crate::components::background_renderer::BackgroundRenderer;

// Import all the components that can be dynamically rendered
use crate::components::hud::{
    main_panel::MainPanel,
    sidebar_left::SidebarLeft,
    sidebar_right::SidebarRight,
    activity_bar::ActivityBar,
    app_bar::AppBar,
    floating_toolbar::FloatingToolbar,
    hud_top_left::HudTopLeft,
    hud_top_right::HudTopRight,
    hud_bottom_left::HudBottomLeft,
    hud_bottom_right::HudBottomRight,
};
use crate::components::layouts::bento_grid::{BentoGrid, BentoGridItem};
use crate::components::chat::{
    chat_input_bar::ChatInputBar,
    chat_history_view::ChatHistoryView,
};

// Helper function to map enum to a component
fn render_component(component_type: &ComponentType) -> Html {
    match component_type {
        ComponentType::ControlsDrawer => html! { <SidebarLeft /> },
        ComponentType::DetailsDrawer => html! { <SidebarRight /> },
        ComponentType::UserDrawer => html! { <ActivityBar /> },
        ComponentType::DrawingToolbar => html! { <FloatingToolbar /> },
        ComponentType::TopLeftHud => html! { <HudTopLeft /> },
        ComponentType::TopRightHud => html! { <HudTopRight /> },
        ComponentType::BottomLeftHud => html! { <HudBottomLeft /> },
        ComponentType::BottomRightHud => html! { <HudBottomRight /> },
        // The config system doesn't currently support children, so for demonstration
        // purposes, we're hardcoding the layout here.
        ComponentType::BentoGrid => html! {
            <BentoGrid>
                <BentoGridItem col_span={2} row_span={1}><div>{"Component A (2x1)"}</div></BentoGridItem>
                <BentoGridItem><div>{"Component B"}</div></BentoGridItem>
                <BentoGridItem row_span={2}><div>{"Component C (1x2)"}</div></BentoGridItem>
                <BentoGridItem col_span={2} row_span={2}><div>{"Component D (2x2)"}</div></BentoGridItem>
                <BentoGridItem><div>{"Component E"}</div></BentoGridItem>
                <BentoGridItem><div>{"Component F"}</div></BentoGridItem>
            </BentoGrid>
        },
        ComponentType::ChatInputBar => html! { <ChatInputBar /> },
        ComponentType::ChatHistoryView => html! { <ChatHistoryView /> },
    }
}

#[function_component(MainLayout)]
pub fn main_layout() -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let state = &app_context.reducer;

    // Render a loading state if the config hasn't been loaded yet.
    if state.config.is_none() {
        return html! {
            <div class="loading-screen">
                <h1>{ "Loading Configuration..." }</h1>
            </div>
        };
    }

    let config = state.config.as_ref().unwrap();

    let middle_area_style = format!(
        "--left-region-width: {}; --right-region-width: {};",
        if state.sidebar_left_collapsed { "60px" } else { "250px" },
        if state.sidebar_right_collapsed { "60px" } else { "250px" },
    );

    let render_region = |region_name: &str| -> Html {
        config.layout.regions.get(region_name)
            .map_or_else(
                || html! {}, // Render nothing if region is not in config
                |region_config| render_component(&region_config.component)
            )
    };

    html! {
        <div id="main-layout">
            <BackgroundRenderer content_type={state.background_content.clone()} />
            <AppBar />
            <div id="middle-area" style={middle_area_style}>
                { render_region("sidebar_left") }
                <MainPanel>
                    <ContentRenderer content_type={state.main_content.clone()} />
                    { render_region("main_panel") }
                    { render_region("hud_top_left") }
                    { render_region("hud_top_right") }
                    { render_region("hud_bottom_left") }
                    { render_region("hud_bottom_right") }
                    { render_region("floating_toolbar") }
                </MainPanel>
                { render_region("sidebar_right") }
                { render_region("activity_bar") }
            </div>
            <StatusBar />
        </div>
    }
}