use super::add_schema::AddSchema;
use super::schema_list::SchemaList;
use crate::component::nav_bar::NavBar;
use crate::constants::BASE_URL;
use log::error;
use std::rc::Rc;
use vc_core::CredentialSchema;
use yew::{platform::spawn_local, prelude::*};

async fn get_all_schemas() -> Result<Vec<CredentialSchema>, reqwest::Error> {
    let url = format!("{}/schema/", BASE_URL);
    let resp = reqwest::get(url).await?;
    let schemas: Vec<CredentialSchema> = resp.json().await?;
    Ok(schemas)
}

#[hook]
pub fn use_schemas() -> (Rc<Vec<CredentialSchema>>, bool, Callback<()>) {
    let schemas = use_state(|| Rc::new(Vec::<CredentialSchema>::new()));
    let loading = use_state(|| true);

    let fetch_schemas = {
        let (schemas_clone, loading_clone) = (schemas.clone(), loading.clone());
        Callback::from(move |_| {
            let schemas = schemas_clone.clone();
            let loading = loading_clone.clone();
            loading.set(true);
            let future = async move {
                match get_all_schemas().await {
                    Ok(new_schemas) => {
                        schemas.set(Rc::new(new_schemas));
                    }
                    Err(_) => {
                        error!("Failed to fetch schemas.");
                    }
                }
                loading.set(false);
            };
            spawn_local(future);
        })
    };

    let fetch_schemas_clone = fetch_schemas.clone();
    use_effect_with_deps(
        move |_| {
            fetch_schemas_clone.emit(());
            || ()
        },
        (),
    );

    (Rc::clone(&schemas), *loading, fetch_schemas)
}

#[function_component(SchemaHome)]
pub fn schema_home() -> Html {
    let (schemas, loading, fetch_schemas) = use_schemas();
    html! {
        <div class="m-8">
        <NavBar />
        <div />
            <SchemaList schemas={schemas} loading={loading} />
            <div />
            <AddSchema fetch_schemas={fetch_schemas} />
        </div>
    }
}
