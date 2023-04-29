use crate::component::{
    credential::credential_home::CredentialHome,
    home::Home,
    issuer::{issuer_details::IssuerDetails, issuer_home::IssuerHome},
    not_found::NotFound,
    schema::{schema_details::SchemaDetails, schema_home::SchemaHome},
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
    #[at("/credential")]
    Credential,
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
        Route::Credential => html! { <CredentialHome /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
