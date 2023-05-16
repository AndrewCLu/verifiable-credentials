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
    let VerifyCredentialResults { verified, reason } = &props.verified;
    let set_verifier = props.set_verifier.clone();
    let set_credential = props.set_credential.clone();
    let set_verified = props.set_verified.clone();

    let reset_verification = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        set_verifier.emit(None);
        set_credential.emit(None);
        set_verified.emit(None);
    });

    let content = if *verified {
        html! {
            <div class="p-4 border border-gray-200 m-8">
                <div>{"Credential Verified!"}</div>
            </div>
        }
    } else {
        html! {
            <div class="p-4 border border-gray-200 m-8">
                <div>{"Credential Not Verified"}</div>
                <div>{reason}</div>
            </div>
        }
    };

    html! {
        <div class="m-8">
            {content}
            <div class="text-center mt-2">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={reset_verification}>
                    {"Verify Another Credential"}
                </button>
            </div>
        </div>
    }
}
