use super::elements::EffortUnitInput;
use crate::gui::crm_creator::WorkoutMessage;
use crate::gui::workout_design::elements;
use crate::gui::workout_design::visualization::Visualizer;
use crate::workout_data::workout::Workout;
use crate::workout_data::{effort, workout};
use iced::{button, Alignment, Column, Container, Element, Length, Row, Text};
use rfd::FileDialog;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path;
pub struct WorkoutDesigner {
    workout: workout::Workout,
    effort_unit_input: EffortUnitInput,
    visualizer: Visualizer,
    export_button: button::State,
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
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkoutDesignerMessage {
    EffortUnitInputEffortChanged(String),
    EffortUnitInputDurationChanged(String),
    CreateTask,
    Effort(usize, EffortMessage),
    ExportButtonPressed,
}

#[derive(Debug, Clone)]
pub enum EffortMessage {
    Edit,
    ModificationDone,
    UpdateValue(String),
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
        }
    }

    pub fn update(&mut self, message: WorkoutDesignerMessage) {
        match message {
            WorkoutDesignerMessage::EffortUnitInputEffortChanged(value) => {
                self.effort_unit_input.set_effort(value);
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
            WorkoutDesignerMessage::Effort(index, EffortMessage::UpdateValue(updated_value)) => {
                self.workout.efforts[index].update_value(updated_value)
            }
            WorkoutDesignerMessage::ExportButtonPressed => {
                if let Some(crm_file_to_write_to) = FileDialog::new()
                    .add_filter("Only Select crm files", &["crm"])
                    .set_directory("~")
                    .save_file()
                {
                    if let (Some(mut opened_crm_file), Some(mut opened_json)) = (
                        open_or_create(&crm_file_to_write_to),
                        open_or_create(&get_path_to_json_file(&crm_file_to_write_to)),
                    ) {
                        let _error_when_writing_crm_file =
                            opened_crm_file.write(self.workout.to_crm().as_bytes());
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
                        .push(self.workout.view())
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
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(path_to_file)
        .ok()
}

fn get_path_to_json_file(path_to_crm_file: &path::Path) -> path::PathBuf {
    path_to_crm_file.with_extension("").with_extension("json")
}
