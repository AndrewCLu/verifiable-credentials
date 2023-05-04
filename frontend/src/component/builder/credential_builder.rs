use super::claim_builder::ClaimBuilder;
use crate::constants::BASE_URL;
use chrono::Utc;
use log::{debug, error};
use serde_json::json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use vc_core::{
    ClaimProperty, ClaimPropertyValue, CredentialSchema, Issuer, SchemaProperty,
    SchemaPropertyValueType, VerifiableCredential,
};
use yew::{platform::spawn_local, prelude::*};

fn build_claim_property_from_schema_property(property: &SchemaProperty) -> ClaimProperty {
    match property {
        SchemaProperty::Value(value) => match value.get_type() {
            SchemaPropertyValueType::Text => {
                ClaimProperty::Value(ClaimPropertyValue::Text("".to_string()))
            }
            SchemaPropertyValueType::Number => ClaimProperty::Value(ClaimPropertyValue::Number(0)),
            SchemaPropertyValueType::Boolean => {
                ClaimProperty::Value(ClaimPropertyValue::Boolean(false))
            }
        },
        SchemaProperty::Map(map) => {
            let mut claim_properties = HashMap::new();
            for (key, property) in map.iter() {
                let subproperties = build_claim_property_from_schema_property(property);
                claim_properties.insert(key.clone(), subproperties);
            }
            ClaimProperty::Map(claim_properties)
        }
        SchemaProperty::Array(array) => {
            let mut claim_properties = Vec::new();
            for property in array.iter() {
                let subproperties = build_claim_property_from_schema_property(property);
                claim_properties.push(subproperties);
            }
            ClaimProperty::Array(claim_properties)
        }
    }
}

fn build_claim_properties_from_schema_properties(
    schema_properties: &HashMap<String, SchemaProperty>,
) -> HashMap<String, ClaimProperty> {
    let mut claim_properties = HashMap::new();
    for (key, property) in schema_properties.iter() {
        let subproperties = build_claim_property_from_schema_property(property);
        claim_properties.insert(key.clone(), subproperties);
    }
    claim_properties
}

#[derive(Properties, PartialEq)]
pub struct CredentialBuilderProps {
    pub issuer: Issuer,
    pub schema: CredentialSchema,
    pub set_credential: Callback<Option<VerifiableCredential>>,
    pub set_schema: Callback<Option<CredentialSchema>>,
}

#[function_component(CredentialBuilder)]
pub fn credential_builder(props: &CredentialBuilderProps) -> Html {
    let issuer = &props.issuer;
    let schema = &props.schema;
    let set_credential = props.set_credential.clone();
    let set_schema = props.set_schema.clone();
    let schema_properties = &props.schema.get_properties().clone();
    let claim_properties_state = use_state(|| {
        Rc::new(RefCell::new(build_claim_properties_from_schema_properties(
            schema_properties,
        )))
    });
    let claim_properties = claim_properties_state.clone();

    let update_nested_claim_property = {
        let schema_properties = schema_properties.clone();
        let claim_properties = claim_properties.clone();
        Callback::from(
            move |(path, claim_value): (Vec<String>, ClaimPropertyValue)| {
                let mut path = path.iter();
                let path_length = path.len();
                let first = path.next().expect("First path element not found.");
                let mut schema_property = schema_properties
                    .get(first)
                    .expect("Schema property not found.");
                let mut claim_properties = claim_properties.borrow_mut();
                let mut claim_property = claim_properties
                    .get_mut(first)
                    .expect("Claim property not found.");
                for key in path.take(path_length - 1) {
                    match schema_property {
                        SchemaProperty::Array(schema_array) => {
                            let index = key.parse::<usize>().expect("Invalid index");
                            if let ClaimProperty::Array(claim_array) = claim_property {
                                schema_property =
                                    schema_array.get(index).expect("Schema property not found.");
                                claim_property = claim_array
                                    .get_mut(index)
                                    .expect("Claim property not found.");
                            } else {
                                error!("Incompatible schema and claim properties");
                            }
                        }
                        SchemaProperty::Map(schema_map) => {
                            if let ClaimProperty::Map(claim_map) = claim_property {
                                schema_property =
                                    schema_map.get(key).expect("Schema property not found.");
                                claim_property =
                                    claim_map.get_mut(key).expect("Claim property not found.");
                            } else {
                                error!("Incompatible schema and claim properties");
                            }
                        }
                        _ => {
                            error!("Incompatible schema and claim properties");
                        }
                    }
                }
                if let ClaimProperty::Value(_) = claim_property {
                    *claim_property = ClaimProperty::Value(claim_value);
                }
            },
        )
    };

    let issuer_id = issuer.get_id().get_str().to_string().clone();
    let schema_id = schema.get_id().get_str().to_string().clone();
    let submit_credential = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let context = vec!["https://www.w3.org/ns/credentials/v2".to_string()];
            let credential_id = Uuid::new_v4().to_string();
            let type_ = vec!["VerifiableCredential".to_string()];
            let issuer_id = issuer_id.clone();
            let valid_from = Utc::now().to_rfc3339();
            let valid_until = Utc::now().to_rfc3339();
            let credential_subject = claim_properties_state.borrow().clone();
            let schema_id = schema_id.clone();
            let credential_schema_ids = vec![schema_id];
            let set_credential = set_credential.clone();
            let request_data = json!({
                "context": context,
                "credential_id": credential_id,
                "type_": type_,
                "issuer_id": issuer_id,
                "valid_from": valid_from,
                "valid_until": valid_until,
                "credential_subject": credential_subject,
                "credential_schema_ids": credential_schema_ids,
            });
            let client = reqwest::Client::new();
            let future = async move {
                let url = format!("{}/credential/", BASE_URL);
                let resp = client.post(url).json(&request_data).send().await;
                match resp {
                    Ok(resp) => {
                        debug!("Response from creating new credential: {:?}", resp);
                        match resp.json::<VerifiableCredential>().await {
                            Ok(verifiable_credential) => {
                                set_credential.emit(Some(verifiable_credential));
                            }
                            Err(e) => {
                                error!(
                                    "Error parsing response from creating new credential: {:?}",
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error creating new credential: {:?}", e);
                    }
                }
            };
            spawn_local(future);
        })
    };

    let credential_form = html! {
        <div>
            <div class="p-4 border border-gray-200">
                <div>{"Issuer: "}</div>
                <h2 class="text-xl font-bold">{issuer.get_name()}</h2>
                <p class="text-gray-600">{"ID: "}{issuer.get_id()}</p>
            </div>
            <div class="p-4 border border-gray-200 mt-2">
                <div>{"Schema: "}</div>
                <h2 class="text-xl font-bold">{schema.get_name()}</h2>
                <p class="text-gray-300">{"ID: "}{schema.get_id()}</p>
                <p class="text-gray-300">{"Description: "}{schema.get_description()}</p>
            </div>
            <div class="p-4 border border-gray-200 mt-2">
                <div>{"Claims: "}</div>
                <ClaimBuilder schema={schema.clone()} claim_properties={Rc::clone(&claim_properties)} update_nested_claim_property={update_nested_claim_property} />
            </div>
        </div>
    };

    html! {
        <div class = "m-8">
            <h1 class="text-3xl text-center mb-2">{"Make Some Claims"}</h1>
            {credential_form}
            <div class="text-center mt-2">
                <button onclick={submit_credential} class="text-white bg-blue-300 rounded-md p-2">{"Submit"}</button>
            </div>
            <div class="text-center mt-2">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={move |_| set_schema.emit(None)}>
                    {"Back"}
                </button>
            </div>
        </div>
    }
}
