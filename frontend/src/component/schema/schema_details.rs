use crate::component::nav_bar::NavBar;
use crate::constants::BASE_URL;
use log::error;
use vc_core::CredentialSchema;
use yew::{platform::spawn_local, prelude::*};

async fn get_schema(schema_id: String) -> Result<CredentialSchema, reqwest::Error> {
    let url = format!("{}/schema/{}", BASE_URL, schema_id);
    let resp = reqwest::get(url).await?;
    let schema: CredentialSchema = resp.json().await?;
    Ok(schema)
}

#[derive(Properties, PartialEq)]
pub struct SchemaDetailsProps {
    pub schema_id: String,
}

#[function_component(SchemaDetails)]
pub fn schema_details(props: &SchemaDetailsProps) -> Html {
    let schema = use_state(|| None);
    let schema_id = props.schema_id.clone();

    let schema_clone = schema.clone();
    use_effect_with_deps(
        move |schema_id| {
            let schema_id = schema_id.clone();
            let future = async move {
                match get_schema(schema_id.clone()).await {
                    Ok(fetched_schema) => {
                        schema_clone.set(Some(fetched_schema));
                    }
                    Err(_) => {
                        error!("Failed to fetch schema {}.", schema_id);
                    }
                }
            };
            spawn_local(future);
            || ()
        },
        schema_id,
    );

    match (*schema).clone() {
        Some(schema) => {
            html! {
                <div class="m-8">
                    <NavBar />
                        <div class="p-4 border border-gray-200">
                            <h2 class="text-xl font-bold">{schema.get_name()}</h2>
                            <p class="text-gray-600">{"ID: "}{schema.get_id()}</p>
                            <p class="text-gray-300">{"ID: "}{schema.get_description()}</p>
                        </div>
                </div>
            }
        }
        None => {
            html! {
                <div class="m-8">
                    <NavBar />
                    <div>
                    {"Unable to fetch schema."}
                    </div>
                </div>
            }
        }
    }
}
