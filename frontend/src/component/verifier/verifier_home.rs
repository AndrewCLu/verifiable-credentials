use super::add_verifier::AddVerifier;
use super::verifier_list::VerifierList;
use crate::component::nav_bar::NavBar;
use crate::constants::BASE_URL;
use log::error;
use std::rc::Rc;
use vc_core::Verifier;
use yew::{platform::spawn_local, prelude::*};

async fn get_all_verifiers() -> Result<Vec<Verifier>, reqwest::Error> {
    let url = format!("{}/verifier/", BASE_URL);
    let resp = reqwest::get(url).await?;
    let verifiers: Vec<Verifier> = resp.json().await?;
    Ok(verifiers)
}

#[hook]
pub fn use_verifiers() -> (Rc<Vec<Verifier>>, bool, Callback<()>) {
    let verifiers = use_state(|| Rc::new(Vec::<Verifier>::new()));
    let loading = use_state(|| true);

    let fetch_verifiers = {
        let (verifiers_clone, loading_clone) = (verifiers.clone(), loading.clone());
        Callback::from(move |_| {
            let verifiers = verifiers_clone.clone();
            let loading = loading_clone.clone();
            loading.set(true);
            let future = async move {
                match get_all_verifiers().await {
                    Ok(new_verifiers) => {
                        verifiers.set(Rc::new(new_verifiers));
                    }
                    Err(_) => {
                        error!("Failed to fetch verifiers.");
                    }
                }
                loading.set(false);
            };
            spawn_local(future);
        })
    };

    let fetch_verifiers_clone = fetch_verifiers.clone();
    use_effect_with_deps(
        move |_| {
            fetch_verifiers_clone.emit(());
            || ()
        },
        (),
    );

    (Rc::clone(&verifiers), *loading, fetch_verifiers)
}

#[function_component(VerifierHome)]
pub fn verifier_home() -> Html {
    let (verifiers, loading, fetch_verifiers) = use_verifiers();
    html! {
        <div class="m-8">
        <NavBar />
        <div />
            <VerifierList verifiers={verifiers} loading={loading} />
            <div />
            <AddVerifier fetch_verifiers={fetch_verifiers} />
        </div>
    }
}
