use log::Level;
use routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod component;
pub mod constants;
pub mod routes;
pub mod util;

#[function_component]
fn App() -> Html {
    html! { <BrowserRouter> <Switch<Route> render={routes::switch} /> </BrowserRouter> }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    yew::Renderer::<App>::new().render();
}
