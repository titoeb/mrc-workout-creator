use super::elements;
use crate::gui::mrc_creator::WorkoutMessage;
use crate::workout_data::workout::WorkoutType;
use iced::widget::{container, Column};
use iced::{Element, Length};

/// Holding the state of the overall MRCCreator Application.
#[derive(Default)]
pub struct WorkoutDefiner {
    selected_workout_type: Option<WorkoutType>,
    workout_name: String,
    workout_description: String,
}

#[derive(Debug, Clone)]
pub enum WorkoutDefinerMessage {
    TypeSelected(WorkoutType),
    GenerateWorkoutClicked,
    LoadWorkoutClicked,
}

impl WorkoutDefiner {
    pub fn update(&mut self, message: WorkoutDefinerMessage) {
        match message {
            WorkoutDefinerMessage::TypeSelected(workout_type) => {
                self.selected_workout_type = Some(workout_type);
            }
            WorkoutDefinerMessage::GenerateWorkoutClicked => {
                eprintln!("This should already be handled!")
            }
            WorkoutDefinerMessage::LoadWorkoutClicked => {
                eprintln!("This should already be handled!")
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
            .push(elements::select_workout_type_drop_down(
                self.selected_workout_type,
            ))
            .push(elements::switch_to_workout_design())
    }

    pub fn get_workout_name(&self) -> &str {
        &self.workout_name
    }

    pub fn get_workout_description(&self) -> &str {
        &self.workout_description
    }
    pub fn get_selected_workout_type(&self) -> Option<WorkoutType> {
        self.selected_workout_type
    }
}
