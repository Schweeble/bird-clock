use ::egui::FontDefinitions;
use eyre::Result;
use interface::Interface;
use models::bird::Bird;
use reqwest::Url;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};
use tokio::runtime::Runtime;

mod error;
mod interface;
mod models;
mod query;

fn main() {
    // Run the GUI in the main thread.
    {
        // Silence wgpu log spam (https://github.com/gfx-rs/wgpu/issues/3206)
        let mut rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
        for loud_crate in ["naga", "wgpu_core", "wgpu_hal"] {
            if !rust_log.contains(&format!("{loud_crate}=")) {
                rust_log += &format!(",{loud_crate}=warn");
            }
        }
        std::env::set_var("RUST_LOG", rust_log);
    }

    let rt = Runtime::new().expect("Couldn't start async runtime");

    let _enter = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            run_sound();
        })
    });

    let options = eframe::NativeOptions {
        fullscreen: true,
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "Bird Clock",
        options,
        Box::new(|cc| Box::new(Interface::new(cc))),
    );
}

fn run_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let mut path = PathBuf::new();
    path.push(".");
    path.push("data");
    path.push("backup");
    path.push("XC134880_Rose-breasted_Grosbeak_Pheucticus_ludovicianus");
    path.set_extension("mp3");

    println!("{}", path.as_path().display());
    let file = BufReader::new(File::open(path.as_path()).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}
