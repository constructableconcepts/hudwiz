use yew::prelude::*;
use crate::plugins::Plugin;
use js_sys::Date;

#[derive(Debug)]
pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn id(&self) -> &'static str {
        "clock"
    }

    fn name(&self) -> &'static str {
        "Clock"
    }

    fn view(&self) -> Html {
        html! { <Clock /> }
    }
}

#[function_component(Clock)]
fn clock() -> Html {
    let time = use_state(|| get_current_time());

    {
        let time = time.clone();
        use_effect(move || {
            let handle = gloo_timers::callback::Interval::new(1000, move || {
                time.set(get_current_time());
            });
            move || drop(handle)
        });
    }

    html! {
        <div>
            { &*time }
        </div>
    }
}

fn get_current_time() -> String {
    let date = Date::new_0();
    date.to_locale_time_string("en-US").into()
}
