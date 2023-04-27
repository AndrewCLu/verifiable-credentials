use super::nav_bar::NavBar;
use crate::constants::BASE_URL;
use log::error;
use vc_core::Issuer;
use yew::{platform::spawn_local, prelude::*};

async fn get_issuer(issuer_id: String) -> Result<Issuer, reqwest::Error> {
    let url = format!("{}/issuer/{}", BASE_URL, issuer_id);
    let resp = reqwest::get(url).await?;
    let issuer: Issuer = resp.json().await?;
    Ok(issuer)
}

#[derive(Properties, PartialEq)]
pub struct IssuerDetailsProps {
    pub issuer_id: String,
}

#[function_component(IssuerDetails)]
pub fn issuer_details(props: &IssuerDetailsProps) -> Html {
    let issuer = use_state(|| None);
    let issuer_id = props.issuer_id.clone();

    let issuer_clone = issuer.clone();
    use_effect_with_deps(
        move |issuer_id| {
            let issuer_id = issuer_id.clone();
            let future = async move {
                match get_issuer(issuer_id.clone()).await {
                    Ok(fetched_issuer) => {
                        issuer_clone.set(Some(fetched_issuer));
                    }
                    Err(_) => {
                        error!("Failed to fetch issuer {}.", issuer_id);
                    }
                }
            };
            spawn_local(future);
            || ()
        },
        issuer_id,
    );

    match (*issuer).clone() {
        Some(issuer) => {
            html! {
                <div class="m-8">
                    <NavBar />
                        <div class="p-4 border border-gray-200">
                            <h2 class="text-xl font-bold">{issuer.get_name()}</h2>
                            <p class="text-gray-600">{"ID: "}{issuer.get_id()}</p>
                        </div>
                </div>
            }
        }
        None => {
            html! {
                <div class="m-8">
                    <NavBar />
                    <div>
                    {"Unable to fetch issuer."}
                    </div>
                </div>
            }
        }
    }
}
