use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::services::state_service::{AppContext, AppAction};

#[function_component(ChatInputBar)]
pub fn chat_input_bar() -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let input_state = use_state(String::new);

    let on_input = {
        let input_state = input_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_state.set(input.value());
        })
    };

    let on_send = {
        let dispatch = app_context.reducer.dispatcher();
        let input_state = input_state.clone();
        Callback::from(move |_: MouseEvent| {
            let message = (*input_state).clone();
            if !message.is_empty() {
                dispatch.dispatch(AppAction::SendChatMessage(message));
                input_state.set(String::new());
            }
        })
    };

    let on_keypress = {
        let dispatch = app_context.reducer.dispatcher();
        let input_state = input_state.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let message = (*input_state).clone();
                if !message.is_empty() {
                    dispatch.dispatch(AppAction::SendChatMessage(message));
                    input_state.set(String::new());
                }
            }
        })
    };

    html! {
        <div class="chat-input-bar">
            <input
                type="text"
                class="chat-input"
                placeholder="Type a message..."
                value={(*input_state).clone()}
                oninput={on_input}
                onkeypress={on_keypress}
            />
            <button onclick={on_send} class="chat-send-button">
                { "Send" }
            </button>
        </div>
    }
}