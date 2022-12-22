use std::fs::File;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bird {
    pub id: String,
    pub gen: String,
    pub sp: String,
    pub ssp: String,
    pub en: String,
    pub rec: String,
    pub cnt: String,
    pub loc: String,
    pub lat: String,
    pub lng: String,
    #[serde(rename(deserialize = "type"))]
    pub call_type: String,
    pub file: String,
}

impl Default for Bird {
    fn default() -> Self {
        Bird {
            id: "134880".into(),
            gen: "Pheucticus".into(),
            sp: "ludovicianus".into(),
            ssp: "".into(),
            en: "Rose-breasted Grosbeak".into(),
            rec: "Jonathon Jongsma".into(),
            cnt: "United States".into(),
            loc: "Grey Cloud Dunes SNA, Washington, Minnesota".into(),
            lat: "44.793".into(),
            lng: "-92.962".into(),
            call_type: "song".into(),
            file: "http://www.xeno-canto.org/134880/download".into(),
        }
    }
}