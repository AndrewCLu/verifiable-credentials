use crate::constants::BASE_URL;
use log::{debug, error};
use serde_json::json;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, PartialEq, Properties)]
pub struct AddVerifierProps {
    pub fetch_verifiers: Callback<()>,
}

#[function_component]
pub fn AddVerifier(props: &AddVerifierProps) -> Html {
    let name = use_state(|| "".to_string());
    let schema_id = use_state(|| "".to_string());
    let fetch_verifiers = props.fetch_verifiers.clone();
    let client = reqwest::Client::new();

    let handle_name_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                name.set(input.value());
            }
        })
    };

    let handle_schema_id_input = {
        let schema_id = schema_id.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                schema_id.set(input.value());
            }
        })
    };

    let on_submit = {
        let name = name.clone();
        let schema_id = schema_id.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.clone();
            let schema_id = schema_id.clone();
            let fetch_verifiers = fetch_verifiers.clone();
            let client = client.clone();
            let request_data = json!({
                "id": Uuid::new_v4().to_string(),
                "name": *name,
                "schema_id": *schema_id,
            });
            let future = async move {
                let url = format!("{}/verifier/", BASE_URL);
                let resp = client.post(url).json(&request_data).send().await;
                match resp {
                    Ok(resp) => {
                        debug!("Response from adding new verifier: {:?}", resp);
                        fetch_verifiers.emit(());
                    }
                    Err(e) => {
                        error!("Error creating new verifier: {:?}", e);
                    }
                }
                name.set("".to_string());
                schema_id.set("".to_string());
            };
            spawn_local(future);
        })
    };

    html! {
        <div class="m-8 text-center">
        <h1 class="text-3xl mb-2">{"Add Verifier"}</h1>
        <form onsubmit={on_submit}>
            <input
                class="border-slate-300 border-2 rounded p-2 mr-2"
                type="text"
                placeholder="Name"
                value={(*name).clone()}
                oninput={handle_name_input}
            />
            <input
                class="border-slate-300 border-2 rounded p-2 mr-2"
                type="text"
                placeholder="Schema ID"
                value={(*schema_id).clone()}
                oninput={handle_schema_id_input}
            />
            <button class="rounded bg-stone-200 p-2" type="submit">{"Submit"}</button>
        </form>
        </div>
    }
}
