use crm_workout_creator::workout_data::positive_float::PositiveFloat;
use crm_workout_creator::workout_data::{effort::Effort, workout::Workout, workout::WorkoutType};

#[test]
fn create_watt_workout() {
    let _new_workout = Workout::new(
        "test_workout",
        "Workout for testing",
        vec![
            Effort::new(
                PositiveFloat::new(300.0).unwrap(),
                PositiveFloat::new(100.0).unwrap(),
                None,
            ),
            Effort::new(
                PositiveFloat::new(300.0).unwrap(),
                PositiveFloat::new(100.0).unwrap(),
                None,
            ),
            Effort::new(
                PositiveFloat::new(60.0).unwrap(),
                PositiveFloat::new(150.0).unwrap(),
                None,
            ),
        ],
        WorkoutType::Watts,
    );
}
