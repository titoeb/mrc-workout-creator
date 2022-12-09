use mrc_workout_creator::workout_data::{effort::Effort, workout::Workout, workout::WorkoutType};

#[test]
fn create_watt_workout() {
    let _new_workout = Workout::new(
        "test_workout",
        "Workout for testing",
        vec![
            Effort::new(300.0, 100.0, None),
            Effort::new(300.0, 100.0, None),
            Effort::new(60.0, 150.0, None),
        ],
        WorkoutType::Watts,
    );
}
