use super::nav_bar::NavBar;
use yew::{function_component, html, Html};

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="m-8">
            <NavBar />
            <div>
            {"This is the home page."}
            </div>
        </div>
    }
}
