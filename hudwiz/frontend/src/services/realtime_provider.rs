use super::realtime_service::RealtimeService;
use std::rc::Rc;
use yew::prelude::*;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct RealtimeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(RealtimeProvider)]
pub fn realtime_provider(props: &RealtimeProviderProps) -> Html {
    let realtime_service = use_state(|| None);
    {
        let realtime_service = realtime_service.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let service = RealtimeService::new("ws://127.0.0.1:8080/ws").await;
                match service {
                    Ok(s) => {
                        // Set the DOM attribute
                        if let Some(window) = window() {
                            if let Some(document) = window.document() {
                                if let Some(body) = document.body() {
                                    let transport_str = format!("{:?}", s.active_transport).to_lowercase();
                                    let _ = body.set_attribute("data-transport", &transport_str);
                                }
                            }
                        }
                        realtime_service.set(Some(Rc::new(s)));
                    },
                    Err(e) => log::error!("Failed to create RealtimeService: {:?}", e),
                }
            });
            || ()
        });
    }

    if let Some(service) = &*realtime_service {
        html! {
            <ContextProvider<Rc<RealtimeService>> context={service.clone()}>
                { for props.children.iter() }
            </ContextProvider<Rc<RealtimeService>>>
        }
    } else {
        html! { for props.children.iter() }
    }
}