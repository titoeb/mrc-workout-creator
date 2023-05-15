use crate::gui::mrc_creator::WorkoutMessage;
use crate::gui::workout_definition::app::WorkoutDefinerMessage;
use iced::alignment::{Alignment, Horizontal};
use iced::widget::{Button, Column, Row, Text};

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
