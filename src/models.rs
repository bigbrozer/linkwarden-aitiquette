use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Deserialize, Serialize, Debug)]
pub struct Link {
    pub id: i64,
    pub name: String,
    pub url: String,
    #[serde(rename = "textContent")]
    pub text_content: Option<String>,
    pub collection: Collection,
    pub tags: Vec<JsonValue>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Collection {
    id: i64,
    #[serde(rename = "ownerId")]
    owner_id: i64,
    name: String,
}
