use super::elements::EffortUnitInput;
use crate::gui::mrc_creator::WorkoutMessage;
use crate::gui::style::{pink_button, LARGE_BUTTON};
use crate::gui::workout_design::elements;
use crate::gui::workout_design::visualization::core::Visualizer;
use crate::workout_data::workout::Workout;
use crate::workout_data::{effort, workout};
use dirs::home_dir;
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::Modifiers;
use iced::widget::{button, container, Column, Row};
use iced::widget::{focus_next, focus_previous};
use iced::Event::Keyboard;
use iced::{Element, Event, Length, Task};
use iced_core::widget::{
    operation::{Focusable, Operation},
    Id,
};
use iced_core::Rectangle;
use iced_runtime::task::widget;
use rfd::FileDialog;
use std::fs;
use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

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
    fn load_workout_from_file(&mut self) -> Task<WorkoutMessage> {
        if let Some(mrc_file_to_read) = FileDialog::new()
            .set_directory(path_or_home_directory(find_bike_computer()))
            .add_filter("Only Select mrc files", &["mrc"])
            .pick_file()
        {
            if let Ok(mrc_to_load) = fs::read_to_string(mrc_file_to_read) {
                match Workout::from_mrc(&mrc_to_load) {
                    Ok(loaded_workout) => {
                        *self = WorkoutDesigner::from(loaded_workout);
                    }
                    Err(error) => {
                        eprintln!("Could not read in the MRC file because of:");
                        eprintln!("{:?}", error);
                    }
                }
            }
        }
        Task::none()
    }
    pub fn update(&mut self, message: WorkoutDesignerMessage) -> Task<WorkoutMessage> {
        match message {
            WorkoutDesignerMessage::EffortUnitStartingValueChanged(value) => {
                self.effort_unit_input.set_starting_value(value);
                Task::none()
            }
            WorkoutDesignerMessage::EffortUnitEndingValueChanged(value) => {
                self.effort_unit_input.set_ending_value(value);
                Task::none()
            }
            WorkoutDesignerMessage::EffortUnitInputDurationChanged(value) => {
                self.effort_unit_input.set_duration(value);
                Task::none()
            }
            WorkoutDesignerMessage::CreateTask => {
                if !self.effort_unit_input.is_empty() {
                    if let Ok(effort) = effort::Effort::try_from(self.effort_unit_input.clone()) {
                        self.workout.add_effort(effort);
                        self.effort_unit_input.clear();
                    }
                }
                Task::none()
            }
            WorkoutDesignerMessage::ExportButtonPressed => {
                if let Some(mrc_file_to_write_to) = FileDialog::new()
                    .set_directory(path_or_home_directory(find_bike_computer()))
                    .add_filter("Only Select mrc files", &["mrc"])
                    .save_file()
                {
                    if let Some(mut opened_mrc_file) =
                        open_or_create(&make_it_mrc(mrc_file_to_write_to))
                    {
                        if let Err(error) = opened_mrc_file.write(self.workout.to_mrc().as_bytes())
                        {
                            eprintln!("Could not write workout because of:");
                            eprintln!("{}", error);
                        }
                    }
                };
                Task::none()
            }
            WorkoutDesignerMessage::LoadWorkoutPressed => self.load_workout_from_file(),
            WorkoutDesignerMessage::IcedEvent(event) => handle_keyboard_inputs(event),
            WorkoutDesignerMessage::Effort(index, effort_message) => {
                self.handle_effort_message(index, effort_message)
            }
        }
    }
    pub fn handle_effort_message(
        &mut self,
        index: usize,
        effort_message: EffortMessage,
    ) -> Task<WorkoutMessage> {
        match effort_message {
            EffortMessage::Delete => {
                self.workout.remove(index);
                Task::none()
            }
            EffortMessage::Edit => {
                self.workout.to_edit(index);
                Task::none()
            }
            EffortMessage::ModificationDone => {
                self.workout.to_idle(index);
                Task::none()
            }
            EffortMessage::UpdateDurationInMinutes(updated_duration_in_minutes) => {
                self.workout.efforts[index].update_duration_of_effort(updated_duration_in_minutes);
                Task::none()
            }
            EffortMessage::UpdateStartingValue(updated_value) => {
                self.workout.efforts[index].update_starting_value(updated_value);
                Task::none()
            }
            EffortMessage::UpdateEndingValue(updated_value) => {
                self.workout.efforts[index].update_ending_value(updated_value);
                Task::none()
            }
        }
    }

    pub fn view(&'_ self) -> Element<'_, WorkoutMessage> {
        container(self.elements())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
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
            .push(self.show_buttons())
            .width(Length::FillPortion(1))
            .spacing(20)
            .align_x(iced::Alignment::Center)
    }
    fn visualize_export_button(&self) -> button::Button<'_, WorkoutMessage> {
        pink_button("Export Workout")
            .height(60.0)
            .width(LARGE_BUTTON)
            .on_press(WorkoutMessage::from(
                WorkoutDesignerMessage::ExportButtonPressed,
            ))
    }
    fn visualize_load_button(&self) -> button::Button<'_, WorkoutMessage> {
        pink_button("Load Workout")
            .height(60.0)
            .width(LARGE_BUTTON)
            .on_press(WorkoutMessage::from(
                WorkoutDesignerMessage::LoadWorkoutPressed,
            ))
    }
    fn show_buttons(&self) -> Row<'_, WorkoutMessage> {
        Row::new()
            .push(self.visualize_load_button())
            .push(self.visualize_export_button())
            .spacing(10)
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
        .truncate(false)
        .append(false)
        .open(path_to_file)
        .ok()
}

fn ignore_event() -> Task<WorkoutMessage> {
    Task::none()
}

fn handle_keyboard_inputs(event: Event) -> Task<WorkoutMessage> {
    if let Keyboard(key) = event {
        match key {
            KeyPressed {
                key: iced_core::keyboard::Key::Named(iced::keyboard::key::Named::F1),
                modifiers: _,
                ..
            } => focus_id::<WorkoutMessage>(0),
            KeyPressed {
                key: iced_core::keyboard::Key::Named(iced::keyboard::key::Named::F2),
                modifiers: _,
                ..
            } => focus_id::<WorkoutMessage>(1),
            KeyPressed {
                key: iced_core::keyboard::Key::Named(iced::keyboard::key::Named::F3),
                modifiers: _,
                ..
            } => focus_id::<WorkoutMessage>(2),
            KeyPressed {
                key: iced_core::keyboard::Key::Named(iced::keyboard::key::Named::Tab),
                modifiers: Modifiers::SHIFT,
                ..
            } => focus_previous::<WorkoutMessage>(),
            KeyPressed {
                key: iced_core::keyboard::Key::Named(iced::keyboard::key::Named::Tab),
                modifiers: _,
                ..
            } => focus_next::<WorkoutMessage>(),

            _ => ignore_event(),
        }
    } else {
        ignore_event()
    }
}

fn _focus_id<T>(id: usize) -> impl Operation<T> {
    struct FocusOn {
        current: usize,
        to_focus_on: usize,
    }
    {
        impl<T> Operation<T> for FocusOn {
            fn focusable(&mut self, state: &mut dyn Focusable, _id: Option<&Id>) {
                if self.current == self.to_focus_on {
                    state.focus()
                } else {
                    state.unfocus()
                }
                self.current += 1;
            }

            fn container(
                &mut self,
                _id: Option<&Id>,
                _bounds: Rectangle,
                operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
            ) {
                operate_on_children(self)
            }
        }
    }

    FocusOn {
        current: 0,
        to_focus_on: id,
    }
}

pub fn focus_id<Message>(id: usize) -> Task<WorkoutMessage>
where
    Message: 'static,
{
    widget(_focus_id(id))
}

fn make_it_mrc(mut path_to_mrc_file: path::PathBuf) -> path::PathBuf {
    path_to_mrc_file.set_extension("mrc");
    path_to_mrc_file
}

fn find_bike_computer() -> Option<PathBuf> {
    let mut potential_bike_computer: Vec<PathBuf> = list_all_mounted_devices()
        .unwrap_or_default()
        .into_iter()
        .filter(|path| is_relevant_computer(path))
        .collect();
    match potential_bike_computer.len() {
        0 => None,
        _ => Some(
            potential_bike_computer
                .pop()
                .expect("Because of match it has at least one entry.")
                .join(Path::new("Internal shared storage/plans")),
        ),
    }
}

fn list_all_mounted_devices() -> Option<Vec<PathBuf>> {
    // Only works for linux right now:
    let potential_location = Path::new("/run/user/1000/gvfs/");
    if !potential_location.exists() {
        return None;
    }
    fs::read_dir(potential_location)
        .ok()?
        .map(|file| match file {
            Ok(file) => Some(file.path()),
            Err(_) => None,
        })
        .collect()
}

fn is_relevant_computer(path: &Path) -> bool {
    all_top_level_directories_exist(path)
}

fn all_top_level_directories_exist(path: &Path) -> bool {
    path.join(Path::new("Internal shared storage/exports"))
        .exists()
        && path
            .join(Path::new("Internal shared storage/factory"))
            .exists()
        && path
            .join(Path::new("Internal shared storage/gnss"))
            .exists()
        && path
            .join(Path::new("Internal shared storage/plans"))
            .exists()
        && path
            .join(Path::new("Internal shared storage/logs"))
            .exists()
        && path
            .join(Path::new("Internal shared storage/maps"))
            .exists()
        && path
            .join(Path::new("Internal shared storage/routes"))
            .exists()
}

fn path_or_home_directory(path: Option<PathBuf>) -> PathBuf {
    path.unwrap_or(home_dir().unwrap_or_default())
}
