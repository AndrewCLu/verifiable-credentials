use log::Level;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod component;
pub mod constants;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/issuer")]
    Issuer,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <component::home::Home /> },
        Route::Issuer => html! { <component::issuer_home::IssuerHome /> },
        Route::NotFound => html! { <component::not_found::NotFound /> },
    }
}

#[function_component]
fn App() -> Html {
    html! { <BrowserRouter> <Switch<Route> render={switch} /> </BrowserRouter> }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    yew::Renderer::<App>::new().render();
}
