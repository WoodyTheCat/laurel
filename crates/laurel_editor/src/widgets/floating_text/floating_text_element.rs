use iced::widget::{
    column, container,
    scrollable::{self, Scrollbar},
    text,
};
use laurel_common::{appearance, Element};

use crate::Message;
use laurel_common::text::Position;

#[derive(Clone, Debug)]
pub struct FloatingElement {
    pub view_box: FloatingText,
    pub position: Position,
}
#[derive(Clone, Debug)]
pub enum FloatingText {
    Diagnostic(String),
}

impl FloatingText {
    pub fn show(&self) -> Element<Message> {
        match self {
            FloatingText::Diagnostic(value) => {
                let mut text_lines = Vec::new();
                for line in value.lines() {
                    let text = text(line).into();
                    text_lines.push(text);
                }

                container(
                    iced::widget::scrollable(column(text_lines).width(iced::Length::Fill))
                        .direction(scrollable::Direction::Vertical(
                            Scrollbar::default().scroller_width(7.0).width(7.0),
                        )),
                )
                .padding(20)
                .style(appearance::container::floating)
                .max_height(400)
                .max_width(500)
                .into()
            }
        }
    }
}
