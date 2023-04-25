use super::add_issuer::AddIssuer;
use super::issuer_list::IssuerList;
use std::rc::Rc;
use vc_core::Issuer;
use yew::{platform::spawn_local, prelude::*};

async fn get_all_issuers() -> Result<Vec<Issuer>, reqwest::Error> {
    let url = "http://localhost:8000/issuer/get_all_issuers";
    let resp = reqwest::get(url).await?;
    let issuers: Vec<Issuer> = resp.json().await?;
    Ok(issuers)
}

#[hook]
pub fn use_issuers() -> (Rc<Vec<Issuer>>, Rc<bool>, Rc<Callback<()>>) {
    let issuers = use_state(|| Rc::new(Vec::<Issuer>::new()));
    let loading = use_state(|| Rc::new(true));

    let (issuers_clone, loading_clone) = (issuers.clone(), loading.clone());
    let fetch_issuers = {
        Rc::new(Callback::from(move |_| {
            let issuers = issuers_clone.clone();
            let loading = loading_clone.clone();
            let future = async move {
                match get_all_issuers().await {
                    Ok(new_issuers) => {
                        issuers.set(Rc::new(new_issuers));
                        loading.set(Rc::new(false));
                    }
                    Err(_) => {
                        eprintln!("Failed to fetch issuers.");
                        loading.set(Rc::new(false));
                    }
                }
            };
            spawn_local(future);
        }))
    };

    let fetch_issuers_clone = fetch_issuers.clone();
    use_effect_with_deps(
        move |_| {
            fetch_issuers_clone.emit(());
            || ()
        },
        (),
    );

    let issuers = (*issuers).clone();
    let loading = (*loading).clone();
    (issuers, loading, fetch_issuers)
}

#[function_component(IssuerHome)]

pub fn issuer_home() -> Html {
    let (issuers, loading, fetch_issuers) = use_issuers();
    html! {
        <div class="m-8">
            <IssuerList issuers={issuers} loading={loading} />
            <div />
            <AddIssuer fetch_issuers={fetch_issuers.clone()} />
        </div>
    }
}
