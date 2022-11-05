use super::app::{EffortMessage, WorkoutDesignerMessage};
use crate::gui::crm_creator::WorkoutMessage;
use crate::workout_data::positive_float::{InvalidPositiveFloatError, PositiveFloat};
use crate::workout_data::{effort, workout};
use iced::{
    container, scrollable, text_input, Alignment, Button, Color, Column, Element, Row, Text,
    TextInput,
};

#[derive(Debug, Clone)]
pub struct EffortUnitInput {
    padding: u16,
    size: u16,
    on_value_change: fn(String) -> WorkoutMessage,
    on_duration_change: fn(String) -> WorkoutMessage,
    creation_message: WorkoutMessage,
    effort: EffortInput,
    duration: DurationInput,
}

impl EffortUnitInput {
    fn current_effort(&self) -> String {
        self.effort.value.clone()
    }
    fn current_duration(&self) -> String {
        self.duration.value.clone()
    }
}

impl Default for EffortUnitInput {
    fn default() -> Self {
        Self::new(
            15,
            30,
            WorkoutMessage::from(WorkoutDesignerMessage::CreateTask),
            |efforts_string| {
                WorkoutMessage::from(WorkoutDesignerMessage::EffortUnitInputEffortChanged(
                    efforts_string,
                ))
            },
            |efforts_string| {
                WorkoutMessage::from(WorkoutDesignerMessage::EffortUnitInputDurationChanged(
                    efforts_string,
                ))
            },
        )
    }
}

impl EffortUnitInput {
    pub fn new(
        padding: u16,
        size: u16,
        creation_message: WorkoutMessage,
        on_value_change: fn(String) -> WorkoutMessage,
        on_duration_change: fn(String) -> WorkoutMessage,
    ) -> Self {
        Self {
            padding,
            size,
            creation_message,
            on_value_change,
            on_duration_change,
            effort: EffortInput::default(),
            duration: DurationInput::default(),
        }
    }
    pub fn set_effort(&mut self, effort: String) {
        self.effort.value = effort;
    }
    pub fn set_duration(&mut self, effort: String) {
        self.duration.value = effort;
    }

    pub fn is_empty(&self) -> bool {
        self.effort.value.is_empty() || self.duration.value.is_empty()
    }
    pub fn clear(&mut self) {
        self.effort.value.clear();
        self.duration.value.clear();
    }

    pub fn view(&mut self) -> Row<'_, WorkoutMessage> {
        Row::new()
            .spacing(10)
            .push(
                text_input::TextInput::new(
                    &mut self.duration.state,
                    "Duration",
                    &self.duration.value,
                    self.on_duration_change,
                )
                .padding(self.padding)
                .size(self.size)
                .on_submit(self.creation_message.clone()),
            )
            .push(
                text_input::TextInput::new(
                    &mut self.effort.state,
                    "Effort",
                    &self.effort.value,
                    self.on_value_change,
                )
                .padding(self.padding)
                .size(self.size)
                .on_submit(self.creation_message.clone()),
            )
    }
}

#[derive(Debug, Clone)]
pub struct EffortInput {
    state: text_input::State,
    value: String,
}

impl Default for EffortInput {
    fn default() -> Self {
        Self {
            state: text_input::State::default(),
            value: String::from(""),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DurationInput {
    state: text_input::State,
    value: String,
}

impl Default for DurationInput {
    fn default() -> Self {
        Self {
            state: text_input::State::default(),
            value: String::from(""),
        }
    }
}

impl TryFrom<EffortUnitInput> for effort::Effort {
    type Error = InvalidPositiveFloatError;
    fn try_from(effort_unit_input: EffortUnitInput) -> Result<Self, Self::Error> {
        Ok(effort::Effort::new(
            PositiveFloat::try_from(effort_unit_input.current_duration())?,
            PositiveFloat::try_from(effort_unit_input.current_effort())?,
            None,
        ))
    }
}

pub(super) fn base_design(title: &'_ str) -> Column<'_, WorkoutMessage> {
    Column::new()
        .align_items(Alignment::Center)
        .push(WhiteText::new(title).size(40))
}

struct WorkoutSectionDesign {}

impl container::StyleSheet for WorkoutSectionDesign {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: None,
            //Some(Background::from(Color::from_rgb8(41, 41, 41))),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from_rgb8(255, 255, 255),
        }
    }
}

impl<'a> workout::Workout {
    pub fn view(
        &'a mut self,
        scrollable_effort: &'a mut scrollable::State,
    ) -> impl Into<Element<'a, WorkoutMessage>> {
        container::Container::new(
            Column::new()
                .spacing(20)
                .push(WhiteText::new("Minutes   |   Value\n"))
                .push(
                    self.efforts.iter_mut().enumerate().fold(
                        scrollable::Scrollable::new(scrollable_effort)
                            .spacing(5)
                            .align_items(Alignment::End),
                        |scrollable, (effort_index, effort)| {
                            scrollable.push(effort.view(effort_index))
                        },
                    ),
                ),
        )
        .style(WorkoutSectionDesign {})
    }
}

impl<'a> effort::Effort {
    fn view(&'a mut self, effort_index: usize) -> impl Into<Element<'a, WorkoutMessage>> {
        match &mut self.gui_state {
            effort::EffortState::Idle {
                edit_button,
                delete_button,
            } => Row::new()
                .spacing(5)
                .push(
                    Row::new()
                        .spacing(10)
                        .push(WhiteText::new(self.duration_in_minutes.to_crm()))
                        .push(WhiteText::new(self.starting_value.to_crm())),
                )
                .push(Button::new(delete_button, Text::new("Delete")).on_press(
                    WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                        effort_index,
                        EffortMessage::Delete,
                    )),
                ))
                .push(Button::new(edit_button, Text::new("Edit")).on_press(
                    WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                        effort_index,
                        EffortMessage::Edit,
                    )),
                )),
            effort::EffortState::Editing {
                value_state,
                value,
                duration_in_minutes_state,
                duration_in_minutes,
            } => Row::new()
                .spacing(5)
                .push(
                    TextInput::new(
                        duration_in_minutes_state,
                        "",
                        duration_in_minutes,
                        move |updated_effort_in_minutes| {
                            WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                                effort_index,
                                EffortMessage::UpdateDurationInMinutes(updated_effort_in_minutes),
                            ))
                        },
                    )
                    .on_submit(WorkoutMessage::Design(
                        WorkoutDesignerMessage::Effort(
                            effort_index,
                            EffortMessage::ModificationDone,
                        ),
                    )),
                )
                .push(
                    TextInput::new(value_state, "", value, move |updated_value| {
                        WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                            effort_index,
                            EffortMessage::UpdateValue(updated_value),
                        ))
                    })
                    .on_submit(WorkoutMessage::Design(
                        WorkoutDesignerMessage::Effort(
                            effort_index,
                            EffortMessage::ModificationDone,
                        ),
                    )),
                ),
        }
    }
}

struct WhiteText {
    text: Text,
}

impl WhiteText {
    fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: Text::new(text).color(Color::WHITE).size(25),
        }
    }
    fn size(self, size: u16) -> Self {
        Self {
            text: self.text.size(size),
        }
    }
}

impl<'a> From<WhiteText> for Element<'a, WorkoutMessage> {
    fn from(white_text: WhiteText) -> Self {
        white_text.text.into()
    }
}
