use crate::gui::mrc_creator::WorkoutMessage;
use iced::theme::palette::EXTENDED_DARK;
use iced::{
    widget::{button, text, text_input, Text},
    Element,
};
use iced::{Background, Color, Vector};
use iced_core::{Border, Length};

pub const TEXT_SIZE: f32 = 22.0;
pub const LARGE_BUTTON: Length = Length::Fixed(150.0);
pub const SMALL_BUTTON: Length = Length::Fixed(70.0);

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
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::BLACK,
            ..button::Appearance::default()
        }
    }
    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance::default()
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border {
                color: PURPLE,
                width: 5.0,
                radius: 5.0.into(),
            },
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
    button::Button::new(
        Text::new(text)
            .size(20.0)
            .horizontal_alignment(iced_core::alignment::Horizontal::Center)
            .vertical_alignment(iced_core::alignment::Vertical::Center),
    )
    .style(iced::theme::Button::Custom(Box::new(PinkRetroButton {})))
    .width(LARGE_BUTTON)
}

fn active_border() -> Border {
    Border {
        radius: 2.0.into(),
        width: 1.0,
        color: PINK,
    }
}

struct RetroPinkTextInput {}

impl text_input::StyleSheet for RetroPinkTextInput {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: EXTENDED_DARK.background.base.color.into(),
            border: active_border(),
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
            border: Border {
                color: PURPLE,
                ..active_border()
            },
            ..self.active(style)
        }
    }
    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: PURPLE,
                ..active_border()
            },
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
                .size(TEXT_SIZE)
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
