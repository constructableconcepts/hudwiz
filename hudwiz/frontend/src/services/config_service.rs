use anyhow::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::functional::UseReducerDispatcher;
use crate::services::state_service::{AppAction, AppState};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub layout: LayoutConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LayoutConfig {
    #[serde(flatten)]
    pub regions: HashMap<String, RegionConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RegionConfig {
    pub component: ComponentType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentType {
    ControlsDrawer,
    DetailsDrawer,
    UserDrawer,
    DrawingToolbar,
    TopLeftHud,
    TopRightHud,
    BottomLeftHud,
    BottomRightHud,
    BentoGrid,
    ChatInputBar,
    ChatHistoryView,
}

pub fn load_config(dispatch: UseReducerDispatcher<AppState>) {
    spawn_local(async move {
        match fetch_config().await {
            Ok(config) => {
                dispatch.dispatch(AppAction::SetConfig(config));
            }
            Err(e) => {
                log::error!("Failed to load config: {}", e);
            }
        }
    });
}

async fn fetch_config() -> Result<Config> {
    let config: Config = Request::get("/static/config.json")
        .send()
        .await?
        .json()
        .await?;
    Ok(config)
}