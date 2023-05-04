use crate::constants::INDEXEDDB_OBJECT_STORE_NAME;
use crate::util::get_indexeddb_connector;
use indexed_db_futures::{js_sys::Array, prelude::*};
use std::rc::Rc;
use vc_core::VerifiableCredential;
use wasm_bindgen::prelude::*;
use yew::{platform::spawn_local, prelude::*};

#[hook]
pub fn use_credentials() -> (Rc<Vec<VerifiableCredential>>, bool, Callback<()>) {
    let credentials = use_state(|| Rc::new(Vec::<VerifiableCredential>::new()));
    let loading = use_state(|| true);

    let fetch_credentials = {
        let (credentials_clone, loading_clone) = (credentials.clone(), loading.clone());
        Callback::from(move |_| {
            let credentials = credentials_clone.clone();
            let loading = loading_clone.clone();
            loading.set(true);
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
                let credentials_array: Array = store
                    .get_all()
                    .expect("Could not get all credentials from IndexedDB store.")
                    .await
                    .expect("Could not get all credentials from IndexedDB store.");
                let mut fetched_credentials = Vec::new();
                for i in 0..credentials_array.length() {
                    let credential: JsValue = credentials_array.get(i);
                    let credential: String = credential
                        .as_string()
                        .expect("Could not convert credential to string.");
                    let credential: VerifiableCredential = serde_json::from_str(&credential)
                        .expect("Could not deserialize credential.");
                    fetched_credentials.push(credential);
                }
                credentials.set(Rc::new(fetched_credentials));
                loading.set(false);
            };
            spawn_local(future);
        })
    };

    let fetch_credentials_clone = fetch_credentials.clone();
    use_effect_with_deps(
        move |_| {
            fetch_credentials_clone.emit(());
            || ()
        },
        (),
    );

    (Rc::clone(&credentials), *loading, fetch_credentials)
}

#[function_component(CredentialList)]
pub fn credential_list() -> Html {
    let (credentials, loading, _fetch_credentials) = use_credentials();
    let credential_list = credentials
        .iter()
        .map(|credential| {
            let credential = credential.get_credential();
            html! {
                <div class="p-4 border border-gray-200">
                    <h2 class="text-xl font-bold">{"Credential: "}</h2>
                    <p class="text-gray-600">{"ID: "}{credential.get_id()}</p>
                    <p class="text-gray-600">{"Issuer: "}{credential.get_issuer()}</p>
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
            <h1 class="text-3xl text-center mb-2">{"All Credentials"}</h1>
            {content}
        </div>
    }
}
