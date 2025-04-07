use crate::LaurelTheme;
use iced::{
    Background, Color,
    widget::container::{self, Catalog, Style},
};

impl Catalog for LaurelTheme {
    type Class<'a> = container::StyleFn<'a, Self>;

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        class(self)
    }

    fn default<'a>() -> Self::Class<'a> {
        Box::new(normal)
    }
}

pub fn normal(theme: &LaurelTheme) -> Style {
    Style {
        background: Some(theme.base.into()),
        text_color: Some(theme.text),
        ..Default::default()
    }
}

pub fn menu(theme: &LaurelTheme) -> Style {
    Style {
        background: Some(theme.overlay.into()),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}
pub fn floating(theme: &LaurelTheme) -> Style {
    Style {
        background: Some(theme.surface.into()),
        border: theme.border_med,
        text_color: Some(theme.text),
        ..Default::default()
    }
}

pub fn saved(_theme: &LaurelTheme) -> Style {
    Style {
        background: Some(Background::from(Color::from_rgba8(25, 31, 43, 1.0))),
        text_color: Some(Color::from_rgb8(130, 130, 130)),
        ..Default::default()
    }
}

pub fn transparent(theme: &LaurelTheme) -> Style {
    Style {
        background: Some(Color::TRANSPARENT.into()),
        text_color: Some(theme.text),
        ..Default::default()
    }
}
