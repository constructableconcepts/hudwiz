use yew::prelude::*;
use yew_router::prelude::*;
use crate::services::auth_service::AuthContext;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct PrivateRouteProps {
    pub children: Children,
}

#[function_component(PrivateRoute)]
pub fn private_route(props: &PrivateRouteProps) -> Html {
    let auth_context = use_context::<AuthContext>().expect("no auth context found");

    if auth_context.user.is_some() {
        html! { for props.children.iter() }
    } else {
        html! { <Redirect<Route> to={Route::Login}/> }
    }
}
