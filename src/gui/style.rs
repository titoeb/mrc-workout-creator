use crate::gui::mrc_creator::WorkoutMessage;
use iced::theme::palette::EXTENDED_DARK;
use iced::{
    widget::{button, text_input, Text},
    Element,
};
use iced::{Background, Color, Font, Theme};
use iced_core::{Border, Length};

pub const TEXT_SIZE: f32 = 22.0;
pub const LARGE_BUTTON: Length = Length::Fixed(180.0);
pub const SMALL_BUTTON: f32 = 90.0;

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

pub fn default_font() -> Font {
    Font {
        family: iced::font::Family::Monospace,
        ..Default::default()
    }
}

pub fn text_with_default_font<'a>(text: String) -> Text<'a> {
    Text::new(text).font(default_font())
}

struct PinkRetroButton {}
impl PinkRetroButton {
    pub fn style(_theme: &Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => PinkRetroButton::active(),
            button::Status::Pressed => PinkRetroButton::pressed(),
            button::Status::Hovered => PinkRetroButton::hovered(),
            button::Status::Disabled => PinkRetroButton::disabled(),
        }
    }
    fn active() -> button::Style {
        button::Style {
            // shadow_offset: Vector::default(),
            background: Some(Background::Color(PINK)),
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::BLACK,
            ..button::Style::default()
        }
    }
    fn disabled() -> button::Style {
        button::Style::default()
    }
    fn hovered() -> button::Style {
        button::Style {
            border: Border {
                color: PURPLE,
                width: 5.0,
                radius: 5.0.into(),
            },
            ..PinkRetroButton::active()
        }
    }
    fn pressed() -> button::Style {
        button::Style {
            // shadow_offset: Vector { x: 5.0, y: 5.0 },
            background: Some(Background::Color(PURPLE)),
            ..PinkRetroButton::active()
        }
    }
}

pub(crate) fn pink_button(text: &str) -> button::Button<'_, WorkoutMessage> {
    button::Button::new(
        text_with_default_font(String::from(text))
            .size(19.0)
            .align_x(iced_core::alignment::Horizontal::Center)
            .align_y(iced_core::alignment::Vertical::Center),
    )
    .style(PinkRetroButton::style)
}

fn active_border() -> Border {
    Border {
        radius: 2.0.into(),
        width: 1.0,
        color: PINK,
    }
}

struct PinkRetroTextInput {}

impl PinkRetroTextInput {
    pub fn style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
        match status {
            text_input::Status::Active => PinkRetroTextInput::active(),
            text_input::Status::Focused => PinkRetroTextInput::focused(),
            text_input::Status::Hovered => PinkRetroTextInput::hovered(),
            text_input::Status::Disabled => PinkRetroTextInput::disabled(),
        }
    }

    fn active() -> text_input::Style {
        text_input::Style {
            background: EXTENDED_DARK.background.base.color.into(),
            border: active_border(),
            icon: WHITE,
            placeholder: PinkRetroTextInput::placeholder_color(),
            value: PinkRetroTextInput::value_color(),
            selection: PinkRetroTextInput::selection_color(),
        }
    }
    fn disabled() -> text_input::Style {
        text_input::Style {
            ..PinkRetroTextInput::active()
        }
    }
    fn focused() -> text_input::Style {
        text_input::Style {
            border: Border {
                color: PURPLE,
                ..active_border()
            },
            ..PinkRetroTextInput::active()
        }
    }
    fn hovered() -> text_input::Style {
        text_input::Style {
            border: Border {
                color: PURPLE,
                ..active_border()
            },
            ..PinkRetroTextInput::active()
        }
    }
    fn placeholder_color() -> Color {
        LIGHT_WHITE
    }
    fn selection_color() -> Color {
        GRAY
    }
    fn value_color() -> Color {
        WHITE
    }
}

pub fn pink_text_input<'a>(
    placeholder: &'a str,
    value: &'a str,
) -> text_input::TextInput<'a, WorkoutMessage> {
    text_input::TextInput::new(placeholder, value)
        .font(default_font())
        .style(PinkRetroTextInput::style)
}

pub struct WhiteText<'a> {
    text: Text<'a>,
}

impl WhiteText<'_> {
    pub fn new(white_text: String) -> Self {
        Self {
            text: text_with_default_font(white_text)
                .size(TEXT_SIZE)
                .color(WHITE),
        }
    }
    pub fn width(self, new_width: u16) -> Self {
        Self {
            text: self.text.width(new_width),
        }
    }
    pub fn align_x(self, alignement: iced::alignment::Horizontal) -> Self {
        Self {
            text: self.text.align_x(alignement),
        }
    }
}

impl<'a> From<WhiteText<'a>> for Element<'a, WorkoutMessage> {
    fn from(white_text: WhiteText<'a>) -> Self {
        white_text.text.into()
    }
}
