use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    api_key: String,
    auth_domain: Option<String>,
    database_url: Option<String>,
    project_id: Option<String>,
    storage_bucket: Option<String>,
    messaging_sender_id: Option<String>,
    app_id: Option<String>,
}

impl Config {
    pub fn initialize(
        api_key: &str,
        auth_domain: Option<String>,
        database_url: Option<String>,
        project_id: Option<String>,
        storage_bucket: Option<String>,
        messaging_sender_id: Option<String>,
        app_id: Option<String>,
    ) -> JsValue {
        let config = Config {
            api_key: String::from(api_key),
            auth_domain,
            database_url,
            project_id,
            storage_bucket,
            messaging_sender_id,
            app_id,
        };
        serde_wasm_bindgen::to_value(&config).unwrap()
    }
}
