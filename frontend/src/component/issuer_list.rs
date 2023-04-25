use std::rc::Rc;
use vc_core::Issuer;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct IssuerListProps {
    pub issuers: Rc<Vec<Issuer>>,
    pub loading: Rc<bool>,
}

#[function_component(IssuerList)]
pub fn issuer_list(props: &IssuerListProps) -> Html {
    let IssuerListProps { issuers, loading } = props;
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

    let content = if **loading {
        html! { <p>{"Loading issuers..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{issuer_list}</div> }
    };

    html! { <div>{content}</div> }
}
