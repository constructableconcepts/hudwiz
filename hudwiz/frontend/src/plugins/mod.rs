pub mod clock;
pub mod system_stats;
pub mod video;
pub mod map;

use yew::prelude::*;

/// A trait for all plugins.
pub trait Plugin: std::fmt::Debug {
    /// The unique identifier for the plugin.
    fn id(&self) -> &'static str;

    /// The display name of the plugin.
    fn name(&self) -> &'static str;

    /// The component to be rendered for this plugin.
    fn view(&self) -> Html;
}

/// A registry for all available plugins.
#[derive(Debug)]
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    /// Creates a new plugin registry.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Registers a new plugin.
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    /// Registers a plugin by its string identifier.
    pub fn register_by_name(&mut self, name: &str) {
        match name {
            "Clock" => self.register(clock::ClockPlugin),
            "SystemStats" => self.register(system_stats::SystemStatsPlugin),
            "Video" => self.register(video::VideoPlugin),
            "Map" => self.register(map::MapPlugin),
            _ => log::warn!("Attempted to register unknown plugin: {}", name),
        }
    }

    /// Returns a list of all registered plugins.
    pub fn get_all(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for PluginRegistry {
    fn eq(&self, other: &Self) -> bool {
        let self_ids: Vec<_> = self.plugins.iter().map(|p| p.id()).collect();
        let other_ids: Vec<_> = other.plugins.iter().map(|p| p.id()).collect();
        self_ids == other_ids
    }
}
