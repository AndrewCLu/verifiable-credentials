use crate::component::nav_bar::NavBar;
use crate::constants::BASE_URL;
use log::error;
use std::collections::HashMap;
use vc_core::{CredentialSchema, SchemaProperty, SchemaPropertyType, SchemaPropertyValue};
use yew::{platform::spawn_local, prelude::*};

async fn get_schema(schema_id: String) -> Result<CredentialSchema, reqwest::Error> {
    let url = format!("{}/schema/{}", BASE_URL, schema_id);
    let resp = reqwest::get(url).await?;
    let schema: CredentialSchema = resp.json().await?;
    Ok(schema)
}

#[derive(Properties, PartialEq)]
pub struct SchemaPropertyValueNodeProps {
    pub property: SchemaPropertyValue,
}

#[function_component(SchemaPropertyValueNode)]
pub fn schema_property_value_node(props: &SchemaPropertyValueNodeProps) -> Html {
    let description = props.property.get_description();
    match props.property.get_type() {
        SchemaPropertyType::Text => {
            html!(
                <>
                    {"(Text) "} {description}
                </>
            )
        }
        SchemaPropertyType::Number => {
            html!(
                <>
                    {"(Number) "} {description}
                </>
            )
        }
        SchemaPropertyType::Boolean => {
            html!(
                <>
                    {"(Boolean) "} {description}
                </>
            )
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SchemaPropertyNodeProps {
    pub property: SchemaProperty,
}

#[function_component(SchemaPropertyNode)]
pub fn schema_property_node(props: &SchemaPropertyNodeProps) -> Html {
    let property = props.property.clone();
    match property {
        SchemaProperty::Value(value) => {
            html! {
                <>
                <SchemaPropertyValueNode property={value} /> {","}
                </>
            }
        }
        SchemaProperty::Array(array) => {
            html! {
                <>
                    {"["}
                        <ul>
                            {for array.iter().map(|value| {
                                html! {
                                    <li class="ml-8">
                                        <SchemaPropertyNode property={value.clone()} />
                                    </li>
                                }
                            })}
                        </ul>
                    {"],"}
                </>
            }
        }
        SchemaProperty::Map(map) => {
            html! {
                <>
                {"{"}
                <div class="ml-8">
                    {for map.iter().map(|(key, value)| {
                        html! {
                            <div>
                                {"\""} {key} {"\": "}
                                <SchemaPropertyNode property={value.clone()} />
                            </div>
                        }
                    })}
                </div>
                {"},"}
                </>
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SchemaPropertiesProps {
    pub properties: HashMap<String, SchemaProperty>,
}

#[function_component(SchemaProperties)]
pub fn schema_properties(props: &SchemaPropertiesProps) -> Html {
    let properties = props.properties.clone();
    html! {
        <div>
            {"{"}
            {for properties.iter().map(|(key, value)| {
                html! {
                    <div class="ml-4">
                        {"\""} {key} {"\": "} <SchemaPropertyNode property={value.clone()} />
                    </div>
                }
            })}
            {"}"}
        </div>
    }
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
                            <p class="text-gray-300">{"ID: "}{schema.get_id()}</p>
                            <p class="text-gray-300">{"Description: "}{schema.get_description()}</p>
                            <div>{"Schema: "}</div>
                            <SchemaProperties properties={(*schema.get_properties()).clone()} />
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
