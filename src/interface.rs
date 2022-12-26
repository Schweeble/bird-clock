use chrono::{DateTime, Local, Timelike};
use egui::{Ui, RichText, FontId};
use rodio::{Sink, OutputStream};
use std::sync::mpsc::{Sender, Receiver};

use crate::{models::bird::Bird, event::Event};

pub struct Clock {
    current_time: DateTime<Local>,
    current_bird: Bird,
    show_bird_info: bool,
    sound_tx: Sender<Event>,
    bird_rx: Receiver<Event>,
}

impl Clock {
    fn new(sound_tx: Sender<Event>, bird_rx: Receiver<Event>) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            current_time: Local::now(),
            current_bird: Bird::default(),
            show_bird_info: false,
            sound_tx,
            bird_rx,
        }
    }
}

impl Clock {
    pub fn ui(&mut self, ui: &mut Ui, current_time: DateTime<Local>) {
        self.current_time = current_time;
        
        if let Ok(event) = self.bird_rx.try_recv() {
            match event {
                Event::GotBird(bird) => { self.current_bird = bird; }
                _ => {}
            }
        }
        

        ui.vertical_centered_justified(|ui| {
            ui.add_space(50.);
            self.time_label(ui)
        });
        ui.vertical_centered_justified(|ui| {
            ui.add_space(20.);
            ui.add(toggle(&mut self.show_bird_info));
            self.toggle_bird_info(ui);
        });
        
    }

    fn time_label(&mut self, ui: &mut Ui) {
        ui.label(RichText::new(format!(
            "{:02}:{:02}:{:02}",
            self.current_time.hour(),
            self.current_time.minute(),
            self.current_time.second(),
        )).font(FontId::proportional(30.0)));
    }

    fn toggle_bird_info(&mut self, ui: &mut Ui) {
        if self.show_bird_info {
            let bird = self.current_bird.clone();
            ui.horizontal(|ui| {

            });
            ui.label(format!("Common Name: {}", bird.en));
            ui.label(format!("Scientific Name: {} {} {}", bird.gen, bird.sp, bird.ssp));
            ui.label(format!("Location Recorded: {}", bird.loc));
            if ui.button("Play Sound").clicked() {
                let _idk = self.sound_tx.send(Event::PlaySound(self.current_bird.clone()));
            }
        }
    }
}

pub struct Interface {
    clock: Clock,
}

impl Interface {
    pub fn new(_cc: &eframe::CreationContext, sound_tx: Sender<Event>, bird_rx: Receiver<Event>) -> Self {
        Self {
            clock: Clock::new(sound_tx, bird_rx),
        }
    }
}

impl eframe::App for Interface {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let local_time = Local::now();
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.clock.ui(ui, local_time);
            });
        ctx.request_repaint_after(std::time::Duration::from_secs_f32(0.1));
    }
}


/// Here is the same code again, but a bit more compact:
#[allow(dead_code)]
fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(toggle(&mut my_bool))`
/// iOS-style toggle switch.
///
/// ## Example:
/// ``` ignore
/// ui.add(toggle(&mut my_bool));
/// ```
pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui_compact(ui, on)
}