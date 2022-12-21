use chrono::{DateTime, Local};


struct ClockState {
    current_time: DateTime<Local>,
    am_pm_toggle: bool,
    current_bird_name: String,
}