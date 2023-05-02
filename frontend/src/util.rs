use crate::constants::{INDEXEDDB_DATABASE_NAME, INDEXEDDB_OBJECT_STORE_NAME};
use indexed_db_futures::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::DomException;

pub async fn get_indexeddb_connector() -> Result<IdbDatabase, DomException> {
    let mut db_req: OpenDbRequest = IdbDatabase::open_u32(INDEXEDDB_DATABASE_NAME, 1)?;
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        if let None = evt
            .db()
            .object_store_names()
            .find(|n| n == INDEXEDDB_OBJECT_STORE_NAME)
        {
            evt.db().create_object_store(INDEXEDDB_OBJECT_STORE_NAME)?;
        }
        Ok(())
    }));

    Ok(db_req.into_future().await?)
}
