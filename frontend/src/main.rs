use crate::component::home::Home;
use yew::prelude::*;

pub mod component;

#[function_component]
fn App() -> Html {
    html! { <Home /> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
