use crate::constants::BASE_URL;
use log::{debug, error};
use serde_json::json;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, PartialEq, Properties)]
pub struct AddSchemaProps {
    pub fetch_schemas: Callback<()>,
}

#[function_component]
pub fn AddSchema(props: &AddSchemaProps) -> Html {
    let name = use_state(|| "".to_string());
    let fetch_schemas = props.fetch_schemas.clone();
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
            let fetch_schemas = fetch_schemas.clone();
            let client = client.clone();
            let request_data = json!({
                "id": Uuid::new_v4().to_string(),
                "name": *name,
            });
            let future = async move {
                let url = format!("{}/schema/add_schema", BASE_URL);
                let resp = client.post(url).json(&request_data).send().await;
                match resp {
                    Ok(resp) => {
                        debug!("Received response: {:?}", resp);
                        fetch_schemas.emit(());
                    }
                    Err(e) => {
                        error!("Error creating new schema: {:?}", e);
                    }
                }
                name.set("".to_string());
            };
            spawn_local(future);
        })
    };

    html! {
        <div class="m-8 text-center">
        <h1 class="text-3xl mb-2">{"Add Schema"}</h1>
        <form onsubmit={on_submit}>
            <input
                class="border-slate-300 border-2 rounded p-2 mr-2"
                type="text"
                placeholder="Schema Name"
                value={(*name).clone()}
                oninput={handle_input}
            />
            <button class="rounded bg-stone-200 p-2" type="submit">{"Submit"}</button>
        </form>
        </div>
    }
}
