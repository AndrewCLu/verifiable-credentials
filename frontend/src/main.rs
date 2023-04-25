use crate::component::home::Home;
use log::Level;
use yew::prelude::*;

pub mod component;
pub mod constants;

#[function_component]
fn App() -> Html {
    html! { <Home /> }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    yew::Renderer::<App>::new().render();
}
