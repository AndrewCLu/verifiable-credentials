use crate::component::verifier::verifier_home::use_verifiers;
use vc_core::Verifier;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectVerifierProps {
    pub set_verifier: Callback<Option<Verifier>>,
}

#[function_component(SelectVerifier)]
pub fn select_verifier(props: &SelectVerifierProps) -> Html {
    let (verifiers, loading, _) = use_verifiers();
    let set_verifier = props.set_verifier.clone();

    let verifier_list = verifiers
        .iter()
        .map(|verifier| {
            let verifier_clone = verifier.clone();
            let set_verifier = set_verifier.clone();
            html! {
                <div class="p-4 border border-gray-200">
                    <button onclick={move |_| set_verifier.emit(Some(verifier_clone.clone()))}>
                    <h2 class="text-xl font-bold">{verifier.get_name()}</h2>
                    <p class="text-gray-600">{"ID: "}{verifier.get_id()}</p>
                    <p class="text-gray-600">{"Schema ID: "}{verifier.get_schema_id()}</p>
                    </button>
                </div>
            }
        })
        .collect::<Html>();

    let content = if loading {
        html! { <p>{"Loading verifiers..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{verifier_list}</div> }
    };

    html! {
    <div class = "m-8">
        <h1 class="text-3xl text-center mb-2">{"Select An Verifier"}</h1>
        {content}
    </div> }
}
