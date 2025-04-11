use iced::{
    widget::{button, column, container, row, text},
    Length, Padding, Pixels,
};

use laurel_common::{appearance, Element};

use crate::{widgets::main_view::MainView, Message};

pub fn layout<'a>(
    child: Element<'a, Message>,
    modal: Option<Element<'a, Message>>,
    is_saved: bool,
) -> Element<'a, Message> {
    // return "Layout".into();
    column!(
        container(navbar(is_saved))
            .width(Length::Fill)
            .style(appearance::container::menu),
        main_view(child, modal),
    )
    .into()
}

fn navbar(is_saved: bool) -> Element<'static, Message> {
    let padding = Padding {
        top: 7.0,
        left: 12.0,
        bottom: 10.0,
        right: 12.0,
    };
    let mut row = row!(
        button(text("Open File").size(Pixels::from(14.0)))
            .style(appearance::button::menu)
            .on_press(Message::SelectFile)
            .padding(padding),
        button(text("Open Folder").size(Pixels::from(14.0)))
            .style(appearance::button::menu)
            .padding(padding)
            .on_press(Message::SelectFolder),
        button(text("Save").size(Pixels::from(14.0)))
            .style(appearance::button::menu)
            .padding(padding)
            .on_press(Message::Save),
    )
    .padding(Padding {
        right: 15.0,
        ..Default::default()
    })
    .align_y(iced::Alignment::Start);

    if !is_saved {
        row = row.push(
            container(text("Unsaved Changes").size(10.0))
                .style(appearance::container::saved)
                .padding(Padding {
                    top: 12.0,
                    left: 12.0,
                    bottom: 7.0,
                    right: 40.0,
                }),
        )
    }
    row.into()
}

fn main_view<'a>(
    child: Element<'a, Message>,
    modal: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    MainView::new(child, modal).into()
}
