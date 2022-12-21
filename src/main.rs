use eframe::egui;
use reqwest::Url;

mod models;

#[tokio::main]
async fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();

    // let options = eframe::NativeOptions {
    //     initial_window_size: Some(egui::vec2(320.0, 240.0)),
    //     ..Default::default()
    // };
    // eframe::run_native(
    //     "My egui App",
    //     options,
    //     Box::new(|_cc| Box::new(BirdClock::default())),
    // )
    let base = "https://xeno-canto.org/api/2/recordings?query=";

    let url: Url = Url::parse((base.to_owned() + "Acorn Woodpecker&page=1").as_str()).expect("couldn't encode");
    
    print!("{}\n", url);

    let resp = reqwest::get(url).await.unwrap();
}
