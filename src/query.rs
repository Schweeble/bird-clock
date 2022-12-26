use std::sync::mpsc::Sender;

use serde::Deserialize;

use crate::{models::bird::Bird, event::Event};

#[derive(Deserialize)]
pub struct QueryResponse {
    pub num_recordings: u32,
    pub num_species: u32,
    pub page: u32,
    pub num_pages: u32,
    pub recordings: Vec<Bird>
}

pub async fn get_bird(bird_tx: Sender<Event>) {
    
}