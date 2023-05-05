use crate::component::credential::credential_list::use_credentials;
use vc_core::{VerifiableCredential, Verifier};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectCredentialProps {
    pub verifier: Verifier,
    pub set_credential: Callback<Option<VerifiableCredential>>,
    pub set_verifier: Callback<Option<Verifier>>,
}

#[function_component(SelectCredential)]
pub fn select_credential(props: &SelectCredentialProps) -> Html {
    let (credentials, loading, _) = use_credentials();
    let verifier = props.verifier.clone();
    let set_credential = props.set_credential.clone();
    let set_verifier = props.set_verifier.clone();

    let credential_list = credentials
        .iter()
        .map(|verifiable_credential| {
            let credential_clone = verifiable_credential.clone();
            let set_credential = set_credential.clone();
            let credential = verifiable_credential.get_credential();
            html! {
                <div class="p-4 border border-gray-200">
                    <button onclick={move |_| set_credential.emit(Some(credential_clone.clone()))}>
                    <h2 class="text-xl font-bold">{"Credential: "}</h2>
                    <p class="text-gray-600">{"ID: "}{credential.get_id()}</p>
                    <p class="text-gray-600">{"Issuer: "}{credential.get_issuer()}</p>
                    </button>
                </div>
            }
        })
        .collect::<Html>();

    let content = if loading {
        html! { <p>{"Loading credentials..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{credential_list}</div> }
    };

    html! {
    <div class = "m-8">
        <div class="p-4 border border-gray-200 mb-2">
            <div>{"Verifier: "}</div>
            <h2 class="text-xl font-bold">{verifier.get_name()}</h2>
            <p class="text-gray-600">{"ID: "}{verifier.get_id()}</p>
        </div>
        <h1 class="text-3xl text-center mb-2">{"Select An Credential"}</h1>
        {content}
        <div class="text-center mt-2">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={move |_| set_verifier.emit(None)}>
                {"Back"}
            </button>
        </div>
    </div> }
}
