use log::debug;
use serde_json::json;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, PartialEq, Properties)]
pub struct AddIssuerProps {
    pub fetch_issuers: Rc<Callback<()>>,
}

#[function_component]
pub fn AddIssuer(props: &AddIssuerProps) -> Html {
    let name = use_state(|| "".to_string());
    let fetch_issuers = props.fetch_issuers.clone();

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
            let fetch_issuers_clone = fetch_issuers.clone();
            let request_data = json!({
                "id": Uuid::new_v4().to_string(),
                "name": *name,
            });
            spawn_local(async move {
                let client = reqwest::Client::new();
                let resp = client
                    .post("http://127.0.0.1:8000/issuer/add_issuer")
                    .json(&request_data)
                    .send()
                    .await;
                match resp {
                    Ok(resp) => {
                        debug!("Received response: {:?}", resp);
                        fetch_issuers_clone.emit(());
                    }
                    Err(e) => {
                        debug!("Reqwest error: {:?}", e);
                    }
                }
                name.set("".to_string());
            });
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
