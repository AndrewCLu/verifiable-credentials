use super::{
    display_credential::DisplayCredential, make_claims::MakeClaims, select_issuer::SelectIssuer,
    select_schema::SelectSchema,
};
use crate::component::nav_bar::NavBar;
use vc_core::Credential;
use yew::prelude::*;

#[function_component(CredentialHome)]
pub fn credential_home() -> Html {
    let issuer_id = use_state(|| None);
    let schema_id = use_state(|| None);
    let credential = use_state(|| None);
    let set_issuer_id = {
        let issuer_id = issuer_id.clone();
        Callback::from(move |id: String| {
            issuer_id.set(Some(id));
        })
    };
    let set_schema_id = {
        let schema_id = schema_id.clone();
        Callback::from(move |id: String| {
            schema_id.set(Some(id));
        })
    };
    let set_credential = {
        let credential = credential.clone();
        Callback::from(move |cred: Credential| {
            credential.set(Some(cred));
        })
    };

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
                    <SelectIssuer set_issuer_id={set_issuer_id} />
                }
            }}
        </div>
    }
}
