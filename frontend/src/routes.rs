use crate::component::{
    home::Home, issuer_details::IssuerDetails, issuer_home::IssuerHome, not_found::NotFound,
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
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Issuer => html! { <IssuerHome /> },
        Route::IssuerDetails { id } => html! { <IssuerDetails issuer_id={id} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
