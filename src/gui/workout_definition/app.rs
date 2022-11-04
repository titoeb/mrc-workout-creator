use super::elements;
use crate::gui::crm_creator::WorkoutMessage;
use crate::workout_data::workout::WorkoutType;
use iced::button;
use iced::text_input::State;
use iced::widget::pick_list;
use iced::{Column, Container, Element, Length};

/// Holding the state of the overall CRMCreator Application.
#[derive(Default)]
pub struct WorkoutDefiner {
    pick_list: pick_list::State<WorkoutType>,
    selected_workout_type: Option<WorkoutType>,
    workout_name: String,
    workout_description: String,
    name_input: State,
    workout_input: State,
    generate_button: button::State,
    load_workout_button: button::State,
}

#[derive(Debug, Clone)]
pub enum WorkoutDefinerMessage {
    TypeSelected(WorkoutType),
    NameGiven(String),
    DescriptionGiven(String),
    GenerateWorkoutClicked,
    LoadWorkoutClicked,
}

impl WorkoutDefiner {
    pub fn update(&mut self, message: WorkoutDefinerMessage) {
        match message {
            WorkoutDefinerMessage::TypeSelected(workout_type) => {
                self.selected_workout_type = Some(workout_type);
            }
            WorkoutDefinerMessage::NameGiven(workout_name) => self.workout_name = workout_name,
            WorkoutDefinerMessage::DescriptionGiven(workout_description) => {
                self.workout_description = workout_description
            }
            WorkoutDefinerMessage::GenerateWorkoutClicked => {
                eprintln!("This should already be handled!")
            }
            WorkoutDefinerMessage::LoadWorkoutClicked => {
                eprintln!("This should already be handled!")
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
        elements::base_design()
            .push(elements::select_workout_type_drop_down(
                &mut self.pick_list,
                self.selected_workout_type,
            ))
            .push(elements::enter_workout_name(
                &mut self.name_input,
                &self.workout_name,
            ))
            .push(elements::enter_workout_description(
                &mut self.workout_input,
                &self.workout_description,
            ))
            .push(elements::switch_to_workout_design(
                &mut self.generate_button,
                &mut self.load_workout_button,
            ))
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
