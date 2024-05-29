use iced::widget::text_input::TextInput;
use iced::widget::{button, column, text, Column};
use iced::Alignment;

use crate::actions::NoteDate;
use crate::messages::Message;

pub fn left_column<'a>(value: &str, scan_id: String, date: Option<NoteDate>) -> Column<Message> {
    let input_label = "Enter date with format YYYY.MM.DD here ...";
    let input = TextInput::new("Enter date here", value)
        .on_input(Message::InputChanged)
        .padding(10);

    let delete_button = button("Delete").on_press(Message::Delete);

    let id_text = text(format!("ID: {}", scan_id));

    column![
        id_text,
        date_block(date.clone()),
        input_label,
        input,
        delete_button
    ]
    .spacing(20)
    .padding(20)
    .align_items(Alignment::Center)
    .into()
}

fn date_block(date: Option<NoteDate>) -> Column<'static, Message> {
    let post_button: button::Button<'_, Message, iced::Theme, iced::Renderer> =
        button("Post").on_press(Message::Post);

    match date {
        Some(dt) => column![
            text(format!("Year: {}", dt.year)),
            text(format!("Month: {}", dt.month.unwrap_or(0))),
            text(format!("Day: {}", dt.day.unwrap_or(0))),
            text(format!("Path: {}", dt.to_path().to_string_lossy())),
            post_button,
        ],
        None => column![text("Invalid date"),],
    }
}
