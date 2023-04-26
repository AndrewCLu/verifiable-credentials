use super::add_issuer::AddIssuer;
use super::issuer_list::IssuerList;
use super::nav_bar::NavBar;
use crate::constants::BASE_URL;
use log::error;
use std::rc::Rc;
use vc_core::Issuer;
use yew::{platform::spawn_local, prelude::*};

async fn get_all_issuers() -> Result<Vec<Issuer>, reqwest::Error> {
    let url = format!("{}/issuer/get_all_issuers", BASE_URL);
    let resp = reqwest::get(url).await?;
    let issuers: Vec<Issuer> = resp.json().await?;
    Ok(issuers)
}

#[hook]
pub fn use_issuers() -> (Rc<Vec<Issuer>>, bool, Callback<()>) {
    let issuers = use_state(|| Rc::new(Vec::<Issuer>::new()));
    let loading = use_state(|| true);

    let fetch_issuers = {
        let (issuers_clone, loading_clone) = (issuers.clone(), loading.clone());
        Callback::from(move |_| {
            let issuers = issuers_clone.clone();
            let loading = loading_clone.clone();
            loading.set(true);
            let future = async move {
                match get_all_issuers().await {
                    Ok(new_issuers) => {
                        issuers.set(Rc::new(new_issuers));
                    }
                    Err(_) => {
                        error!("Failed to fetch issuers.");
                    }
                }
                loading.set(false);
            };
            spawn_local(future);
        })
    };

    let fetch_issuers_clone = fetch_issuers.clone();
    use_effect_with_deps(
        move |_| {
            fetch_issuers_clone.emit(());
            || ()
        },
        (),
    );

    (Rc::clone(&issuers), *loading, fetch_issuers)
}

#[function_component(IssuerHome)]

pub fn issuer_home() -> Html {
    let (issuers, loading, fetch_issuers) = use_issuers();
    html! {
        <div class="m-8">
        <NavBar />
        <div />
            <IssuerList issuers={issuers} loading={loading} />
            <div />
            <AddIssuer fetch_issuers={fetch_issuers} />
        </div>
    }
}
