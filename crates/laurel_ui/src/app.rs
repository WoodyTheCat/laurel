use iced::{
    Length, Settings, Task,
    widget::{container, row},
};
use laurel_common::Element;

use crate::ui::{LeftPane, RightPane, left_pane, right_pane};

pub struct Laurel {
    left_pane: LeftPane,
    right_pane: RightPane,
}

#[derive(Debug, Clone)]
pub enum Message {
    Left(left_pane::Message),
    Right(right_pane::Message),
}

impl Laurel {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                left_pane: LeftPane::default(),
                right_pane: RightPane::default(),
            },
            Task::none(),
        )
    }

    pub fn settings() -> Settings {
        Settings {
            fonts: laurel_common::fonts::load(),
            antialiasing: false,
            id: Some(String::from("laurel")),
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Left(m) => self.left_pane.update(m).map(Message::Left),
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            container(self.left_pane.view().map(Message::Left)).width(Length::FillPortion(2)),
            container(self.right_pane.view().map(Message::Right)).width(Length::FillPortion(3))
        ]
        .spacing(8.0)
        .padding(8.0)
        .into()
    }
}
