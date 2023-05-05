use crate::component::{
    builder::builder_home::BuilderHome,
    credential::{credential_details::CredentialDetails, credential_home::CredentialHome},
    home::Home,
    issuer::{issuer_details::IssuerDetails, issuer_home::IssuerHome},
    not_found::NotFound,
    schema::{schema_details::SchemaDetails, schema_home::SchemaHome},
    verifier::verifier_home::VerifierHome,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/issuer")]
    Issuer,
    #[at("/issuer/:id")]
    IssuerDetails { id: String },
    #[at("/schema")]
    Schema,
    #[at("/schema/:id")]
    SchemaDetails { id: String },
    #[at("/verifier")]
    Verifier,
    #[at("/builder")]
    Builder,
    #[at("/credential")]
    Credential,
    #[at("/credential/:id")]
    CredentialDetails { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Issuer => html! { <IssuerHome /> },
        Route::IssuerDetails { id } => html! { <IssuerDetails issuer_id={id} /> },
        Route::Schema => html! { <SchemaHome /> },
        Route::SchemaDetails { id } => html! { <SchemaDetails schema_id={id} /> },
        Route::Verifier => html! { <VerifierHome /> },
        Route::Builder => html! { <BuilderHome /> },
        Route::Credential => html! { <CredentialHome /> },
        Route::CredentialDetails { id } => html! { <CredentialDetails credential_id={id} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
