use super::issuer_home::use_issuers;
use yew::prelude::*;

#[function_component(IssuerList)]
pub fn issuer_list() -> Html {
    let (issuers, loading, _) = use_issuers();
    let issuer_list = issuers
        .iter()
        .map(|issuer| {
            html! {
                <div class="p-4 border border-gray-200">
                    <h2 class="text-xl font-bold">{issuer.get_name()}</h2>
                    <p class="text-gray-600">{"ID: "}{issuer.get_id()}</p>
                </div>
            }
        })
        .collect::<Html>();

    let content = if *loading {
        html! { <p>{"Loading issuers..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{issuer_list}</div> }
    };

    html! { <div>{content}</div> }
}
