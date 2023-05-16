use crate::constants::BASE_URL;
use log::{debug, error};
use serde::Deserialize;
use serde_json::json;
use vc_core::{VerifiableCredential, Verifier};
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, PartialEq, Deserialize)]
pub struct VerifyCredentialResults {
    pub verified: bool,
    pub reason: String,
}

#[derive(Properties, PartialEq)]
pub struct PresentationBuilderProps {
    pub verifier: Verifier,
    pub credential: VerifiableCredential,
    pub set_verified: Callback<Option<VerifyCredentialResults>>,
    pub set_credential: Callback<Option<VerifiableCredential>>,
}

#[function_component(PresentationBuilder)]
pub fn presentation_builder(props: &PresentationBuilderProps) -> Html {
    let verifier = &props.verifier;
    let verifier_id = verifier.get_id().clone();
    let verifiable_credential = props.credential.clone();
    let set_credential = props.set_credential.clone();
    let set_verified = props.set_verified.clone();

    let verifiable_credential_clone = verifiable_credential.clone();
    let submit_credential = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        let client = reqwest::Client::new();
        let verifier_id = verifier_id.clone();
        let verifier_id_str = verifier_id.get_str();
        let verifiable_credential = verifiable_credential_clone.clone();
        let verifiable_credential_str = serde_json::to_string(&verifiable_credential)
            .expect("Could not serialize verifiable credential.");
        let set_verified = set_verified.clone();
        let request_data = json!({
            "verifier_id": verifier_id_str,
            "verifiable_credential": verifiable_credential_str,
        });
        let future = async move {
            let url = format!("{}/verifier/verify", BASE_URL);
            let resp = client.post(url).json(&request_data).send().await;
            match resp {
                Ok(resp) => {
                    debug!("Response from verifying credential: {:?}", resp);
                    match resp.json::<VerifyCredentialResults>().await {
                        Ok(results) => {
                            set_verified.emit(Some(results));
                        }
                        Err(e) => {
                            error!("Error parsing response from verifying credential: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error verifying credential: {:?}", e);
                }
            }
        };
        spawn_local(future);
    });

    let credential = verifiable_credential.get_credential();
    html! {
        <div>
            <div class="p-4 border border-gray-200 mb-2">
                <div>{"Verifier: "}</div>
                <h2 class="text-xl font-bold">{verifier.get_name()}</h2>
                <p class="text-gray-600">{"ID: "}{verifier.get_id()}</p>
            </div>
            <div class="p-4 border border-gray-200 mb-2">
                <div>{"Credential: "}</div>
                <h2 class="text-xl font-bold">{credential.get_id()}</h2>
                <p class="text-gray-600">{"Issuer: "}{credential.get_issuer()}</p>
            </div>
            <h1 class="text-3xl text-center mb-2">{"Confirm Verifier and Credential"}</h1>
            <div class="text-center mt-2">
                <button class="bg-slate-500 hover:bg-slate-700 text-white font-bold py-2 px-4 rounded" onclick={submit_credential}>
                    {"Submit"}
                </button>
            </div>
            <div class="text-center mt-2">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={move |_| set_credential.emit(None)}>
                    {"Back"}
                </button>
            </div>
        </div>
    }
}
