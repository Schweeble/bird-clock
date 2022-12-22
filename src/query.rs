use serde::Deserialize;

use crate::models::bird::Bird;

#[derive(Deserialize)]
pub struct QueryResponse {
    pub num_recordings: u32,
    pub num_species: u32,
    pub page: u32,
    pub num_pages: u32,
    pub recordings: Vec<Bird>
}