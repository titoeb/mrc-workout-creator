use std::num::ParseFloatError;

use super::app::{EffortMessage, WorkoutDesignerMessage};
use crate::gui::mrc_creator::WorkoutMessage;
use crate::gui::style::{pink_button, pink_text_input, WhiteText};
use crate::workout_data::ToMRC;
use crate::workout_data::{effort, workout};
use iced::widget::{container, scrollable, Column, Row, TextInput};
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
                pink_text_input("Duration in Minutes", &self.duration.value)
                    .padding(self.padding)
                    .size(self.size)
                    .on_submit(self.creation_message.clone())
                    .on_input(self.on_duration_change),
            )
            .push(
                pink_text_input("Starting Wattage", &self.effort.starting_value)
                    .padding(self.padding)
                    .size(self.size)
                    .on_submit(self.creation_message.clone())
                    .on_input(self.on_starting_value_change),
            )
            .push(
                pink_text_input("Ending Wattage", &self.effort.ending_value)
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
        container::Container::new(
            Column::new()
                .spacing(20)
                .push(effort_string_headers())
                .push(scrollable(
                    self.efforts.iter().enumerate().fold(
                        Column::new(),
                        |scrollable, (effort_index, effort)| {
                            scrollable.push(effort.view(effort_index))
                        },
                    ),
                )),
        )
    }
}

impl<'a> effort::Effort {
    fn view(&'a self, effort_index: usize) -> impl Into<Element<'a, WorkoutMessage>> {
        match &self.gui_state {
            effort::EffortState::Idle => Row::new()
                .spacing(5)
                .push(effort_string_row(
                    self.duration_in_minutes.to_mrc(),
                    self.starting_value.to_mrc(),
                    self.ending_value.to_mrc(),
                ))
                .push(pink_button("Delete").on_press(WorkoutMessage::Design(
                    WorkoutDesignerMessage::Effort(effort_index, EffortMessage::Delete),
                )))
                .push(pink_button("Edit").on_press(WorkoutMessage::Design(
                    WorkoutDesignerMessage::Effort(effort_index, EffortMessage::Edit),
                ))),
            effort::EffortState::Editing {
                starting_value,
                ending_value,
                duration_in_minutes,
            } => Row::new()
                .spacing(5)
                .width(300)
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
                        )))
                        .width(90)
                        .size(25),
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
                        )))
                        .width(90)
                        .size(25),
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
                        )))
                        .width(90)
                        .size(25),
                ),
        }
    }
}

fn effort_string_text<'a>(white_text: String) -> WhiteText<'a> {
    WhiteText::new(white_text)
        .width(90)
        .horizontal_alignment(iced_native::alignment::Horizontal::Center)
}

fn effort_string_headers<'a>() -> Row<'a, WorkoutMessage> {
    effort_string_row(
        String::from("Duration"),
        String::from("Start"),
        String::from("End"),
    )
}

fn effort_string_row<'a>(
    first_value: String,
    second_value: String,
    third_value: String,
) -> Row<'a, WorkoutMessage> {
    Row::new()
        .push(effort_string_text(first_value))
        .push(effort_string_text(second_value))
        .push(effort_string_text(third_value))
        .align_items(Alignment::Start)
        .width(300)
}
