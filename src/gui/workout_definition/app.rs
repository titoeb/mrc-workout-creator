use super::elements;
use crate::gui::mrc_creator::WorkoutMessage;
use iced::widget::{container, Column};
use iced::{Element, Length};

/// Holding the state of the overall MRCCreator Application.
#[derive(Default)]
pub struct WorkoutDefiner {
    workout_name: String,
    workout_description: String,
}

#[derive(Debug, Clone)]
pub enum WorkoutDefinerMessage {
    GenerateWorkoutClicked,
    LoadWorkoutClicked,
}

impl WorkoutDefiner {
    pub fn update(&mut self, message: WorkoutDefinerMessage) {
        match message {
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
        elements::base_design().push(elements::switch_to_workout_design())
    }

    pub fn get_workout_name(&self) -> &str {
        &self.workout_name
    }

    pub fn get_workout_description(&self) -> &str {
        &self.workout_description
    }
}
