use iced::{
    Border, Color,
    widget::scrollable::{Catalog, Rail, Scroller, Status, Style, StyleFn},
};

use crate::LaurelTheme;

use super::container;

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
        container: container::normal(theme),
        gap: None,
        vertical_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                color: Color::BLACK,
                border: Border::default(),
            },
        },
        horizontal_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                color: Color::BLACK,
                border: Border::default(),
            },
        },
    }
}
