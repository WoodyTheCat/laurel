pub mod button;
pub mod container;
pub mod rule;
pub mod scrollable;
pub mod text;
pub mod text_input;

use iced::{Border, Color};

#[derive(Debug, Clone)]
pub struct LaurelTheme {
    pub text: Color,
    pub subtle: Color,
    pub muted: Color,
    pub base: Color,
    pub surface: Color,
    pub overlay: Color,
    pub border_low: Border,
    pub border_med: Border,
    pub border_hi: Border,
    pub highlight_low: Color,
    pub highlight_med: Color,
    pub highlight_high: Color,

    pub accent: Color,
}

impl Default for LaurelTheme {
    fn default() -> Self {
        Self {
            text: Color::from_rgb8(87, 82, 121),
            subtle: Color::from_rgb8(121, 117, 147),
            muted: Color::from_rgb8(152, 147, 165),
            base: Color::from_rgb8(250, 244, 237),
            surface: Color::from_rgb8(255, 250, 243),
            overlay: Color::from_rgb8(242, 233, 225),
            border_low: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: Color::from_rgb8(244, 237, 232),
            },
            border_med: Border {
                radius: 6.0.into(),
                width: 1.5,
                color: Color::from_rgb8(223, 218, 217),
            },
            border_hi: Border {
                radius: 8.0.into(),
                width: 2.0,
                color: Color::from_rgb8(244, 237, 232),
            },
            highlight_low: Color::from_rgb8(244, 237, 232),
            highlight_med: Color::from_rgb8(223, 218, 217),
            highlight_high: Color::from_rgb8(244, 237, 232),
            accent: Color::from_rgb8(144, 122, 169),
        }
    }
}

impl iced::theme::Base for LaurelTheme {
    fn base(&self) -> iced::theme::Style {
        iced::theme::Style {
            background_color: self.base,
            text_color: self.text,
        }
    }

    fn palette(&self) -> Option<iced::theme::Palette> {
        None
    }
}
