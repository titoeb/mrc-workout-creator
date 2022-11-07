use mrc_workout_creator::workout_data::positive_float::PositiveFloat;
use mrc_workout_creator::workout_data::{effort::Effort, workout::Workout, workout::WorkoutType};

#[test]
fn simple_percent_of_ftp_workout_to_mrc() {
    assert_eq!(
        Workout::new(
            "test_workout",
            "test-1",
            vec![
                Effort::new(
                    PositiveFloat::new(5.0).unwrap(),
                    PositiveFloat::new(80.0).unwrap(),
                    None,
                ),
                Effort::new(
                    PositiveFloat::new(5.0).unwrap(),
                    PositiveFloat::new(100.0).unwrap(),
                    None,
                ),
            ],
            WorkoutType::PercentOfFTP
        )
        .to_mrc(),
        "[COURSE HEADER]\n\
    DESCRIPTION = test-1\n\
    MINUTES PERCENTAGE\n\
    [END COURSE HEADER]\n\
    [COURSE DATA]\n\
    0.00\t80.00\n\
    5.00\t80.00\n\
    5.00\t100.00\n\
    10.00\t100.00\n\
    [END COURSE DATA]"
    )
}

#[test]
fn simple_watt_workout_to_mrc() {
    assert_eq!(
        Workout::new(
            "test_workout",
            "test-1",
            vec![
                Effort::new(
                    PositiveFloat::new(5.0).unwrap(),
                    PositiveFloat::new(80.0).unwrap(),
                    None,
                ),
                Effort::new(
                    PositiveFloat::new(5.0).unwrap(),
                    PositiveFloat::new(100.0).unwrap(),
                    None,
                ),
            ],
            WorkoutType::Watts
        )
        .to_mrc(),
        "[COURSE HEADER]\n\
        DESCRIPTION = test-1\n\
        MINUTES WATTS\n\
        [END COURSE HEADER]\n\
        [COURSE DATA]\n\
        0.00\t80.00\n\
        5.00\t80.00\n\
        5.00\t100.00\n\
        10.00\t100.00\n\
        [END COURSE DATA]"
    )
}
