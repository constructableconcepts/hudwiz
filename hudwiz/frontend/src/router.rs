use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::main_page::MainPage;
use crate::pages::settings_page::SettingsPage;
use crate::pages::login_page::LoginPage;
use crate::pages::signup_page::SignupPage;
use crate::pages::landing_page::LandingPage;
use crate::components::private_route::PrivateRoute;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/app")]
    App,
    #[at("/settings")]
    Settings,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::App => html! { <PrivateRoute><MainPage /></PrivateRoute> },
        Route::Settings => html! { <PrivateRoute><SettingsPage /></PrivateRoute> },
        Route::Login => html! { <LoginPage /> },
        Route::Signup => html! { <SignupPage /> },
    }
}
