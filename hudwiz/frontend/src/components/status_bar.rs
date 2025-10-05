use yew::prelude::*;
use material_yew::Icon;

#[function_component(StatusBar)]
pub fn status_bar() -> Html {
    html! {
        <div id="status-bar" class="status-bar">
            <div class="status-bar-left-1 toolbar-group">
                <div><Icon icon={{"build"}} /></div>
                <div><Icon icon={{"science"}} /></div>
            </div>
            <div class="status-bar-left-2 toolbar-group">
                <div><Icon icon={"build"} /></div>
                <div><Icon icon={"bug_report"} /></div>
            </div>
            <div class="status-bar-center toolbar-group">
                <div><Icon icon={{"history"}} /></div>
                <span>{"System Status: OK"}</span>
            </div>
            <div class="status-bar-right-1 toolbar-group">
                <div><Icon icon={{"view_quilt"}} /></div>
                <span>{"Ln 1, Col 1"}</span>
                <span>{"UTF-8"}</span>
                <span>{"v0.0.1"}</span>
            </div>
            <div class="status-bar-right-2 toolbar-group">
                <div><Icon icon={{"notifications"}} /></div>
                <div><Icon icon={{"account_circle"}} /></div>
            </div>
        </div>
    }
}
