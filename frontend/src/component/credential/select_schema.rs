use crate::component::schema::schema_home::use_schemas;
use vc_core::{CredentialSchema, Issuer};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectSchemaProps {
    pub issuer: Issuer,
    pub set_schema: Callback<Option<CredentialSchema>>,
    pub set_issuer: Callback<Option<Issuer>>,
}

#[function_component(SelectSchema)]
pub fn select_schema(props: &SelectSchemaProps) -> Html {
    let (schemas, loading, _) = use_schemas();
    let issuer = &props.issuer;
    let set_schema = props.set_schema.clone();
    let set_issuer = props.set_issuer.clone();

    let schema_list = schemas
        .iter()
        .map(|schema| {
            let schema_clone = schema.clone();
            let set_schema = set_schema.clone();
            html! {
                <div class="p-4 border border-gray-200">
                    <button onclick={move |_| set_schema.emit(Some(schema_clone.clone()))}>
                    <h2 class="text-xl font-bold">{schema.get_name()}</h2>
                    <p class="text-gray-600">{"ID: "}{schema.get_id()}</p>
                    </button>
                </div>
            }
        })
        .collect::<Html>();

    let content = if loading {
        html! { <p>{"Loading schemas..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{schema_list}</div> }
    };

    html! {
        <div class = "m-8">
            <div class="p-4 border border-gray-200 mb-2">
                <div>{"Issuer: "}</div>
                <h2 class="text-xl font-bold">{issuer.get_name()}</h2>
                <p class="text-gray-600">{"ID: "}{issuer.get_id()}</p>
            </div>
            <h1 class="text-3xl text-center mb-2">{"Select A Schema"}</h1>
            {content}
            <div class="text-center mt-2">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={move |_| set_issuer.emit(None)}>
                    {"Back"}
                </button>
            </div>
        </div>
    }
}
