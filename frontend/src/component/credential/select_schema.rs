use crate::component::schema::schema_home::use_schemas;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectSchemaProps {
    pub set_schema_id: Callback<String>,
}

#[function_component(SelectSchema)]
pub fn select_schema(props: &SelectSchemaProps) -> Html {
    let (schemas, loading, _) = use_schemas();
    let set_schema_id = props.set_schema_id.clone();

    let schema_list = schemas
        .iter()
        .map(|schema| {
            let schema_id = schema.get_id().get_str().to_string();
            let set_schema_id = set_schema_id.clone();
            html! {
                <div class="p-4 border border-gray-200">
                    <button onclick={move |_| set_schema_id.emit(schema_id.clone())}>
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

    html! { <div class = "m-8">
    <h1 class="text-3xl text-center mb-2">{"Select A Schema"}</h1>
    {content}</div> }
}
