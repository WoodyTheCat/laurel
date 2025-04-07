use iced::{
    Background, Border, Color,
    widget::button::{Catalog, Status, Style, StyleFn},
};

use crate::LaurelTheme;

impl Catalog for LaurelTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn menu(_theme: &LaurelTheme, status: Status) -> Style {
    Style {
        background: Some(Background::from(if status == Status::Hovered {
            Color::from_rgb8(70, 77, 89)
        } else {
            Color::from_rgb8(25, 31, 43)
        })),
        text_color: Color::WHITE,
        ..Default::default()
    }
}

pub fn default(_theme: &LaurelTheme, _status: Status) -> Style {
    Style {
        ..Default::default()
    }
}

pub fn transparent(theme: &LaurelTheme, status: Status) -> Style {
    Style {
        text_color: match status {
            Status::Pressed => theme.accent,
            Status::Disabled => theme.muted,
            _ => theme.text,
        },
        background: Some(
            match status {
                Status::Hovered => theme.highlight_low,
                Status::Pressed => theme.highlight_med,
                _ => Color::TRANSPARENT,
            }
            .into(),
        ),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
