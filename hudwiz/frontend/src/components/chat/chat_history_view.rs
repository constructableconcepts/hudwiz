use yew::prelude::*;
use crate::services::state_service::{AppContext, Author};

#[function_component(ChatHistoryView)]
pub fn chat_history_view() -> Html {
    let app_context = use_context::<AppContext>().expect("no app context found");
    let conversation = &app_context.reducer.conversation;

    html! {
        <div class="chat-history-view">
            { for conversation.messages.iter().map(|message| {
                let message_class = if message.author == Author::User {
                    "chat-message user-message"
                } else {
                    "chat-message bot-message"
                };
                html! {
                    <div class={message_class}>
                        <div class="message-bubble">
                            <p>{ &message.text }</p>
                        </div>
                    </div>
                }
            }) }
        </div>
    }
}