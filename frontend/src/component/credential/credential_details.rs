use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CredentialDetailsProps {
    pub credential_id: String,
}

#[function_component(CredentialDetails)]
pub fn credential_details(_props: &CredentialDetailsProps) -> Html {
    html! {
        <div class="m-8">
            {"credential details"}
        </div>
    }
}
