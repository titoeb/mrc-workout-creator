use super::elements::EffortUnitInput;
use crate::gui::mrc_creator::WorkoutMessage;
use crate::gui::workout_design::elements;
use crate::gui::workout_design::visualization::Visualizer;
use crate::workout_data::workout::Workout;
use crate::workout_data::{effort, workout};
use iced::theme::Container;
use iced::widget::{button, scrollable, Column, Row, Text};
use iced::{Alignment, Element, Length};
use rfd::FileDialog;
use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use std::path;
pub struct WorkoutDesigner {
    workout: workout::Workout,
    effort_unit_input: EffortUnitInput,
    visualizer: Visualizer,
    export_button: button::State,
    scrollable_efforts: scrollable::State,
}

impl Default for WorkoutDesigner {
    fn default() -> Self {
        Self {
            workout: workout::Workout::new(
                "untitled",
                "no description",
                vec![],
                workout::WorkoutType::Watts,
            ),
            effort_unit_input: EffortUnitInput::default(),
            visualizer: Visualizer::default(),
            export_button: button::State::new(),
            scrollable_efforts: scrollable::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkoutDesignerMessage {
    EffortUnitStartingValueChanged(String),
    EffortUnitEndingValueChanged(String),
    EffortUnitInputDurationChanged(String),
    CreateTask,
    Effort(usize, EffortMessage),
    ExportButtonPressed,
}

#[derive(Debug, Clone)]
pub enum EffortMessage {
    Edit,
    ModificationDone,
    UpdateStartingValue(String),
    UpdateEndingValue(String),
    UpdateDurationInMinutes(String),
    Delete,
}

impl From<Workout> for WorkoutDesigner {
    fn from(workout: Workout) -> Self {
        Self {
            workout,
            effort_unit_input: EffortUnitInput::default(),
            visualizer: Visualizer::default(),
            export_button: button::State::new(),
            scrollable_efforts: scrollable::State::new(),
        }
    }
}

impl WorkoutDesigner {
    pub fn new(
        workout_name: &'_ str,
        workout_description: &'_ str,
        workout_type: workout::WorkoutType,
    ) -> Self {
        Self {
            workout: workout::Workout::empty(workout_name, workout_description, workout_type),
            effort_unit_input: EffortUnitInput::default(),
            visualizer: Visualizer::default(),
            export_button: button::State::new(),
            scrollable_efforts: scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: WorkoutDesignerMessage) {
        match message {
            WorkoutDesignerMessage::EffortUnitStartingValueChanged(value) => {
                self.effort_unit_input.set_starting_value(value);
            }
            WorkoutDesignerMessage::EffortUnitEndingValueChanged(value) => {
                self.effort_unit_input.set_ending_value(value);
            }
            WorkoutDesignerMessage::EffortUnitInputDurationChanged(value) => {
                self.effort_unit_input.set_duration(value);
            }
            WorkoutDesignerMessage::CreateTask => {
                if !self.effort_unit_input.is_empty() {
                    self.workout.add_effort(
                        effort::Effort::try_from(self.effort_unit_input.clone())
                            .expect("Could not create effort"),
                    );
                    self.effort_unit_input.clear();
                }
            }
            WorkoutDesignerMessage::Effort(index, EffortMessage::Delete) => {
                self.workout.remove(index);
            }
            WorkoutDesignerMessage::Effort(index, EffortMessage::Edit) => {
                self.workout.to_edit(index)
            }
            WorkoutDesignerMessage::Effort(index, EffortMessage::ModificationDone) => {
                self.workout.to_idle(index)
            }
            WorkoutDesignerMessage::Effort(
                index,
                EffortMessage::UpdateDurationInMinutes(updated_duration_in_minutes),
            ) => self.workout.efforts[index].update_duration_of_effort(updated_duration_in_minutes),
            WorkoutDesignerMessage::Effort(
                index,
                EffortMessage::UpdateStartingValue(updated_value),
            ) => self.workout.efforts[index].update_starting_value(updated_value),
            WorkoutDesignerMessage::Effort(
                index,
                EffortMessage::UpdateEndingValue(updated_value),
            ) => self.workout.efforts[index].update_ending_value(updated_value),
            WorkoutDesignerMessage::ExportButtonPressed => {
                if let Some(mrc_file_to_write_to) = FileDialog::new()
                    .add_filter("Only Select mrc files", &["mrc"])
                    .set_directory("~")
                    .save_file()
                {
                    if let (Some(mut opened_mrc_file), Some(mut opened_json)) = (
                        open_or_create(&mrc_file_to_write_to),
                        open_or_create(&get_path_to_json_file(&mrc_file_to_write_to)),
                    ) {
                        let _error_when_writing_mrc_file =
                            opened_mrc_file.write(self.workout.to_mrc().as_bytes());
                        let _error_when_writing_json_file = opened_json
                            .write(serde_json::to_string(&self.workout).unwrap().as_bytes());
                    }
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<WorkoutMessage> {
        Container::new(self.elements())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn elements(&'_ mut self) -> Column<'_, WorkoutMessage> {
        let cloned_workout = self.workout.clone();
        elements::base_design(match self.workout.workout_type {
            workout::WorkoutType::PercentOfFTP => "Percentage of FTP Workout",
            workout::WorkoutType::Watts => "Watts Workout",
        })
        .push(self.effort_unit_input.view())
        .padding(10)
        .spacing(30)
        .push(
            Row::new()
                .padding(20)
                .push(
                    Column::new()
                        .push(self.workout.view(&mut self.scrollable_efforts))
                        .push(
                            button::Button::new(
                                &mut self.export_button,
                                Text::new("Export Workout"),
                            )
                            .on_press(WorkoutMessage::from(
                                WorkoutDesignerMessage::ExportButtonPressed,
                            ))
                            .width(Length::Units(120)),
                        )
                        .width(Length::FillPortion(1))
                        .spacing(20)
                        .align_items(Alignment::Center),
                )
                .push(
                    Row::new()
                        .push(self.visualizer.view(cloned_workout))
                        .width(Length::FillPortion(2)),
                ),
        )
    }
}

fn open_or_create(path_to_file: &path::PathBuf) -> Option<File> {
    if path_to_file.exists() {
        let _ = remove_file(path_to_file);
    }
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(path_to_file)
        .ok()
}

fn get_path_to_json_file(path_to_mrc_file: &path::Path) -> path::PathBuf {
    path_to_mrc_file.with_extension("").with_extension("json")
}
