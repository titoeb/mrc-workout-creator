use crm_workout_creator::workout_data::positive_float::PositiveFloat;
use crm_workout_creator::workout_data::workout::{Effort, EffortUnit, Watts, Workout};

#[test]
fn create_watt_workout() {
    let _new_workout = Workout::new(
        "test_workout",
        "Workout for testing",
        vec![
            Effort::SingleEffort(EffortUnit::new(
                PositiveFloat::new(300.0).unwrap(),
                Watts::new(PositiveFloat::new(100.0).unwrap()),
            )),
            Effort::GroupEffort {
                efforts: vec![
                    EffortUnit::new(
                        PositiveFloat::new(300.0).unwrap(),
                        Watts::new(PositiveFloat::new(100.0).unwrap()),
                    ),
                    EffortUnit::new(
                        PositiveFloat::new(60.0).unwrap(),
                        Watts::new(PositiveFloat::new(150.0).unwrap()),
                    ),
                ],
            },
        ],
    );
}
