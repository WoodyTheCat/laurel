use iced::widget::text;
use iced::widget::text::LineHeight;
use laurel_common::Text;

macro_rules! icon_font {
    [$($name:ident,$code:expr;)+] => {
        $(
            pub fn $name<'a>() -> Text<'a> {
                to_text($code)
            }
        )+

    };
}

icon_font![
    bold, '\u{E061}';
    italic, '\u{E101}';
    underline, '\u{E19A}';
    heading, '\u{E388}';
    ul, '\u{E10C}';
    ol, '\u{E1D1}';
    sigma, '\u{E201}';
    code, '\u{E097}';
    files, '\u{E0D3}';
    save_all, '\u{E414}';
    map, '\u{E114}';
];

fn to_text<'a>(unicode: char) -> Text<'a> {
    text(unicode.to_string())
        .line_height(LineHeight::Relative(1.0))
        .size(laurel_common::ICON_SIZE)
        .font(laurel_common::fonts::ICON)
}
