use yew::prelude::*;
use crate::plugins::Plugin;

#[derive(Debug)]
pub struct SystemStatsPlugin;

impl Plugin for SystemStatsPlugin {
    fn id(&self) -> &'static str {
        "system-stats"
    }

    fn name(&self) -> &'static str {
        "System Stats"
    }

    fn view(&self) -> Html {
        html! { <SystemStats /> }
    }
}

#[function_component(SystemStats)]
fn system_stats() -> Html {
    html! {
        <div>
            <p>{ "CPU: 42%" }</p>
            <p>{ "Memory: 64%" }</p>
            <p>{ "Disk: 87%" }</p>
        </div>
    }
}
