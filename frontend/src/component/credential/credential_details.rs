use crate::component::builder::display_credential::ClaimPropertyNode;
use crate::component::nav_bar::NavBar;
use crate::constants::INDEXEDDB_OBJECT_STORE_NAME;
use crate::util::get_indexeddb_connector;
use indexed_db_futures::prelude::*;
use vc_core::VerifiableCredential;
use wasm_bindgen::prelude::*;
use yew::{platform::spawn_local, prelude::*};

#[derive(Properties, PartialEq)]
pub struct CredentialDetailsProps {
    pub credential_id: String,
}

#[function_component(CredentialDetails)]
pub fn credential_details(props: &CredentialDetailsProps) -> Html {
    let credential = use_state(|| None);
    let credential_id = props.credential_id.clone();

    let fetch_credential = {
        let credential = credential.clone();
        Callback::from(move |_| {
            let credential = credential.clone();
            let credential_id = credential_id.clone();
            let future = async move {
                let db = get_indexeddb_connector()
                    .await
                    .expect("Could not open IndexedDB.");
                let tx: IdbTransaction = db
                    .transaction_on_one_with_mode(
                        INDEXEDDB_OBJECT_STORE_NAME,
                        IdbTransactionMode::Readwrite,
                    )
                    .expect("Could not create IndexedDB transaction.");
                let store: IdbObjectStore = tx
                    .object_store(INDEXEDDB_OBJECT_STORE_NAME)
                    .expect("Could not create IndexedDB object store.");
                let key = JsValue::from_str(&credential_id);
                let fetched_credential = store
                    .get(&key)
                    .expect("Could not get credential from IndexedDB store.")
                    .await
                    .expect("Could not get credential from IndexedDB store.");
                let fetched_credential = fetched_credential
                    .expect("Could not get credential from IndexedDB store.")
                    .as_string()
                    .expect("Could not convert credential to string.");
                let fetched_credential: VerifiableCredential =
                    serde_json::from_str(&fetched_credential)
                        .expect("Could not deserialize credential.");
                credential.set(Some(fetched_credential));
            };
            spawn_local(future);
        })
    };

    use_effect_with_deps(
        move |_| {
            fetch_credential.emit(());
            || ()
        },
        (),
    );

    let content = match (*credential).clone() {
        Some(credential) => {
            let proofs = credential.get_proof().clone();
            let credential = credential.get_credential();
            let claims = credential.get_credential_subject();
            html! {
                <div class="m-8">
                    <h2 class="text-xl font-bold">{"Credential: "}</h2>
                    <p class="text-gray-600">{"ID: "}{credential.get_id()}</p>
                    <p class="text-gray-600">{"Issuer: "}{credential.get_issuer()}</p>
                    <p class="text-gray-600">{"Valid From: "}{credential.get_valid_from()}</p>
                    <p class="text-gray-600">{"Valid Until: "}{credential.get_valid_until()}</p>
                    <div class="text-left">
                        {"{"}
                        {for claims.iter().map(|(key, value)| {
                            html! {
                                <div class="ml-4">
                                    {"\""} {key} {"\": "} <ClaimPropertyNode property={value.clone()} />
                                </div>
                            }
                        })}
                        {"}"}
                    </div>
                    <div>
                        <p class="text-l font-bold">{"Proofs: "}</p>
                        <div class="text-center">
                            {for proofs.iter().map(|proof| {
                                html! {
                                    <div class="border-md rounded bg-slate-50 m-2">
                                        <div>
                                            {"Type: "} {proof.get_type()}
                                        </div>
                                        <div>
                                            {"Created: "} {proof.get_created()}
                                        </div>
                                        <div>
                                            {"Proof Purpose: "} {proof.get_proof_purpose()}
                                        </div>
                                        <div>
                                            {"Verification Method Id: "} {proof.get_verification_method()}
                                        </div>
                                        <div>
                                            {"Proof Value: "} {hex::encode(proof.get_proof_value())}
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>
            }
        }
        None => html! {
            <div class="m-8">
                <div class="text-2xl font-bold">{"Loading..."}</div>
            </div>
        },
    };

    html! {
        <div class="m-8">
            <NavBar />
            {content}
        </div>
    }
}
