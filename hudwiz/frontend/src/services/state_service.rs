use yew::prelude::*;
use std::rc::Rc;
use yew::functional::Reducible;
use crate::services::config_service::Config;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    TransparentSvg,
    IFrame(String),
    Video(String),
    WebGL(String),
    Empty,
    AnimatedSvg,
    SolidColor(String),
    Image(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Author {
    User,
    Bot,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: usize,
    pub author: Author,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

impl Default for Conversation {
    fn default() -> Self {
        Self {
            messages: vec![],
        }
    }
}

// Represents the state of the application.
#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub theme: String,
    pub title: String,
    pub selected_pattern_title: String,
    pub background_pattern: String,
    pub sidebar_left_collapsed: bool,
    pub sidebar_right_collapsed: bool,
    pub hud_top_left_collapsed: bool,
    pub hud_top_right_collapsed: bool,
    pub hud_bottom_left_collapsed: bool,
    pub hud_bottom_right_collapsed: bool,
    pub activity_bar_collapsed: bool,
    pub background_content: ContentType,
    pub main_content: ContentType,
    pub config: Option<Config>,
    pub conversation: Conversation,
    pub language: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            title: "hudwiz".to_string(),
            selected_pattern_title: "hudwiz".to_string(),
            background_pattern: "default".to_string(),
            sidebar_left_collapsed: true,
            sidebar_right_collapsed: true,
            hud_top_left_collapsed: true,
            hud_top_right_collapsed: true,
            hud_bottom_left_collapsed: true,
            hud_bottom_right_collapsed: true,
            activity_bar_collapsed: false,
            background_content: ContentType::AnimatedSvg,
            main_content: ContentType::Image("/static/images/background_content.jpg".to_string()),
            config: None,
            conversation: Conversation::default(),
            language: "en-US".to_string(),
        }
    }
}

// Actions that can be dispatched to update the state.
pub enum AppAction {
    SetTheme(String),
    SetTitle(String),
    SetSelectedPatternTitle(String),
    SetBackgroundPattern(String),
    ToggleSidebarLeft,
    ToggleSidebarRight,
    ToggleHudTopLeft,
    ToggleHudTopRight,
    ToggleHudBottomLeft,
    ToggleHudBottomRight,
    ToggleActivityBar,
    SetBackgroundContent(ContentType),
    SetMainContent(ContentType),
    SetConfig(Config),
    SendChatMessage(String),
    SetLanguage(String),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        match action {
            AppAction::SetTheme(theme) => next_state.theme = theme,
            AppAction::SetTitle(title) => next_state.title = title,
            AppAction::SetSelectedPatternTitle(title) => next_state.selected_pattern_title = title,
            AppAction::SetBackgroundPattern(pattern) => next_state.background_pattern = pattern,
            AppAction::ToggleSidebarLeft => next_state.sidebar_left_collapsed = !next_state.sidebar_left_collapsed,
            AppAction::ToggleSidebarRight => next_state.sidebar_right_collapsed = !next_state.sidebar_right_collapsed,
            AppAction::ToggleHudTopLeft => next_state.hud_top_left_collapsed = !next_state.hud_top_left_collapsed,
            AppAction::ToggleHudTopRight => next_state.hud_top_right_collapsed = !next_state.hud_top_right_collapsed,
            AppAction::ToggleHudBottomLeft => next_state.hud_bottom_left_collapsed = !next_state.hud_bottom_left_collapsed,
            AppAction::ToggleHudBottomRight => next_state.hud_bottom_right_collapsed = !next_state.hud_bottom_right_collapsed,
            AppAction::ToggleActivityBar => next_state.activity_bar_collapsed = !next_state.activity_bar_collapsed,
            AppAction::SetBackgroundContent(content) => next_state.background_content = content,
            AppAction::SetMainContent(content) => next_state.main_content = content,
            AppAction::SetConfig(config) => next_state.config = Some(config),
            AppAction::SendChatMessage(text) => {
                let new_message = Message {
                    id: next_state.conversation.messages.len(),
                    author: Author::User,
                    text,
                };
                next_state.conversation.messages.push(new_message);
            }
            AppAction::SetLanguage(lang) => next_state.language = lang,
        }
        next_state.into()
    }
}

// The context that will be provided to components.
#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub reducer: UseReducerHandle<AppState>,
}

#[derive(Properties, PartialEq)]
pub struct AppStateProviderProps {
    #[prop_or_default]
    pub children: Children,
}

use crate::services::config_service;

use yew::functional::UseReducerHandle;

#[function_component(AppStateProvider)]
pub fn app_state_provider(props: &AppStateProviderProps) -> Html {
    let reducer = use_reducer(AppState::default);

    {
        let dispatch = reducer.dispatcher();
        use_effect_with((), move |_| {
            config_service::load_config(dispatch);
            || ()
        });
    }

    let app_context = AppContext {
        reducer,
    };

    html! {
        <ContextProvider<AppContext> context={app_context}>
            { for props.children.iter() }
        </ContextProvider<AppContext>>
    }
}
