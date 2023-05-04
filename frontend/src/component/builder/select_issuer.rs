use crate::component::issuer::issuer_home::use_issuers;
use vc_core::Issuer;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectIssuerProps {
    pub set_issuer: Callback<Option<Issuer>>,
}

#[function_component(SelectIssuer)]
pub fn select_issuer(props: &SelectIssuerProps) -> Html {
    let (issuers, loading, _) = use_issuers();
    let set_issuer = props.set_issuer.clone();

    let issuer_list = issuers
        .iter()
        .map(|issuer| {
            let issuer_clone = issuer.clone();
            let set_issuer = set_issuer.clone();
            html! {
                <div class="p-4 border border-gray-200">
                    <button onclick={move |_| set_issuer.emit(Some(issuer_clone.clone()))}>
                    <h2 class="text-xl font-bold">{issuer.get_name()}</h2>
                    <p class="text-gray-600">{"ID: "}{issuer.get_id()}</p>
                    </button>
                </div>
            }
        })
        .collect::<Html>();

    let content = if loading {
        html! { <p>{"Loading issuers..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{issuer_list}</div> }
    };

    html! {
    <div class = "m-8">
        <h1 class="text-3xl text-center mb-2">{"Select An Issuer"}</h1>
        {content}
    </div> }
}
