use crate::Route;
use std::rc::Rc;
use vc_core::CredentialSchema;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SchemaListProps {
    pub schemas: Rc<Vec<CredentialSchema>>,
    pub loading: bool,
}

#[function_component(SchemaList)]
pub fn schema_list(props: &SchemaListProps) -> Html {
    let SchemaListProps { schemas, loading } = props;
    let schema_list = schemas
        .iter()
        .map(|schema| {
            let schema_id = schema.get_id().get_str().to_string();
            html! {
                <div class="p-4 border border-gray-200">
                <Link<Route> to={Route::SchemaDetails {id: schema_id}}>
                    <h2 class="text-xl font-bold">{schema.get_name()}</h2>
                    <p class="text-gray-600">{"ID: "}{schema.get_id()}</p>
                </Link<Route>>

                </div>
            }
        })
        .collect::<Html>();

    let content = if *loading {
        html! { <p>{"Loading schemas..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{schema_list}</div> }
    };

    html! { <div class = "m-8">
    <h1 class="text-3xl text-center mb-2">{"All Schemas"}</h1>
    {content}</div> }
}
