use vc_core::{VerifiableCredential, Verifier};
use yew::prelude::*;

use super::presentation_builder::VerifyCredentialResults;

#[derive(Properties, PartialEq)]
pub struct DisplayResultProps {
    pub verifier: Verifier,
    pub credential: VerifiableCredential,
    pub verified: VerifyCredentialResults,
    pub set_verifier: Callback<Option<Verifier>>,
    pub set_credential: Callback<Option<VerifiableCredential>>,
    pub set_verified: Callback<Option<VerifyCredentialResults>>,
}

#[function_component(DisplayResult)]
pub fn display_result(props: &DisplayResultProps) -> Html {
    html! {
        <div>
            {"I am displaying results"}
        </div>
    }
}
