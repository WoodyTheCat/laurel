use crate::LaurelTheme;
use iced::widget::rule::{self, Catalog, Style};

impl Catalog for LaurelTheme {
    type Class<'a> = rule::StyleFn<'a, Self>;

    fn style(&self, class: &Self::Class<'_>) -> rule::Style {
        class(self)
    }

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }
}

pub fn default(theme: &LaurelTheme) -> Style {
    Style {
        fill_mode: rule::FillMode::Padded(6),
        radius: 0.0.into(),
        width: 1,
        color: theme.highlight_med,
    }
}
