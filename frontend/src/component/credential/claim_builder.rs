use log::error;
use std::collections::HashMap;
use vc_core::{
    ClaimProperty, ClaimPropertyValue, Credential, CredentialSchema, SchemaProperty,
    SchemaPropertyType, SchemaPropertyValue,
};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropertyValueNodeProps {
    pub schema_property: SchemaPropertyValue,
    pub claim_property: ClaimPropertyValue,
    pub path: Vec<String>,
    pub update_nested_claim_property: Callback<(ClaimPropertyValue, Vec<String>)>,
}

#[function_component(PropertyValueNode)]
pub fn property_value_node(props: &PropertyValueNodeProps) -> Html {
    let description = props.schema_property.get_description();

    html! {
        <div>
            <input
                type="text"
                oninput={Callback::from(move |e: InputEvent| {
                    let target: Option<EventTarget> = e.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                    form_data_clone.set_with(move |data| {
                        // Set the value in the nested structure using the path
                        update_nested_claim_property(data, &current_path, input);
                    });
                })}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PropertyNodeProps {
    pub schema_property: SchemaProperty,
    pub claim_property: ClaimProperty,
}

#[function_component(PropertyNode)]
pub fn property_node(props: &PropertyNodeProps) -> Html {
    let schema_property = props.schema_property.clone();
    let claim_property = props.claim_property.clone();

    match (schema_property, claim_property) {
        (SchemaProperty::Value(schema_value), ClaimProperty::Value(claim_value)) => {
            html! {
                <>
                    <PropertyValueNode schema_property={schema_value} claim_property={claim_value} /> {","}
                </>
            }
        }
        (SchemaProperty::Array(schema_array), ClaimProperty::Array(claim_array)) => {
            html! {
                <>
                    {"["}
                        <ul>
                            {for schema_array.iter().zip(claim_array.iter()).map(|(schema_value, claim_value)| {
                                html! {
                                    <li class="ml-8">
                                        <PropertyNode schema_property={schema_value.clone()} claim_property={claim_value.clone()} />
                                    </li>
                                }
                            })}
                        </ul>
                    {"],"}
                </>
            }
        }
        (SchemaProperty::Map(schema_map), ClaimProperty::Map(claim_map)) => {
            html! {
                <>
                    {"{"}
                    <div class="ml-8">
                        {for schema_map.iter().map(|(key, schema_value)| {
                            let claim_value = claim_map.get(key).expect("Claim property not found.");
                            html! {
                                <div>
                                    {"\""} {key} {"\": "}
                                    <PropertyNode schema_property={schema_value.clone()} claim_property={claim_value.clone()} />
                                </div>
                            }
                        })}
                    </div>
                    {"},"}
                </>
            }
        }
        _ => {
            error!("Incompatible schema and claim properties");
            html! { "Incompatible schema and claim properties" }
        }
    }
}

// Helper function to update the nested ClaimProperty based on the path and the input value
fn update_nested_claim_property(claim_property: &mut ClaimProperty, path: &[String], value: &str) {
    if let Some((head, tail)) = path.split_first() {
        match claim_property {
            ClaimProperty::Map(map) => {
                if let Some(entry) = map.get_mut(head) {
                    update_nested_claim_property(entry, tail, value);
                }
            }
            // Add cases for Array if needed
            _ => {}
        }
    } else {
        // Set the value at the current node
        *claim_property = ClaimProperty::Value(ClaimPropertyValue::Text(value.to_string()));
    }
}

fn build_claim_tree_from_schema_property(property: &SchemaProperty) -> ClaimProperty {
    match property {
        SchemaProperty::Value(value) => ClaimProperty::Value(value.get_default_value()),
        SchemaProperty::Map(map) => {
            let mut claim_tree = HashMap::new();
            for (key, property) in map.iter() {
                let subtree = build_claim_tree_from_schema_property(property);
                claim_tree.insert(key.clone(), subtree);
            }
            ClaimProperty::Map(claim_tree)
        }
        SchemaProperty::Array(array) => {
            let mut claim_tree = Vec::new();
            for property in array.iter() {
                let subtree = build_claim_tree_from_schema_property(property);
                claim_tree.push(subtree);
            }
            ClaimProperty::Array(claim_tree)
        }
    }
}

fn build_claim_tree_from_schema(
    schema_properties: &HashMap<String, SchemaProperty>,
) -> HashMap<String, ClaimProperty> {
    let mut claim_tree = HashMap::new();
    for (key, property) in schema_properties.iter() {
        let subtree = build_claim_tree_from_schema_property(property);
        claim_tree.insert(key.clone(), subtree);
    }
    claim_tree
}

#[derive(Properties, Clone, PartialEq)]
pub struct ClaimBuilderProps {
    pub schema: CredentialSchema,
    pub set_credential: Callback<Option<Credential>>,
}

#[function_component(ClaimBuilder)]
pub fn claim_builder(props: &ClaimBuilderProps) -> Html {
    let schema_properties = &props.schema.get_properties().clone();
    let form_data = use_state(|| {
        build_claim_tree_from_schema(schema_properties);
    });
    let form_data_clone = form_data.clone();
    let claim_properties = (*form_data).clone();

    // TODO: Submit credential
    let on_submit = {
        let on_submit = props.on_submit.clone();
        Callback::from(move |event: yew::events::Submit| {
            event.prevent_default();
            on_submit.emit(form_data_clone.get().clone());
        })
    };

    html! {
        <div>
            <form onsubmit={on_submit}>
                {"{"}
                {for schema_properties.iter().map(|(key, schema_value)| {
                    let claim_value = claim_properties.get(key).expect("Claim property not found.");
                    html! {
                        <div class="ml-4">
                            {"\""} {key} {"\": "} <PropertyNode schema_property={schema_value.clone()} claim_property={claim_value.clone()} />
                        </div>
                    }
                })}
                {"}"}
                <button type="submit">{"Submit"}</button>
            </form>
        </div>
    }
}
