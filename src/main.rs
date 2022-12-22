use std::{sync::mpsc::{Sender, Receiver}, time::Duration};
use eframe::egui;
use models::bird::Bird;
use reqwest::Url;
use tokio::runtime::Runtime;
use tokio::task;

mod models;
mod error;
mod query;

// #[tokio::main]
// async fn main() {
//     // Log to stdout (if you run with `RUST_LOG=debug`).
//     // tracing_subscriber::fmt::init();

//     // let options = eframe::NativeOptions {
//     //     initial_window_size: Some(egui::vec2(320.0, 240.0)),
//     //     ..Default::default()
//     // };
//     // eframe::run_native(
//     //     "My egui App",
//     //     options,
//     //     Box::new(|_cc| Box::new(BirdClock::default())),
//     // )
//     // let base = "https://xeno-canto.org/api/2/recordings?query=";

//     // let url: Url = Url::parse((base.to_owned() + "Acorn Woodpecker&page=1").as_str()).expect("couldn't encode");
    
//     // print!("{}\n", url);

//     // let resp = reqwest::get(url).await.unwrap();
// }

struct ClockApp {
    tx: Sender<Bird>,
    rx: Receiver<Bird>,

    current_bird: Option<Bird>,
    backup_bird: Option<Bird>,


}

#[tokio::main]
async fn main() {
    
    let options = eframe::NativeOptions::default();
    // Run the GUI in the main thread.
    eframe::run_native(
        "Download and show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(ClockApp::default())),
    )
}

impl Default for ClockApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            tx,
            rx,
            current_bird: Some(Bird::default()),
            backup_bird: Some(Bird::default())
        }

    }
}

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {ui.spinner() });
    }
}
