use super::{
    display_credential::DisplayCredential, make_claims::MakeClaims, select_issuer::SelectIssuer,
    select_schema::SelectSchema,
};
use crate::component::nav_bar::NavBar;
use yew::prelude::*;

#[function_component(CredentialHome)]
pub fn credential_home() -> Html {
    let issuer_id = use_state(|| None);
    let schema_id = use_state(|| None);
    let credential = use_state(|| None);

    html! {
        <div class="m-8">
        <NavBar />
        <div />
            {html! {
                if credential.is_some() {
                    <DisplayCredential />
                } else if issuer_id.is_some() && schema_id.is_some() {
                    <MakeClaims />
                } else if issuer_id.is_some() {
                    <SelectSchema />
                } else {
                    <SelectIssuer />
                }
            }}
        </div>
    }
}
