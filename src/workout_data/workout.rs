use crate::workout_data::positive_float;
/// A planed workout.
#[derive(Debug, Clone, PartialEq)]
pub struct Workout<EffortType>
where
    EffortType: GenerateEffortHeader,
{
    /// Name of the workout.
    /// The full name of the file will be <name>.crm.
    name: String,
    /// Description of the workout.
    /// Will be in the `.crm`-file
    description: String,
    /// The individual efforst of the Workout.
    efforts: Vec<Effort<EffortType>>,
}
impl<EffortType> Workout<EffortType>
where
    EffortType: GenerateEffortHeader,
{
    /// Create a new Workout
    pub fn new(name: &'_ str, description: &'_ str, efforts: Vec<Effort<EffortType>>) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            efforts,
        }
    }
}

/// Individual efforts in a training.
/// Can either be a single unit, or contain
/// multiple ones.
#[derive(Debug, Clone, PartialEq)]
pub enum Effort<EffortType>
where
    EffortType: GenerateEffortHeader,
{
    /// A single unit, e.g. 400 watts for a minute.
    SingleEffort(EffortUnit<EffortType>),
    /// A group of efforts that could be repeated like
    /// two Minutes of 250 Watts, then one minute of
    /// 300 watts.
    GroupEffort {
        /// The individual efforts.
        efforts: Vec<EffortUnit<EffortType>>,
    },
}

/// Combining a type of effort with a duration
/// for which it should be executed.
#[derive(Debug, Clone, PartialEq)]
pub struct EffortUnit<EffortType> {
    duration_in_minutes: positive_float::PositiveFloat,
    effort: EffortType,
}

impl<EffortType> EffortUnit<EffortType> {
    /// Creating a new Effort unit.
    pub fn new(duration_in_minutes: positive_float::PositiveFloat, effort: EffortType) -> Self {
        Self {
            duration_in_minutes,
            effort,
        }
    }
}
/// This function is used to create the correct Header in
/// the crm file. For "Wattage", that would be
/// MINUTES WATTAGE
pub trait GenerateEffortHeader {
    /// How does the correct header in csr file for
    /// this type?
    fn generate_effort_header() -> &'static str;
}

/// A Wattage that should be executed.
#[derive(Debug, Clone, PartialEq)]
pub struct Watts {
    watts: positive_float::PositiveFloat,
}

impl Watts {
    /// Creating a Wattage
    pub fn new(watts: positive_float::PositiveFloat) -> Self {
        Self { watts }
    }
}

impl GenerateEffortHeader for Watts {
    fn generate_effort_header() -> &'static str {
        "MINUTES WATTS"
    }
}

/// Percentage of FTP that should be execute.
/// Then the effort is proportional to the FTP
/// of the athlete.
#[derive(Debug, Clone, PartialEq)]
pub struct PercentOfFTP {
    percentage: positive_float::PositiveFloat,
}
impl PercentOfFTP {
    /// Create a new percentage of FTP.
    pub fn new(percentage: positive_float::PositiveFloat) -> Self {
        Self { percentage }
    }
}

impl GenerateEffortHeader for PercentOfFTP {
    fn generate_effort_header() -> &'static str {
        "MINUTES PERCENTAGE"
    }
}

#[cfg(test)]
mod test {
    mod workout {

        use super::super::{Effort, EffortUnit, Watts, Workout};
        use crate::workout_data::positive_float;
        #[test]
        fn construct_workout() {
            let _ = Workout::new(
                "test_workout",
                "Workout for testing",
                vec![
                    Effort::SingleEffort(EffortUnit::new(
                        positive_float::PositiveFloat::new(300.0)
                            .expect("A positive duration can be created."),
                        Watts::new(
                            positive_float::PositiveFloat::new(100.0)
                                .expect("Positive Percentage can be created"),
                        ),
                    )),
                    Effort::GroupEffort {
                        efforts: vec![
                            EffortUnit::new(
                                positive_float::PositiveFloat::new(300.0)
                                    .expect("A positive duration can be created."),
                                Watts::new(
                                    positive_float::PositiveFloat::new(100.0)
                                        .expect("Positive Percentage can be created"),
                                ),
                            ),
                            EffortUnit::new(
                                positive_float::PositiveFloat::new(60.0)
                                    .expect("A positive duration can be created."),
                                Watts::new(
                                    positive_float::PositiveFloat::new(150.0)
                                        .expect("Positive Percentage can be created"),
                                ),
                            ),
                        ],
                    },
                ],
            );
        }
    }

    mod effort {
        use super::super::{Effort, EffortUnit, PercentOfFTP, Watts};
        use crate::workout_data::positive_float;
        #[test]
        fn create_single_effort_with_watts() {
            let _ = Effort::SingleEffort(EffortUnit::new(
                positive_float::PositiveFloat::new(300.0)
                    .expect("A positive duration can be created."),
                Watts::new(
                    positive_float::PositiveFloat::new(100.0)
                        .expect("Positive Percentage can be created"),
                ),
            ));
        }

        #[test]
        fn create_single_effort_with_percentage() {
            let _ = Effort::SingleEffort(EffortUnit::new(
                positive_float::PositiveFloat::new(300.0)
                    .expect("A positive duration can be created."),
                PercentOfFTP::new(
                    positive_float::PositiveFloat::new(100.0)
                        .expect("Positive Percentage can be created"),
                ),
            ));
        }
        #[test]
        fn construct_group_of_effort() {
            let _ = Effort::GroupEffort {
                efforts: vec![
                    EffortUnit::new(
                        positive_float::PositiveFloat::new(300.0)
                            .expect("A positive duration can be created."),
                        PercentOfFTP::new(
                            positive_float::PositiveFloat::new(100.0)
                                .expect("Positive Percentage can be created"),
                        ),
                    ),
                    EffortUnit::new(
                        positive_float::PositiveFloat::new(60.0)
                            .expect("A positive duration can be created."),
                        PercentOfFTP::new(
                            positive_float::PositiveFloat::new(150.0)
                                .expect("Positive Percentage can be created"),
                        ),
                    ),
                ],
            };
        }
    }
    mod effort_unit {

        use super::super::{EffortUnit, PercentOfFTP};
        use crate::workout_data::positive_float;
        #[test]
        fn construct_effort_unit() {
            let _ = EffortUnit::new(
                positive_float::PositiveFloat::new(60.0)
                    .expect("A positive duration can be created."),
                PercentOfFTP::new(
                    positive_float::PositiveFloat::new(150.0)
                        .expect("Positive Percentage can be created"),
                ),
            );
        }
    }
}
