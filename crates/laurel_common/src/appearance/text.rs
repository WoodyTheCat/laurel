use iced::widget::text::{Catalog, Style, StyleFn};

use crate::LaurelTheme;

impl Catalog for LaurelTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &LaurelTheme) -> Style {
    Style {
        color: Some(theme.text),
    }
}
