use iced::widget::{button, column, image::Image, row};
use iced::Command;
use iced::{executor::Default as IceExecutor, Alignment, Application, Theme};
use log::info;

use crate::actions::NoteDate;
use crate::left_column::left_column;
use crate::messages::Message;
use crate::{scans::Scan, settings::Paths};

#[derive(Debug)]
pub struct NoteApp {
    index: usize,
    scans: Vec<Scan>,

    // for text input
    input: String,

    // Parsed date
    date: Option<NoteDate>,

    // Paths
    paths: Paths,
}

/// Config for the application. Right now it only holds the paths

impl Application for NoteApp {
    type Message = Message;
    type Executor = IceExecutor;
    type Flags = Paths;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        // log flags
        info!("Flags: {:?}", flags);

        let mut app = Self {
            index: 0,
            scans: Vec::new(),
            input: String::new(),
            date: None,
            paths: flags.clone(),
        };
        app.scans = Scan::populate_scans(&app.paths);
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Note App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
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
                match self.scans[self.index].delete(&self.paths) {
                    Ok(_) => {
                        // Inefficient but will do for now
                        self.scans = Scan::populate_scans(&self.paths);
                    }
                    Err(e) => {
                        println!("Error deleting scan: {}", e);
                    }
                }
            }
            Message::Post => {
                if let Some(date) = &self.date {
                    match self.scans[self.index].post(date, &self.paths) {
                        Ok(_) => {
                            self.scans = Scan::populate_scans(&self.paths);
                        }
                        Err(e) => {
                            println!("Error posting scan: {}", e);
                        }
                    }
                }
            }
        };
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let scan = &self.scans[self.index];
        let image_path = scan.to_path(&self.paths).join("result.jpg");

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
