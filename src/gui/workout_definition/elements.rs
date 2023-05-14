use crate::gui::mrc_creator::WorkoutMessage;
use crate::gui::workout_definition::app::WorkoutDefinerMessage;
use crate::workout_data::workout::WorkoutType;
use iced::alignment::{Alignment, Horizontal};
use iced::widget::{Button, Column, PickList, Row, Text, TextInput};

pub(super) fn select_workout_type_drop_down<'a>(
    currently_selected_workout: Option<WorkoutType>,
) -> Row<'a, WorkoutMessage> {
    Row::new().push(
        PickList::new(
            &WorkoutType::ALL[..],
            currently_selected_workout,
            |workout_type| WorkoutMessage::from(WorkoutDefinerMessage::TypeSelected(workout_type)),
        )
        .placeholder("Workout Type")
        .padding(5)
        .text_size(35)
        .padding(10),
    )
}
pub(super) fn enter_workout_name<'a>(name: &str) -> Row<'a, WorkoutMessage> {
    Row::new().push(
        TextInput::new("Workout Name", name)
            .on_input(|workout_name| {
                WorkoutMessage::from(WorkoutDefinerMessage::NameGiven(workout_name))
            })
            .padding(5)
            .size(50)
            .width(iced::Length::Fixed(350.0)),
    )
}

pub(super) fn enter_workout_description<'a>(description: &str) -> Row<'a, WorkoutMessage> {
    Row::new().push(
        TextInput::new("Workout Description", description)
            .on_input(|workout_description| {
                WorkoutMessage::from(WorkoutDefinerMessage::DescriptionGiven(workout_description))
            })
            .padding(5)
            .size(20)
            .width(iced::Length::Fixed(400.0)),
    )
}

pub(super) fn switch_to_workout_design<'a>() -> Row<'a, WorkoutMessage> {
    Row::new()
        .spacing(10)
        .push(
            Button::new(
                Text::new("Generate Workout")
                    .size(23)
                    .horizontal_alignment(Horizontal::Center),
            )
            .padding(10)
            .on_press(WorkoutMessage::from(
                WorkoutDefinerMessage::GenerateWorkoutClicked,
            )),
        )
        .push(
            Button::new(
                Text::new("Load Workout")
                    .size(23)
                    .horizontal_alignment(Horizontal::Center),
            )
            .padding(10)
            .on_press(WorkoutMessage::from(
                WorkoutDefinerMessage::LoadWorkoutClicked,
            )),
        )
        .padding(10)
}

pub(super) fn base_design<'a>() -> Column<'a, WorkoutMessage> {
    Column::new()
        .align_items(Alignment::Center)
        .spacing(20)
        .push(Text::new("Workout Definition").size(50))
        .push(Row::new())
}
