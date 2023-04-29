use vc_core::{ClaimProperty, ClaimPropertyValue, Credential};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ClaimPropertyValueNodeProps {
    pub value: ClaimPropertyValue,
}

#[function_component(ClaimPropertyValueNode)]
pub fn claim_property_value_node(props: &ClaimPropertyValueNodeProps) -> Html {
    let value = props.value.clone();
    match value {
        ClaimPropertyValue::Text(text) => {
            html!(
                <>
                    {text}
                </>
            )
        }
        ClaimPropertyValue::Number(number) => {
            html!(
                <>
                    {number.to_string()}
                </>
            )
        }
        ClaimPropertyValue::Boolean(boolean) => {
            html!(
                <>
                    {boolean.to_string()}
                </>
            )
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ClaimPropertyNodeProps {
    pub property: ClaimProperty,
}

#[function_component(ClaimPropertyNode)]
pub fn claim_property_node(props: &ClaimPropertyNodeProps) -> Html {
    let property = props.property.clone();
    match property {
        ClaimProperty::Value(value) => {
            html! {
                <>
                    <ClaimPropertyValueNode value={value} /> {","}
                </>
            }
        }
        ClaimProperty::Array(array) => {
            html! {
                <>
                    {"["}
                        <ul>
                            {for array.iter().map(|value| {
                                html! {
                                    <li class="ml-8">
                                        <ClaimPropertyNode property={value.clone()} />
                                    </li>
                                }
                            })}
                        </ul>
                    {"],"}
                </>
            }
        }
        ClaimProperty::Map(map) => {
            html! {
                <>
                    {"{"}
                    <div class="ml-8">
                        {for map.iter().map(|(key, value)| {
                            html! {
                                <div>
                                    {"\""} {key} {"\": "}
                                    <ClaimPropertyNode property={value.clone()} />
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
pub struct DisplayCredentialProps {
    pub credential: Credential,
}

#[function_component(DisplayCredential)]
pub fn display_credential(props: &DisplayCredentialProps) -> Html {
    let credential = props.credential.clone();
    let claims = credential.get_credential_subject();
    html! {
        <div>
            <h2 class="text-xl font-bold">{"Credential: "}</h2>
            <p class="text-gray-600">{"ID: "}{credential.get_id()}</p>
            <p class="text-gray-600">{"Issuer: "}{credential.get_issuer()}</p>
            <p class="text-gray-600">{"Valid From: "}{credential.get_valid_from()}</p>
            <p class="text-gray-600">{"Valid Until: "}{credential.get_valid_until()}</p>
            <div>
                {"{"}
                {for claims.iter().map(|(key, value)| {
                    html! {
                        <div class="ml-4">
                            {"\""} {key} {"\": "} <ClaimPropertyNode property={value.clone()} />
                        </div>
                    }
                })}
                {"}"}
            </div>
        </div>
    }
}
