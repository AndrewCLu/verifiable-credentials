use crate::Route;
use std::rc::Rc;
use vc_core::Issuer;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct IssuerListProps {
    pub issuers: Rc<Vec<Issuer>>,
    pub loading: bool,
}

#[function_component(IssuerList)]
pub fn issuer_list(props: &IssuerListProps) -> Html {
    let IssuerListProps { issuers, loading } = props;
    let issuer_list = issuers
        .iter()
        .map(|issuer| {
            let issuer_id = issuer.get_id().get_str().to_string();
            html! {
                <div class="p-4 border border-gray-200">
                    <Link<Route> to={Route::IssuerDetails {id: issuer_id}}>
                        <h2 class="text-xl font-bold">{issuer.get_name()}</h2>
                        <p class="text-gray-600">{"ID: "}{issuer.get_id()}</p>
                    </Link<Route>>
                </div>
            }
        })
        .collect::<Html>();

    let content = if *loading {
        html! { <p>{"Loading issuers..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{issuer_list}</div> }
    };

    html! {
        <div class = "m-8">
            <h1 class="text-3xl text-center mb-2">{"All Issuers"}</h1>
            {content}
        </div>
    }
}
