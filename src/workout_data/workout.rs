use crate::workout_data::positive_float;

#[derive(Debug, Clone, PartialEq)]
struct Workout<EffortType> {
    description: String,
    units: Vec<WorkoutUnit<EffortType>>,
}
impl<EffortType> Workout<EffortType> {
    fn new(description: &'_ str, units: Vec<WorkoutUnit<EffortType>>) -> Self {
        Self {
            description: String::from(description),
            units,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WorkoutUnit<EffortType> {
    duration_in_minutes: positive_float::PositiveFloat,
    effort: EffortType,
}

impl<EffortType> WorkoutUnit<EffortType> {
    fn new(duration_in_minutes: positive_float::PositiveFloat, effort: EffortType) -> Self {
        Self {
            duration_in_minutes,
            effort,
        }
    }
}

trait GenerateEffortHeader {
    fn generate_effort_header() -> &'static str;
}

#[derive(Debug, Clone, PartialEq)]
struct Watts {
    watts: positive_float::PositiveFloat,
}

impl Watts {
    fn new(watts: positive_float::PositiveFloat) -> Self {
        Self { watts }
    }
}

impl GenerateEffortHeader for Watts {
    fn generate_effort_header() -> &'static str {
        "MINUTES WATTS"
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PercentOfFTP {
    percentage: positive_float::PositiveFloat,
}
impl PercentOfFTP {
    fn new(percentage: positive_float::PositiveFloat) -> Self {
        Self { percentage }
    }
}

impl GenerateEffortHeader for PercentOfFTP {
    fn generate_effort_header() -> &'static str {
        "MINUTES PERCENTAGE"
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Effort<EffortType>
where
    EffortType: GenerateEffortHeader,
{
    SingleEffort(EffortType),
    GroupOfEffort { efforts: Vec<Effort<EffortType>> },
}

#[cfg(test)]
mod test {
    mod workout {

        use super::super::{Effort, Watts, Workout, WorkoutUnit};
        use crate::workout_data::positive_float;
        #[test]
        fn construct_workout() {
            let _ = Workout::new(
                "Workout for testing",
                vec![WorkoutUnit::new(
                    positive_float::PositiveFloat::new(10.0)
                        .expect("Positive Duration can be created"),
                    Effort::SingleEffort(Watts::new(
                        positive_float::PositiveFloat::new(250.0)
                            .expect("Positive Wattage can be created"),
                    )),
                )],
            );
        }
    }
    mod workout_unit {

        use super::super::{Effort, Watts, WorkoutUnit};
        use crate::workout_data::positive_float;
        #[test]
        fn construct_workout_unit() {
            let _ = WorkoutUnit::new(
                positive_float::PositiveFloat::new(10.0).expect("Positive Duration can be created"),
                Effort::SingleEffort(Watts::new(
                    positive_float::PositiveFloat::new(250.0)
                        .expect("Positive Wattage can be created"),
                )),
            );
        }
    }
    mod effort_type {
        use super::super::{Effort, PercentOfFTP, Watts};
        use crate::workout_data::positive_float;
        #[test]
        fn construct_watts() {
            let _ = Effort::SingleEffort(Watts::new(
                positive_float::PositiveFloat::new(250.0).expect("Positive Wattage can be created"),
            ));
        }
        #[test]
        fn construct_percentage_of_ftp() {
            let _ = Effort::SingleEffort(PercentOfFTP::new(
                positive_float::PositiveFloat::new(100.0)
                    .expect("Positive Percentag can be created"),
            ));
        }

        #[test]
        fn construct_group_of_effort() {
            let _ = Effort::GroupOfEffort {
                efforts: vec![
                    Effort::SingleEffort(PercentOfFTP::new(
                        positive_float::PositiveFloat::new(100.0)
                            .expect("Positive Percentag can be created"),
                    )),
                    Effort::SingleEffort(PercentOfFTP::new(
                        positive_float::PositiveFloat::new(90.0)
                            .expect("Positive Percentag can be created"),
                    )),
                ],
            };
        }
    }
}
