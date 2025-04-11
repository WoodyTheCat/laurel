use crate::Message;
use iced::widget::scrollable::{Id, Scrollbar};
use iced::widget::{column, container, scrollable, text};
use iced::{alignment, Background, Border, Color, Length, Padding, Pixels};
use laurel_common::{Element, LaurelTheme};

pub mod floating_text;
pub mod layout;
pub mod main_view;
pub mod modal;
pub mod textbox;
pub mod textbox_container;
pub mod view_port;

fn code_line_style(_style: &LaurelTheme) -> container::Style {
    container::Style {
        background: Some(Background::from(Color::TRANSPARENT)),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::BLACK,
        },
        text_color: Some(Color::from_rgb8(153, 153, 153)),
        ..Default::default()
    }
}

pub fn line_number(
    number_of_lines: usize,
    font_size: f32,
    height: f32,
    id: Id,
) -> Element<'static, Message> {
    let mut lines: Vec<Element<'static, Message>> = Vec::new();
    let box_height = text::LineHeight::default().to_absolute(Pixels(font_size)).0;
    for i in 1..number_of_lines.saturating_add(1) {
        let padding = Padding {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 5.0,
        };

        let container = container(text(i).size(font_size))
            .center_x(Length::Shrink)
            .align_y(alignment::Vertical::Top)
            .width(Length::Fixed(80.0))
            .padding(padding)
            .height(box_height);
        lines.push(container.into())
    }
    scrollable(
        container(column(lines))
            .style(code_line_style)
            .height(height),
    )
    .id(id)
    .direction(scrollable::Direction::Vertical(
        Scrollbar::default().scroller_width(0.0).width(0.0),
    ))
    .height(Length::Fill)
    .into()
}
