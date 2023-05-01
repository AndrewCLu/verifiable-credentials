use super::{
    credential_builder::CredentialBuilder, display_credential::DisplayCredential,
    select_issuer::SelectIssuer, select_schema::SelectSchema,
};
use crate::component::nav_bar::NavBar;
use vc_core::{CredentialSchema, Issuer, VerifiableCredential};
use yew::prelude::*;

#[function_component(CredentialHome)]
pub fn credential_home() -> Html {
    let issuer = use_state(|| None);
    let schema = use_state(|| None);
    let credential = use_state(|| None);
    let set_issuer = {
        let issuer_clone = issuer.clone();
        Callback::from(move |issuer: Option<Issuer>| {
            issuer_clone.set(issuer);
        })
    };
    let set_schema = {
        let schema_clone = schema.clone();
        Callback::from(move |schema: Option<CredentialSchema>| {
            schema_clone.set(schema);
        })
    };
    let set_credential = {
        let credential_clone = credential.clone();
        Callback::from(move |credential: Option<VerifiableCredential>| {
            credential_clone.set(credential);
        })
    };

    html! {
        <div class="m-8">
        <NavBar />
        <div />
            {html! {
                if credential.is_some() {
                    <DisplayCredential credential={(*credential.as_ref().unwrap()).clone()} />
                } else if issuer.is_some() && schema.is_some() {
                    <CredentialBuilder issuer={(*issuer.as_ref().unwrap()).clone()} schema={(*schema.as_ref().unwrap()).clone()} set_credential={set_credential} set_schema={set_schema} />
                } else if issuer.is_some() {
                    <SelectSchema issuer={(*issuer.as_ref().unwrap()).clone()} set_schema={set_schema} set_issuer={set_issuer} />
                } else {
                    <SelectIssuer set_issuer={set_issuer} />
                }
            }}
        </div>
    }
}
