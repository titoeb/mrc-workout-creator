use crate::gui::mrc_creator::WorkoutMessage;
use iced::theme::palette::EXTENDED_DARK;
use iced::{
    widget::{button, text, text_input, Text},
    Element,
};
use iced_native::{Background, Color, Vector};

pub const PINK: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 1.0,
    a: 1.0,
};
pub const PURPLE: Color = Color {
    r: 171.0 / 255.0,
    g: 32.0 / 255.0,
    b: 253.0 / 255.0,
    a: 1.0,
};
pub const GRAY: Color = Color {
    r: 40.0 / 255.0,
    g: 50.0 / 255.0,
    b: 52.0 / 255.0,
    a: 1.0,
};
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.8,
};
pub const LIGHT_WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.08,
};

struct PinkRetroButton {}

impl button::StyleSheet for PinkRetroButton {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(PINK)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
        }
    }
    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance::default()
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_color: PURPLE,
            border_width: 5.0,
            border_radius: 5.0,
            ..self.active(style)
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 5.0, y: 5.0 },
            background: Some(Background::Color(PURPLE)),
            ..self.active(style)
        }
    }
}

pub(crate) fn pink_button(text: &str) -> button::Button<'_, WorkoutMessage> {
    button::Button::new(Text::new(text))
        .style(iced::theme::Button::Custom(Box::new(PinkRetroButton {})))
}

struct RetroPinkTextInput {}

impl text_input::StyleSheet for RetroPinkTextInput {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: EXTENDED_DARK.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: PINK,
            icon_color: WHITE,
        }
    }
    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self.active(style)
        }
    }
    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border_color: PURPLE,
            ..self.active(style)
        }
    }
    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border_color: PURPLE,
            ..self.active(style)
        }
    }
    fn disabled_color(&self, _style: &Self::Style) -> Color {
        GRAY
    }
    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        LIGHT_WHITE
    }
    fn selection_color(&self, _style: &Self::Style) -> Color {
        GRAY
    }
    fn value_color(&self, _style: &Self::Style) -> Color {
        WHITE
    }
}

pub fn pink_text_input<'a>(
    placeholder: &'a str,
    value: &'a str,
) -> text_input::TextInput<'a, WorkoutMessage> {
    text_input::TextInput::new(placeholder, value).style(iced::theme::TextInput::Custom(Box::new(
        RetroPinkTextInput {},
    )))
}

pub struct WhiteText<'a> {
    text: Text<'a>,
}

impl WhiteText<'_> {
    pub fn new(white_text: String) -> Self {
        Self {
            text: text(white_text)
                .size(25)
                .style(iced::theme::Text::Color(WHITE)),
        }
    }
    pub fn width(self, new_width: u16) -> Self {
        Self {
            text: self.text.width(new_width),
        }
    }
    pub fn horizontal_alignment(self, alignement: iced::alignment::Horizontal) -> Self {
        Self {
            text: self.text.horizontal_alignment(alignement),
        }
    }
}

impl<'a> From<WhiteText<'a>> for Element<'a, WorkoutMessage> {
    fn from(white_text: WhiteText<'a>) -> Self {
        white_text.text.into()
    }
}
