use crate::constants::BASE_URL;
use log::{debug, error};
use serde_json::json;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, PartialEq, Properties)]
pub struct AddVerificationMethodProps {
    pub issuer_id: String,
    pub fetch_issuer: Callback<()>,
}

#[function_component]
pub fn AddVerificationMethod(props: &AddVerificationMethodProps) -> Html {
    let type_ = use_state(|| "MyK256VerificationMethod".to_string());
    let issuer_id = props.issuer_id.clone();
    let fetch_issuer = props.fetch_issuer.clone();
    let client = reqwest::Client::new();

    let handle_type_input = {
        let type_ = type_.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                type_.set(input.value());
            }
        })
    };

    let on_submit = {
        let type_ = type_.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let type_ = type_.clone();
            let issuer_id = issuer_id.clone();
            let fetch_issuer = fetch_issuer.clone();
            let client = client.clone();
            let request_data = json!({
                "verification_method_id": Uuid::new_v4().to_string(),
                "type_": *type_,
            });
            let future = async move {
                let url = format!("{}/issuer/{}/verification_method", BASE_URL, issuer_id);
                let resp = client.post(url).json(&request_data).send().await;
                match resp {
                    Ok(resp) => {
                        debug!("Response from adding new verification method: {:?}", resp);
                        fetch_issuer.emit(());
                    }
                    Err(e) => {
                        error!("Error creating new verification method: {:?}", e);
                    }
                }
                type_.set("".to_string());
            };
            spawn_local(future);
        })
    };

    html! {
        <div class="m-8 text-center">
        <h1 class="text-3xl mb-2">{"Add Verification Method"}</h1>
        <form onsubmit={on_submit}>
            <input
                class="border-slate-300 border-2 rounded p-2 mr-2"
                type="text"
                placeholder="Verification Method Type"
                value={(*type_).clone()}
                oninput={handle_type_input}
            />
            <button class="rounded bg-stone-200 p-2" type="submit">{"Submit"}</button>
        </form>
        </div>
    }
}
