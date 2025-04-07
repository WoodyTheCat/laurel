use iced::widget::text_input::{Catalog, Status, Style, StyleFn};
use iced::{Border, Color};

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

pub fn default(theme: &LaurelTheme, _status: Status) -> Style {
    Style {
        background: iced::Background::Color(theme.base),
        value: theme.text,
        icon: theme.text,
        border: Border::default(),
        placeholder: Color::BLACK,
        selection: Color::WHITE,
    }
}
