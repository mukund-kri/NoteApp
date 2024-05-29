mod actions;
mod left_column;
mod messages;
mod scans;

use iced::widget::image::Image;
use iced::widget::{button, column, row};
use iced::{Alignment, Result as ICEDResult, Sandbox, Settings};

use actions::NoteDate;
use left_column::left_column;
use messages::Message;
use scans::Scan;

#[derive(Debug)]
struct NoteApp {
    index: usize,
    scans: Vec<Scan>,

    // for text input
    input: String,

    // Parsed date
    date: Option<NoteDate>,
}

impl Sandbox for NoteApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            index: 0,
            scans: Scan::populate_scans(),
            input: String::new(),
            date: None,
        }
    }

    fn title(&self) -> String {
        String::from("Note App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Previous => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
            Message::Next => {
                if self.index < self.scans.len() - 1 {
                    self.index += 1;
                }
            }
            Message::InputChanged(input) => {
                self.input = input;
                match NoteDate::validate(self.input.clone().as_str()) {
                    Ok(date) => {
                        self.date = Some(date);
                    }
                    Err(_) => {
                        self.date = None;
                    }
                }
            }
            Message::Delete => {
                match self.scans[self.index].delete() {
                    Ok(_) => {
                        // Inefficient but will do for now
                        self.scans = Scan::populate_scans();
                    }
                    Err(e) => {
                        println!("Error deleting scan: {}", e);
                    }
                }
            }
            Message::Post => {
                if let Some(date) = &self.date {
                    match self.scans[self.index].post(date) {
                        Ok(_) => {
                            self.scans = Scan::populate_scans();
                        }
                        Err(e) => {
                            println!("Error posting scan: {}", e);
                        }
                    }
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let scan = &self.scans[self.index];
        let image_path = scan.to_path().join("result.jpg");

        row![
            column![
                button("Previous").on_press(Message::Previous),
                Image::new(image_path)
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill),
                button("Next").on_press(Message::Next),
            ]
            .padding(20)
            .align_items(Alignment::Center),
            left_column(&self.input, scan.id.clone(), self.date.clone())
        ]
        .align_items(Alignment::Center)
        .into()
    }
}

fn main() -> ICEDResult {
    println!("Hello, world!");

    NoteApp::run(Settings::default())
}
