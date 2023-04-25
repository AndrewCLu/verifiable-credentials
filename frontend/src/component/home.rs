use super::issuer_home::IssuerHome;
use yew::{function_component, html, Html};

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="m-8">
            <IssuerHome />
        </div>
    }
}
