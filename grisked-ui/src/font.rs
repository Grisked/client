use iced::{
    alignment,
    widget::{text, Text},
    Font, Length,
};

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../fonts/NotoEmoji-Regular.ttf"),
};

pub fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20)
}

pub enum FontFamily {
    Kanit,
    IndieFlower,
}

impl FontFamily {
    pub fn get_path(&self) -> String {
        match self {
            Self::Kanit => "Kanit".to_string(),
            Self::IndieFlower => "IndieFlower".to_string(),
        }
    }
}

pub enum FontSize {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontSize {
    pub fn from_code(code: u16) -> Self {
        match code {
            100 => Self::Thin,
            200 => Self::ExtraLight,
            300 => Self::Light,
            400 => Self::Regular,
            500 => Self::Medium,
            600 => Self::SemiBold,
            700 => Self::Bold,
            800 => Self::ExtraBold,
            900 => Self::Black,
            _ => Self::Regular,
        }
    }

    pub fn get_font(&self, family: FontFamily) -> Font {
        match family {
            FontFamily::Kanit => match self {
                Self::Thin => Font::External {
                    name: "Kanit-Thin",
                    bytes: include_bytes!("../../fonts/Kanit/100.ttf"),
                },
                Self::ExtraLight => Font::External {
                    name: "Kanit-ExtraLight",
                    bytes: include_bytes!("../../fonts/Kanit/200.ttf"),
                },
                Self::Light => Font::External {
                    name: "Kanit-Light",
                    bytes: include_bytes!("../../fonts/Kanit/300.ttf"),
                },
                Self::Regular => Font::External {
                    name: "Kanit-Regular",
                    bytes: include_bytes!("../../fonts/Kanit/400.ttf"),
                },
                Self::Medium => Font::External {
                    name: "Kanit-Medium",
                    bytes: include_bytes!("../../fonts/Kanit/500.ttf"),
                },
                Self::SemiBold => Font::External {
                    name: "Kanit-SemiBold",
                    bytes: include_bytes!("../../fonts/Kanit/600.ttf"),
                },
                Self::Bold => Font::External {
                    name: "Kanit-Bold",
                    bytes: include_bytes!("../../fonts/Kanit/700.ttf"),
                },
                Self::ExtraBold => Font::External {
                    name: "Kanit-ExtraBold",
                    bytes: include_bytes!("../../fonts/Kanit/800.ttf"),
                },
                Self::Black => Font::External {
                    name: "Kanit-Black",
                    bytes: include_bytes!("../../fonts/Kanit/900.ttf"),
                },
            },
            FontFamily::IndieFlower => match self {
                _ => Font::External {
                    name: "IndieFlower-Regular",
                    bytes: include_bytes!("../../fonts/IndieFlower/400.ttf"),
                },
            },
        }
    }
}

pub enum FontType {
    Header,
    Title,
    TextBold,
    Text,
}

impl FontType {
    pub fn get_text(&self, value: String, family: FontFamily) -> Text<'static> {
        let font = self.get_font(family);

        text(value).font(font)
    }

    fn get_font(&self, family: FontFamily) -> Font {
        match self {
            Self::Header => FontSize::SemiBold.get_font(family),
            Self::Title => FontSize::Medium.get_font(family),
            Self::TextBold => FontSize::Regular.get_font(family),
            Self::Text => FontSize::Light.get_font(family),
        }
    }
}
