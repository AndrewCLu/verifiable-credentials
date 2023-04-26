use crate::constants::BASE_URL;
use log::{debug, error};
use serde_json::json;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, PartialEq, Properties)]
pub struct AddIssuerProps {
    pub fetch_issuers: Callback<()>,
}

#[function_component]
pub fn AddIssuer(props: &AddIssuerProps) -> Html {
    let name = use_state(|| "".to_string());
    let fetch_issuers = props.fetch_issuers.clone();
    let client = reqwest::Client::new();

    let handle_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                name.set(input.value());
            }
        })
    };

    let on_submit = {
        let name = name.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.clone();
            let fetch_issuers = fetch_issuers.clone();
            let client = client.clone();
            let request_data = json!({
                "id": Uuid::new_v4().to_string(),
                "name": *name,
            });
            let future = async move {
                let url = format!("{}/issuer/add_issuer", BASE_URL);
                let resp = client.post(url).json(&request_data).send().await;
                match resp {
                    Ok(resp) => {
                        debug!("Received response: {:?}", resp);
                        fetch_issuers.emit(());
                    }
                    Err(e) => {
                        error!("Error creating new issuer: {:?}", e);
                    }
                }
                name.set("".to_string());
            };
            spawn_local(future);
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <input
                type="text"
                placeholder="Issuer Name"
                value={(*name).clone()}
                oninput={handle_input}
            />
            <button type="submit">{"Submit"}</button>
        </form>
    }
}
