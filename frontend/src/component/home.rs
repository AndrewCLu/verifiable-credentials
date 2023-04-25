use super::add_issuer::AddIssuer;
use super::issuer::IssuerList;
use yew::{function_component, html, Html};

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="m-8">
            <IssuerList />
            <div />
            <AddIssuer />
        </div>
    }
}
