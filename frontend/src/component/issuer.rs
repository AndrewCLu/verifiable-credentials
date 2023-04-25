use std::rc::Rc;
use vc_core::Issuer;
use yew::{platform::spawn_local, prelude::*};

async fn fetch_issuers() -> Result<Vec<Issuer>, reqwest::Error> {
    let url = "http://localhost:8000/issuer/get_all_issuers";
    let resp = reqwest::get(url).await?;
    let issuers: Vec<Issuer> = resp.json().await?;
    Ok(issuers)
}

#[function_component(IssuerList)]
pub fn issuer_list() -> Html {
    let issuers = use_state(Vec::<Issuer>::new);
    let loading = use_state(|| true);

    let (issuers_clone, loading_clone) = (issuers.clone(), loading.clone());
    use_effect_with_deps(
        |()| {
            let future = async move {
                match fetch_issuers().await {
                    Ok(new_issuers) => {
                        issuers_clone.set(new_issuers);
                        loading_clone.set(false);
                    }
                    Err(_) => {
                        eprintln!("Failed to fetch issuers.");
                        loading_clone.set(false);
                    }
                }
            };

            spawn_local(future);
            || ()
        },
        (),
    );

    let issuer_list = issuers
        .iter()
        .map(|issuer| {
            html! {
                <div class="p-4 border border-gray-200">
                    <h2 class="text-xl font-bold">{issuer.get_name()}</h2>
                    <p class="text-gray-600">{"ID: "}{issuer.get_id()}</p>
                </div>
            }
        })
        .collect::<Html>();

    let content = if *loading {
        html! { <p>{"Loading issuers..."}</p> }
    } else {
        html! { <div class="grid grid-cols-4 gap-4">{issuer_list}</div> }
    };

    html! { <div>{content}</div> }
}
