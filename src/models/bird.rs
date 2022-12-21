use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bird {
    pub id: u32,
    pub gen: String,
    pub sp: String,
    pub ssp: String,
    pub en: String,
    pub rec: String,
    pub loc: String,
    pub lat: String,
    pub lng: String,
    #[serde(rename(deserialize = "type"))]
    pub call_type: String,
    pub file: String,
}