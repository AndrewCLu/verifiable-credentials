use std::rc::Rc;
use vc_core::Verifier;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct VerifierListProps {
    pub verifiers: Rc<Vec<Verifier>>,
    pub loading: bool,
}

#[function_component(VerifierList)]
pub fn verifier_list(props: &VerifierListProps) -> Html {
    let VerifierListProps { verifiers, loading } = props;
    let verifier_list = verifiers
        .iter()
        .map(|verifier| {
            let verifier_id = verifier.get_id().get_str().to_string();
            html! {
                <div class="p-4 border border-gray-200">
                <h2 class="text-xl font-bold">{verifier.get_name()}</h2>
                    <p class="text-gray-600">{"ID: "}{verifier_id}</p>
                    <p class="text-gray-600">{"Schema ID: "}{verifier.get_schema_id()}</p>
                </div>
            }
        })
        .collect::<Html>();

    let content = if *loading {
        html! { <p>{"Loading verifiers..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{verifier_list}</div> }
    };

    html! { <div class = "m-8">
    <h1 class="text-3xl text-center mb-2">{"All Verifiers"}</h1>
    {content}</div> }
}
