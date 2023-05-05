use vc_core::{VerifiableCredential, Verifier};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PresentationBuilderProps {
    pub verifier: Verifier,
    pub credential: VerifiableCredential,
    pub set_verified: Callback<bool>,
    pub set_credential: Callback<Option<VerifiableCredential>>,
}

#[function_component(PresentationBuilder)]
pub fn presentation_builder(props: &PresentationBuilderProps) -> Html {
    html! {
        <div>
            {"I am the presentation builder"}
        </div>
    }
}
