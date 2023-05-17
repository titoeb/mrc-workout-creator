use super::elements::EffortUnitInput;
use crate::gui::mrc_creator::WorkoutMessage;
use crate::gui::workout_design::elements;
use crate::gui::workout_design::visualization::Visualizer;
use crate::workout_data::workout::Workout;
use crate::workout_data::{effort, workout};
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::Modifiers;
use iced::widget::{button, container, Column, Row, Text};
use iced::widget::{focus_next, focus_previous};
use iced::Event::Keyboard;
use iced::{Alignment, Command, Element, Event, Length};
use rfd::FileDialog;
use std::fs;
use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;

use std::path;

pub struct WorkoutDesigner {
    workout: workout::Workout,
    effort_unit_input: EffortUnitInput,
    visualizer: Visualizer,
}

impl Default for WorkoutDesigner {
    fn default() -> Self {
        Self {
            workout: workout::Workout::new("untitled", "no description", vec![]),
            effort_unit_input: EffortUnitInput::default(),
            visualizer: Visualizer::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkoutDesignerMessage {
    EffortUnitStartingValueChanged(String),
    EffortUnitEndingValueChanged(String),
    EffortUnitInputDurationChanged(String),
    CreateTask,
    ExportButtonPressed,
    LoadWorkoutPressed,
    IcedEvent(Event),
    Effort(usize, EffortMessage),
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
        }
    }
}

impl WorkoutDesigner {
    pub fn new(workout_name: &'_ str, workout_description: &'_ str) -> Self {
        Self {
            workout: workout::Workout::empty(workout_name, workout_description),
            effort_unit_input: EffortUnitInput::default(),
            visualizer: Visualizer::default(),
        }
    }
    fn load_workout_from_file(&mut self) -> Command<WorkoutMessage> {
        if let Some(json_file_to_read) = FileDialog::new()
            .add_filter("Only Select json files", &["json"])
            .pick_file()
        {
            if let Ok(json_to_load) = fs::File::open(json_file_to_read) {
                if let Ok(loaded_workout) =
                    serde_json::from_reader::<fs::File, workout::Workout>(json_to_load)
                {
                    *self = WorkoutDesigner::from(loaded_workout);
                } else {
                    eprintln!("Invalid Json file.")
                }
            }
        }
        Command::none()
    }
    pub fn update(&mut self, message: WorkoutDesignerMessage) -> Command<WorkoutMessage> {
        match message {
            WorkoutDesignerMessage::EffortUnitStartingValueChanged(value) => {
                self.effort_unit_input.set_starting_value(value);
                Command::none()
            }
            WorkoutDesignerMessage::EffortUnitEndingValueChanged(value) => {
                self.effort_unit_input.set_ending_value(value);
                Command::none()
            }
            WorkoutDesignerMessage::EffortUnitInputDurationChanged(value) => {
                self.effort_unit_input.set_duration(value);
                Command::none()
            }
            WorkoutDesignerMessage::CreateTask => {
                if !self.effort_unit_input.is_empty() {
                    if let Ok(effort) = effort::Effort::try_from(self.effort_unit_input.clone()) {
                        self.workout.add_effort(effort);
                        self.effort_unit_input.clear();
                    }
                }
                Command::none()
            }
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
                };
                Command::none()
            }
            WorkoutDesignerMessage::LoadWorkoutPressed => self.load_workout_from_file(),
            WorkoutDesignerMessage::IcedEvent(event) => {
                if let Keyboard(key) = event {
                    match key {
                        KeyPressed {
                            key_code: iced::keyboard::KeyCode::Tab,
                            modifiers: Modifiers::SHIFT,
                        } => focus_previous::<WorkoutMessage>(),
                        KeyPressed {
                            key_code: iced::keyboard::KeyCode::Tab,
                            modifiers: _,
                        } => focus_next::<WorkoutMessage>(),
                        _ => ignore_event(),
                    }
                } else {
                    ignore_event()
                }
            }
            WorkoutDesignerMessage::Effort(index, effort_message) => {
                self.handle_effort_message(index, effort_message)
            }
        }
    }
    pub fn handle_effort_message(
        &mut self,
        index: usize,
        effort_message: EffortMessage,
    ) -> Command<WorkoutMessage> {
        match effort_message {
            EffortMessage::Delete => {
                self.workout.remove(index);
                Command::none()
            }
            EffortMessage::Edit => {
                self.workout.to_edit(index);
                Command::none()
            }
            EffortMessage::ModificationDone => {
                self.workout.to_idle(index);
                Command::none()
            }
            EffortMessage::UpdateDurationInMinutes(updated_duration_in_minutes) => {
                self.workout.efforts[index].update_duration_of_effort(updated_duration_in_minutes);
                Command::none()
            }
            EffortMessage::UpdateStartingValue(updated_value) => {
                self.workout.efforts[index].update_starting_value(updated_value);
                Command::none()
            }
            EffortMessage::UpdateEndingValue(updated_value) => {
                self.workout.efforts[index].update_ending_value(updated_value);
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<WorkoutMessage> {
        container(self.elements())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn elements(&self) -> Column<'_, WorkoutMessage> {
        elements::base_design()
            .push(self.effort_unit_input.view())
            .push(
                Row::new()
                    .padding(20)
                    .push(self.display_workout_and_buttons())
                    .push(self.display_main_page()),
            )
    }
    fn display_main_page(&self) -> Row<'_, WorkoutMessage> {
        let cloned_workout = self.workout.clone();
        Row::new()
            .push(self.visualizer.view(cloned_workout))
            .width(Length::FillPortion(2))
    }

    fn display_workout_and_buttons(&self) -> Column<'_, WorkoutMessage> {
        Column::new()
            .push(self.workout.view())
            .push(self.visualize_export_button())
            .push(self.visualize_load_button())
            .width(Length::FillPortion(1))
            .spacing(20)
            .align_items(Alignment::Center)
    }
    fn visualize_export_button(&self) -> button::Button<'_, WorkoutMessage> {
        button::Button::new(Text::new("Export Workout"))
            .on_press(WorkoutMessage::from(
                WorkoutDesignerMessage::ExportButtonPressed,
            ))
            .width(Length::Fixed(120.0))
    }
    fn visualize_load_button(&self) -> button::Button<'_, WorkoutMessage> {
        button::Button::new(Text::new("Load existing Workout"))
            .on_press(WorkoutMessage::from(
                WorkoutDesignerMessage::LoadWorkoutPressed,
            ))
            .width(Length::Fixed(120.0))
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

fn ignore_event() -> Command<WorkoutMessage> {
    Command::none()
}
