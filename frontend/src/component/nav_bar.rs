use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
            <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 md:flex-row md:space-x-8 md:mt-0">
                <li class="m-2 p-2 border border-slate-200 bg-slate-100 rounded-lg"><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                <li class="m-2 p-2 border border-slate-200 bg-slate-100 rounded-lg"><Link<Route> to={Route::Issuer}>{ "Issuers" }</Link<Route>></li>
                <li class="m-2 p-2 border border-slate-200 bg-slate-100 rounded-lg"><Link<Route> to={Route::Schema}>{ "Schemas" }</Link<Route>></li>
                <li class="m-2 p-2 border border-slate-200 bg-slate-100 rounded-lg"><Link<Route> to={Route::Credential}>{ "Credential Builder" }</Link<Route>></li>
                <li class="m-2 p-2 border border-slate-200 bg-slate-100 rounded-lg"><Link<Route> to={Route::MyCredentials}>{ "My Credential" }</Link<Route>></li>
            </ul>
        </nav>
    }
}
