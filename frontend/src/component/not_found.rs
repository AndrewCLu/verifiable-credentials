use yew::{function_component, html, Html};

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="m-8">
            {"The page you requested cannot be found."}
        </div>
    }
}
