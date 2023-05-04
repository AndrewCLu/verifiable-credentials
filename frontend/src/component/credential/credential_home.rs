use super::credential_list::CredentialList;
use crate::component::nav_bar::NavBar;
use yew::prelude::*;

#[function_component(CredentialHome)]
pub fn credential_home() -> Html {
    html! {
        <div class="m-8">
            <NavBar />
            <CredentialList />
        </div>
    }
}
