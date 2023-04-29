use vc_core::{Credential, CredentialSchema, Issuer};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MakeClaimsProps {
    pub issuer: Issuer,
    pub schema: CredentialSchema,
    pub set_credential: Callback<Option<Credential>>,
    pub set_schema: Callback<Option<CredentialSchema>>,
}

#[function_component(MakeClaims)]
pub fn make_claims(props: &MakeClaimsProps) -> Html {
    let issuer = &props.issuer;
    let schema = &props.schema;
    let set_schema = props.set_schema.clone();

    let content = html! {
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
        </div>
    };

    html! {
        <div class = "m-8">
            <h1 class="text-3xl text-center mb-2">{"Make Some Claims"}</h1>
            {content}
            <div class="text-center mt-2">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={move |_| set_schema.emit(None)}>
                    {"Back"}
                </button>
            </div>
        </div>
    }
}
