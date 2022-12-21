use chrono::DateTime;
use chrono::Timelike;
use iced::Renderer;
use iced::executor;
use iced::theme::{self, Theme};
use iced::widget::Radio;
use iced::widget::{column, container, row, text, radio};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};
use chrono::Local;

use std::time::Instant;
use std::time::{Duration};

pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

struct Stopwatch {
    current_time: DateTime<Local>,
    state: State,
    am_pm: bool
}

enum State {
    Ticking,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
    AM_PM(bool)
}

impl Application for Stopwatch {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Stopwatch, Command<Message>) {
        (
            Stopwatch {
                current_time: Local::now(),
                state: State::Ticking,
                am_pm: true
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Bird Clock")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(_now) => {
                self.current_time = Local::now()
            }
            Message::AM_PM(on) => {
                self.am_pm = !on;
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Ticking { .. } => {
                iced_futures::backend::native::tokio::time::every(Duration::from_millis(100)).map(Message::Tick)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let minute: u32 = self.current_time.minute();
        let mut hour: u32 = self.current_time.hour();

        if self.am_pm && hour == 0 {
            hour = 12;
        } else if self.am_pm {
            hour = hour % 12
        }

        let seconds = self.current_time.second();
        let show_match = |show| {
            match show {
                true => Some(show),
                false=> None
            }
        };
        let am_pm_label = if self.am_pm {
            let label = if self.current_time.hour() >= 12 {
                "PM"
            } else {
                "AM"
            };
            label
           
        } else {
            ""
        };

        let duration = text(format!(
            "{:0>1}:{:0>2}:{:0>2}{}",
            hour,
            minute,
            seconds,
            am_pm_label
        ))
        .size(180);

        let format_radio = |label: &str, show: bool| -> Radio<Message, Renderer> {
            radio(
                label, 
                show, 
                show_match(show), 
                Message::AM_PM
            )
            .width(Length::Units(100))
        };

        let controls = row![format_radio("AM/PM", self.am_pm)];

        let content = column![duration, controls]
            .align_items(Alignment::Center)
            .spacing(40);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}