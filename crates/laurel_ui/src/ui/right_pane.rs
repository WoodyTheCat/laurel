use iced::Task;
use laurel_common::Element;

#[derive(Debug, Default)]
pub struct RightPane;

#[derive(Debug, Clone)]
pub enum Message {}

impl RightPane {
    pub fn view(&self) -> Element<Message> {
        "Right".into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            _ => Task::none(),
        }
    }
}
