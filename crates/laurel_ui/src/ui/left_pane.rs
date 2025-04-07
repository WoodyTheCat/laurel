use iced::{
    Length, Padding, Task,
    widget::{Button, Container, Rule, center, column, container, row},
};
use laurel_common::{Element, Text, appearance};

use crate::ui::icon;

#[derive(Debug, Default)]
pub struct LeftPane;

#[derive(Debug, Clone)]
pub enum Message {
    Debug(String),
}

impl LeftPane {
    pub fn view(&self) -> Element<Message> {
        let toolbar = container(self.toolbar()).height(32);

        column![
            toolbar,
            Container::new("A")
                .padding(16.0)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(appearance::container::floating),
        ]
        .spacing(8.0)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Debug(s) => {
                println!("{}", s);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    // UI

    fn toolbar(&self) -> Element<Message> {
        fn button_trans(content: Text, on_press: Message) -> Element<Message> {
            Button::new(center(content).style(appearance::container::transparent))
                .width(34)
                .height(28.8)
                .padding(4)
                .on_press(on_press)
                .style(appearance::button::transparent)
                .into()
        }

        row![
            container(
                row![
                    button_trans(icon::files(), Message::Debug("1".into())),
                    Rule::vertical(1),
                    button_trans(icon::save_all(), Message::Debug("2".into())),
                    Rule::vertical(1),
                    button_trans(icon::map(), Message::Debug("3".into())),
                ]
                .padding(Padding::new(1.6))
            )
            .style(appearance::container::floating),
            container(
                row![
                    button_trans(icon::bold(), Message::Debug("1".into())),
                    Rule::vertical(1),
                    button_trans(icon::italic(), Message::Debug("2".into())),
                    Rule::vertical(1),
                    button_trans(icon::underline(), Message::Debug("3".into())),
                ]
                .padding(Padding::new(1.6))
            )
            .style(appearance::container::floating),
            container(
                row![
                    button_trans(icon::heading(), Message::Debug("1".into())),
                    Rule::vertical(1),
                    button_trans(icon::ul(), Message::Debug("2".into())),
                    Rule::vertical(1),
                    button_trans(icon::ol(), Message::Debug("3".into())),
                    Rule::vertical(1),
                    button_trans(icon::sigma(), Message::Debug("4".into())),
                    Rule::vertical(1),
                    button_trans(icon::code(), Message::Debug("5".into())),
                ]
                .padding(Padding::new(1.6))
            )
            .style(appearance::container::floating),
        ]
        .spacing(8.0)
        .into()
    }
}
