use chrono::{DateTime, Local, Timelike};
use egui::Ui;

pub struct Clock {
    current_time: DateTime<Local>,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            current_time: Local::now(),
        }
    }
}

impl Clock {
    pub fn ui(&mut self, ui: &mut Ui, current_time: DateTime<Local>) {
        self.current_time = current_time;
        ui.label(format!(
            "{:02}{:02}{:02}",
            self.current_time.hour(),
            self.current_time.minute(),
            self.current_time.second(),
        ));
    }
}

pub struct Interface {
    clock: Clock,
}

impl Interface {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            clock: Clock::default(),
        }
    }
}

impl eframe::App for Interface {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(5.0);
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.clock.ui(ui, Local::now());
            });
        ctx.request_repaint_after(std::time::Duration::from_secs_f32(0.1));
    }
}
