use log::error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use vc_core::{
    ClaimProperty, ClaimPropertyValue, CredentialSchema, SchemaProperty, SchemaPropertyValue,
    SchemaPropertyValueType,
};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropertyValueNodeProps {
    pub schema_value: SchemaPropertyValue,
    pub claim_value: ClaimPropertyValue,
    pub path: Vec<String>,
    pub update_nested_claim_property: Callback<(Vec<String>, ClaimPropertyValue)>,
}

#[function_component(PropertyValueNode)]
pub fn property_value_node(props: &PropertyValueNodeProps) -> Html {
    let schema_type = props.schema_value.get_type();
    let claim_value = props.claim_value.clone();
    let path = props.path.clone();
    let update_nested_claim_property = props.update_nested_claim_property.clone();

    match (schema_type, claim_value) {
        (SchemaPropertyValueType::Text, ClaimPropertyValue::Text(text)) => {
            html! {
                <>
                    <input
                        class="border rounded-md"
                        type="text"
                        value={text}
                        oninput={Callback::from(move |e: InputEvent| {
                            let path = path.clone();
                            let target: Option<EventTarget> = e.target();
                            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                            let value = input.expect("No claim input found.").value();
                            update_nested_claim_property.emit((path, ClaimPropertyValue::Text(value)));
                        })}
                    />
                </>
            }
        }
        (SchemaPropertyValueType::Number, ClaimPropertyValue::Number(number)) => {
            html! {
                <>
                    <input
                        class="border rounded-md"
                        type="number"
                        value={number.to_string()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let path = path.clone();
                            let target: Option<EventTarget> = e.target();
                            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                            let value = input.expect("No claim input found.").value().parse::<i32>().expect("Input must be a number.");
                            update_nested_claim_property.emit((path, ClaimPropertyValue::Number(value)));
                        })}
                    />
                </>
            }
        }
        (SchemaPropertyValueType::Boolean, ClaimPropertyValue::Boolean(boolean)) => {
            html! {
                <>
                    <input
                        class="border rounded-md"
                        type="checkbox"
                        checked={boolean}
                        onclick={Callback::from(move |_| {
                            let path = path.clone();
                            update_nested_claim_property.emit((path, ClaimPropertyValue::Boolean(!boolean)));
                        })}
                    />
                </>
            }
        }
        _ => {
            error!("Schema type and claim value type do not match.");
            html! {
                <div>
                    {"Schema type and claim value type do not match."}
                </div>
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PropertyNodeProps {
    pub schema_property: SchemaProperty,
    pub claim_property: ClaimProperty,
    pub path: Vec<String>,
    pub update_nested_claim_property: Callback<(Vec<String>, ClaimPropertyValue)>,
}

#[function_component(PropertyNode)]
pub fn property_node(props: &PropertyNodeProps) -> Html {
    let schema_property = props.schema_property.clone();
    let claim_property = props.claim_property.clone();
    let path = props.path.clone();
    let update_nested_claim_property = props.update_nested_claim_property.clone();

    match (schema_property, claim_property) {
        (SchemaProperty::Value(schema_value), ClaimProperty::Value(claim_value)) => {
            html! {
                <>
                    <PropertyValueNode schema_value={schema_value} claim_value={claim_value} path={path} update_nested_claim_property={update_nested_claim_property} /> {","}
                </>
            }
        }
        (SchemaProperty::Array(schema_array), ClaimProperty::Array(claim_array)) => {
            html! {
                <>
                    {"["}
                        <ul>
                            {for schema_array.iter().zip(claim_array.iter()).enumerate().map(|(index, (schema_value, claim_value))| {
                                let mut new_path = path.clone();
                                let update_nested_claim_property = update_nested_claim_property.clone();
                                new_path.push(index.to_string());
                                html! {
                                    <li class="ml-8">
                                        <PropertyNode schema_property={schema_value.clone()} claim_property={claim_value.clone()} path={new_path} update_nested_claim_property={update_nested_claim_property} />
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
                            let mut new_path = path.clone();
                            let update_nested_claim_property = update_nested_claim_property.clone();
                            new_path.push(key.clone());
                            html! {
                                <div>
                                    {"\""} {key} {"\": "}
                                    <PropertyNode schema_property={schema_value.clone()} claim_property={claim_value.clone()} path={new_path} update_nested_claim_property={update_nested_claim_property} />
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

#[derive(Properties, Clone, PartialEq)]
pub struct ClaimBuilderProps {
    pub schema: CredentialSchema,
    pub claim_properties: Rc<RefCell<HashMap<String, ClaimProperty>>>,
    pub update_nested_claim_property: Callback<(Vec<String>, ClaimPropertyValue)>,
}

#[function_component(ClaimBuilder)]
pub fn claim_builder(props: &ClaimBuilderProps) -> Html {
    let schema_properties = &props.schema.get_properties().clone();
    let update_nested_claim_property = props.update_nested_claim_property.clone();
    let claim_properties = props.claim_properties.clone();
    let claim_properties_map = claim_properties.borrow();

    html! {
        <div>
            <form>
                {"{"}
                {for schema_properties.iter().map(|(key, schema_value)| {
                    let update_nested_claim_property = update_nested_claim_property.clone();
                    let claim_value = claim_properties_map.get(key).expect("Claim property not found.");
                    let path = vec![key.clone()];
                    html! {
                        <div class="ml-4">
                            {"\""} {key} {"\": "} <PropertyNode schema_property={schema_value.clone()} claim_property={claim_value.clone()} path={path} update_nested_claim_property={update_nested_claim_property} />
                        </div>
                    }
                })}
                {"}"}
            </form>
        </div>
    }
}
