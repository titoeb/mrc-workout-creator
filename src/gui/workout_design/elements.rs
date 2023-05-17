use std::num::ParseFloatError;

use super::app::{EffortMessage, WorkoutDesignerMessage};
use crate::gui::mrc_creator::WorkoutMessage;
use crate::workout_data::ToMRC;
use crate::workout_data::{effort, workout};
use iced::widget::{container, scrollable, text, text_input, Button, Column, Row, Text, TextInput};
use iced::{Alignment, Element};

#[derive(Debug, Clone)]
pub struct EffortUnitInput {
    padding: u16,
    size: u16,
    on_starting_value_change: fn(String) -> WorkoutMessage,
    on_ending_value_change: fn(String) -> WorkoutMessage,
    on_duration_change: fn(String) -> WorkoutMessage,
    creation_message: WorkoutMessage,
    effort: EffortInput,
    duration: DurationInput,
}

impl EffortUnitInput {
    fn starting_value(&self) -> String {
        self.effort.starting_value.clone()
    }
    fn ending_value(&self) -> String {
        self.effort.ending_value.clone()
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
                WorkoutMessage::from(WorkoutDesignerMessage::EffortUnitStartingValueChanged(
                    efforts_string,
                ))
            },
            |efforts_string| {
                WorkoutMessage::from(WorkoutDesignerMessage::EffortUnitEndingValueChanged(
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
        on_starting_value_change: fn(String) -> WorkoutMessage,
        on_ending_value_change: fn(String) -> WorkoutMessage,
        on_duration_change: fn(String) -> WorkoutMessage,
    ) -> Self {
        Self {
            padding,
            size,
            creation_message,
            on_starting_value_change,
            on_ending_value_change,
            on_duration_change,
            effort: EffortInput::default(),
            duration: DurationInput::default(),
        }
    }
    pub fn set_starting_value(&mut self, starting_value: String) {
        self.effort.starting_value = starting_value;
    }
    pub fn set_ending_value(&mut self, ending_value: String) {
        self.effort.ending_value = ending_value;
    }

    pub fn set_duration(&mut self, effort: String) {
        self.duration.value = effort;
    }

    pub fn is_empty(&self) -> bool {
        self.effort.starting_value.is_empty() || self.duration.value.is_empty()
    }
    pub fn clear(&mut self) {
        self.effort.starting_value.clear();
        self.effort.ending_value.clear();
        self.duration.value.clear();
    }

    pub fn view(&self) -> Row<'_, WorkoutMessage> {
        Row::new()
            .spacing(10)
            .push(
                text_input::TextInput::new("Duration", &self.duration.value)
                    .padding(self.padding)
                    .size(self.size)
                    .on_submit(self.creation_message.clone())
                    .on_input(self.on_duration_change),
            )
            .push(
                text_input::TextInput::new("Starting Value", &self.effort.starting_value)
                    .padding(self.padding)
                    .size(self.size)
                    .on_submit(self.creation_message.clone())
                    .on_input(self.on_starting_value_change),
            )
            .push(
                text_input::TextInput::new("Ending Value", &self.effort.ending_value)
                    .padding(self.padding)
                    .size(self.size)
                    .on_submit(self.creation_message.clone())
                    .on_input(self.on_ending_value_change),
            )
    }
}

#[derive(Debug, Clone, Default)]
pub struct EffortInput {
    starting_value: String,
    ending_value: String,
}

#[derive(Debug, Clone, Default)]
pub struct DurationInput {
    value: String,
}

impl TryFrom<EffortUnitInput> for effort::Effort {
    type Error = ParseFloatError;
    fn try_from(effort_unit_input: EffortUnitInput) -> Result<Self, Self::Error> {
        Ok(effort::Effort::new(
            effort_unit_input.current_duration().parse()?,
            effort_unit_input.starting_value().parse()?,
            if effort_unit_input.ending_value().is_empty() {
                None
            } else {
                Some(effort_unit_input.ending_value().parse()?)
            },
        ))
    }
}

pub(super) fn base_design<'a>() -> Column<'a, WorkoutMessage> {
    Column::new()
        .align_items(Alignment::Center)
        .padding(10)
        .spacing(30)
}

impl<'a> workout::Workout {
    pub fn view(&'a self) -> impl Into<Element<'a, WorkoutMessage>> {
        let workout_duration = self.workout_duration();
        let average_intensity = self.average_intensity();
        container::Container::new(
            Column::new()
                .spacing(20)
                .push(WhiteText::new(String::from(
                    "Minutes   |   Starting-value | Ending Value\n",
                )))
                .push(scrollable(
                    self.efforts.iter().enumerate().fold(
                        Column::new(),
                        |scrollable, (effort_index, effort)| {
                            scrollable.push(effort.view(effort_index))
                        },
                    ),
                ))
                .push(WhiteText::new(
                    format!("Duration: {:.1}", workout_duration,),
                ))
                .push(WhiteText::new(format!(
                    "Average Intensity: {:.1}",
                    average_intensity,
                ))),
        )
    }
}

impl<'a> effort::Effort {
    fn view(&'a self, effort_index: usize) -> impl Into<Element<'a, WorkoutMessage>> {
        match &self.gui_state {
            effort::EffortState::Idle => Row::new()
                .spacing(5)
                .push(
                    Row::new()
                        .spacing(10)
                        .push(WhiteText::new(self.duration_in_minutes.to_mrc()))
                        .push(WhiteText::new(self.starting_value.to_mrc()))
                        .push(WhiteText::new(self.ending_value.to_mrc())),
                )
                .push(Button::new(text("Delete")).on_press(WorkoutMessage::Design(
                    WorkoutDesignerMessage::Effort(effort_index, EffortMessage::Delete),
                )))
                .push(Button::new(text("Edit")).on_press(WorkoutMessage::Design(
                    WorkoutDesignerMessage::Effort(effort_index, EffortMessage::Edit),
                ))),
            effort::EffortState::Editing {
                starting_value,
                ending_value,
                duration_in_minutes,
            } => Row::new()
                .spacing(5)
                .push(
                    TextInput::new("", duration_in_minutes)
                        .on_input(move |updated_effort_in_minutes| {
                            WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                                effort_index,
                                EffortMessage::UpdateDurationInMinutes(updated_effort_in_minutes),
                            ))
                        })
                        .on_submit(WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                            effort_index,
                            EffortMessage::ModificationDone,
                        ))),
                )
                .push(
                    TextInput::new("", starting_value)
                        .on_input(move |updated_starting_value| {
                            WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                                effort_index,
                                EffortMessage::UpdateStartingValue(updated_starting_value),
                            ))
                        })
                        .on_submit(WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                            effort_index,
                            EffortMessage::ModificationDone,
                        ))),
                )
                .push(
                    TextInput::new("", ending_value)
                        .on_input(move |updated_ending_value| {
                            WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                                effort_index,
                                EffortMessage::UpdateEndingValue(updated_ending_value),
                            ))
                        })
                        .on_submit(WorkoutMessage::Design(WorkoutDesignerMessage::Effort(
                            effort_index,
                            EffortMessage::ModificationDone,
                        ))),
                ),
        }
    }
}

struct WhiteText<'a> {
    text: Text<'a>,
}

impl WhiteText<'_> {
    fn new(white_text: String) -> Self {
        Self {
            text: text(white_text).size(25),
        }
    }
}

impl<'a> From<WhiteText<'a>> for Element<'a, WorkoutMessage> {
    fn from(white_text: WhiteText<'a>) -> Self {
        white_text.text.into()
    }
}
