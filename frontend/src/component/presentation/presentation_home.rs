use super::{
    display_result::DisplayResult, presentation_builder::PresentationBuilder,
    select_credential::SelectCredential, select_verifier::SelectVerifier,
};
use crate::component::nav_bar::NavBar;
use vc_core::{VerifiableCredential, Verifier};
use yew::prelude::*;

#[function_component(PresentationHome)]
pub fn presentation_home() -> Html {
    let verifier = use_state(|| None);
    let credential = use_state(|| None);
    let verified = use_state(|| false);
    let set_verifier = {
        let verifier_clone = verifier.clone();
        Callback::from(move |verifier: Option<Verifier>| {
            verifier_clone.set(verifier);
        })
    };
    let set_credential = {
        let credential_clone = credential.clone();
        Callback::from(move |credential: Option<VerifiableCredential>| {
            credential_clone.set(credential);
        })
    };
    let set_verified = {
        let verified_clone = verified.clone();
        Callback::from(move |verified: bool| {
            verified_clone.set(verified);
        })
    };

    html! {
        <div class="m-8">
        <NavBar />
        <div />
            {html! {
                if *verified {
                    <DisplayResult verifier={(*verifier.as_ref().unwrap()).clone()} credential={(*credential.as_ref().unwrap()).clone()} set_verifier={set_verifier} set_credential={set_credential} set_verified={set_verified} />
                } else if verifier.is_some() && credential.is_some() {
                    <PresentationBuilder verifier={(*verifier.as_ref().unwrap()).clone()} credential={(*credential.as_ref().unwrap()).clone()} set_verified={set_verified} set_credential={set_credential} />
                } else if verifier.is_some() {
                    <SelectCredential verifier={(*verifier.as_ref().unwrap()).clone()} set_credential={set_credential} set_verifier={set_verifier} />
                } else {
                    <SelectVerifier set_verifier={set_verifier} />
                }
            }}
        </div>
    }
}
