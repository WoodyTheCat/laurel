pub mod appearance;

pub use appearance::LaurelTheme;

pub type Element<'a, Message, Renderer = iced::Renderer, Theme = LaurelTheme> =
    iced::Element<'a, Message, Theme, Renderer>;

pub type Canvas<P, Message, Renderer = iced::Renderer, Theme = LaurelTheme> =
    iced::widget::Canvas<P, Message, Theme, Renderer>;

pub type Text<'a, Renderer = iced::Renderer, Theme = LaurelTheme> =
    iced::widget::Text<'a, Theme, Renderer>;

pub const ICON_SIZE: f32 = 14.0;
pub const ICON_SIZE_MID: f32 = 24.0;
pub const ICON_SIZE_LARGE: f32 = 32.0;

pub mod fonts {
    use iced::Font;
    use std::borrow::Cow;

    pub const ICON: Font = Font::with_name("lucide");

    pub fn load() -> Vec<Cow<'static, [u8]>> {
        vec![
            // include_bytes!("../fonts/iosevka-term-regular.ttf")
            //     .as_slice()
            //     .into(),
            // include_bytes!("../fonts/iosevka-term-bold.ttf")
            //     .as_slice()
            //     .into(),
            // include_bytes!("../fonts/iosevka-term-italic.ttf")
            //     .as_slice()
            //     .into(),
            include_bytes!("../../../data/lucide.ttf").as_slice().into(),
        ]
    }
}
