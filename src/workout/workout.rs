use crate::workout::positive_float;

#[derive(Debug, Clone, PartialEq)]
struct Workout {
    description: String,
    units: Vec<WorkoutUnit>,
}
impl Workout {
    fn new(description: &'_ str, units: Vec<WorkoutUnit>) -> Self {
        Self {
            description: String::from(description),
            units,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WorkoutUnit {
    duration_in_minutes: positive_float::PositiveFloat,
    effort: EffortType,
}

impl WorkoutUnit {
    fn new(duration_in_minutes: positive_float::PositiveFloat, effort: EffortType) -> Self {
        Self {
            duration_in_minutes,
            effort,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum EffortType {
    Watts(positive_float::PositiveFloat),
    PercentOfFTP(positive_float::PositiveFloat),
    GroupOfEffort { efforts: Vec<EffortType> },
}

#[cfg(test)]
mod test {
    mod workout {

        use super::super::{EffortType, Workout, WorkoutUnit};
        use crate::workout::positive_float;
        #[test]
        fn construct_workout() {
            let _ = Workout::new(
                "Workout for testing",
                vec![WorkoutUnit::new(
                    positive_float::PositiveFloat::new(10.0)
                        .expect("Positive Duration can be created"),
                    EffortType::Watts(
                        positive_float::PositiveFloat::new(250.0)
                            .expect("Positive Wattage can be created"),
                    ),
                )],
            );
        }
    }
    mod workout_unit {

        use super::super::{EffortType, WorkoutUnit};
        use crate::workout::positive_float;
        #[test]
        fn construct_workout_unit() {
            let _ = WorkoutUnit::new(
                positive_float::PositiveFloat::new(10.0).expect("Positive Duration can be created"),
                EffortType::Watts(
                    positive_float::PositiveFloat::new(250.0)
                        .expect("Positive Wattage can be created"),
                ),
            );
        }
    }
    mod effort_type {
        use super::super::EffortType;
        use crate::workout::positive_float;
        #[test]
        fn construct_watts() {
            let _ = EffortType::Watts(
                positive_float::PositiveFloat::new(250.0).expect("Positive Wattage can be created"),
            );
        }
        #[test]
        fn construct_percentage_of_ftp() {
            let _ = EffortType::PercentOfFTP(
                positive_float::PositiveFloat::new(100.0)
                    .expect("Positive Percentag can be created"),
            );
        }

        #[test]
        fn construct_group_of_effort() {
            let _ = EffortType::GroupOfEffort {
                efforts: vec![
                    EffortType::PercentOfFTP(
                        positive_float::PositiveFloat::new(100.0)
                            .expect("Positive Percentag can be created"),
                    ),
                    EffortType::Watts(
                        positive_float::PositiveFloat::new(250.0)
                            .expect("Positive Wattage can be created"),
                    ),
                ],
            };
        }
    }
}
