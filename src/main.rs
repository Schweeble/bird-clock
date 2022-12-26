use egui_wgpu::{WgpuConfiguration, SurfaceErrorAction};
use event::Event;
use interface::Interface;
use models::bird::Bird;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf, sync::Arc, time::Duration,
};
use tokio::runtime::Runtime;

mod error;
mod interface;
mod models;
mod query;
mod event;

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

    let (sound_tx, sound_rx) = std::sync::mpsc::channel();
    let (bird_tx, bird_rx) = std::sync::mpsc::channel();

    tokio::spawn(async move {
        loop {
            let tx = bird_tx.clone();
            let mut interval = tokio::time::interval(Duration::from_secs(3600));
            interval.tick().await;
            query::get_bird(tx).await;
        }
    });

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(10));
        loop {
            if let Ok(e) = sound_rx.try_recv() {
                match e {
                    Event::PlaySound(bird) => {
                        run_sound(bird);
                    },
                    _ => {},
                }
            }
            interval.tick().await;
        }
    });

    let wgpu_options = WgpuConfiguration {
        device_descriptor: wgpu::DeviceDescriptor {
            label: Some("egui bird clock device"),
            features: wgpu::Features::default(),
            limits: wgpu::Limits::downlevel_defaults()
        },
        backends: wgpu::Backends::VULKAN | wgpu::Backends::PRIMARY | wgpu::Backends::GL,
        present_mode: wgpu::PresentMode::AutoVsync,
        power_preference: wgpu::PowerPreference::LowPower,
        depth_format: None,
        on_surface_error: Arc::new(|err| {
            if err == wgpu::SurfaceError::Outdated {
                // this error occurs when the app is minimized on windows,
                // do nothing
            } else {
                println!("Dropped frame with error: {}", err);
            }
            SurfaceErrorAction::SkipFrame
        })
    };

    let options = eframe::NativeOptions {
        fullscreen: true,
        renderer: eframe::Renderer::Wgpu,
        wgpu_options: wgpu_options,
        ..Default::default()
    };

    eframe::run_native(
        "Bird Clock",
        options,
        Box::new(|cc| Box::new(Interface::new(cc, sound_tx, bird_rx))),
    );
}

fn run_sound(bird: Bird) {
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
