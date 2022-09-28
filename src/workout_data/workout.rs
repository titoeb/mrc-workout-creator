use crate::workout_data::positive_float::PositiveFloat;
/// A planed workout.
#[derive(Debug, Clone, PartialEq)]
pub struct Workout<EffortType>
where
    EffortType: GenerateCRMEffortHeader + ConvertEffortToCRM,
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
    EffortType: GenerateCRMEffortHeader + ConvertEffortToCRM,
{
    /// Create a new Workout
    pub fn new(name: &'_ str, description: &'_ str, efforts: Vec<Effort<EffortType>>) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            efforts,
        }
    }

    fn crm_header(&self) -> String {
        format! {
            "[COURSE HEADER]\n\
            DESCRIPTION = {}\n\
            {}\n\
            [END COURSE HEADER]",
            self.description,
            EffortType::generate_effort_header()
        }
    }
}

/// Individual efforts in a training.
/// Can either be a single unit, or contain
/// multiple ones.
#[derive(Debug, Clone, PartialEq)]
pub enum Effort<EffortType>
where
    EffortType: GenerateCRMEffortHeader + ConvertEffortToCRM,
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
pub struct EffortUnit<EffortType>
where
    EffortType: ConvertEffortToCRM + GenerateCRMEffortHeader,
{
    duration_in_minutes: PositiveFloat,
    effort: EffortType,
}

impl<EffortType> EffortUnit<EffortType>
where
    EffortType: ConvertEffortToCRM + GenerateCRMEffortHeader,
{
    /// Creating a new Effort unit.
    pub fn new(duration_in_minutes: PositiveFloat, effort: EffortType) -> Self {
        Self {
            duration_in_minutes,
            effort,
        }
    }
    fn effort_in_crm(&self) -> String {
        self.effort.to_crm()
    }
    fn to_crm(&self, starting_minute: PositiveFloat) -> String {
        format! {
            "{}\t{}\n\
            {}\t{}", starting_minute.to_crm(), self.effort_in_crm(), starting_minute.add(&self.duration_in_minutes).to_crm(), self.effort_in_crm()
        }
    }
}
/// This function is used to create the correct Header in
/// the crm file. For "Wattage", that would be
/// MINUTES WATTAGE
pub trait GenerateCRMEffortHeader {
    /// How does the correct header in csr file for
    /// this type?
    fn generate_effort_header() -> &'static str;
}

/// How to convert the specific effort like Watt
/// into a string that can be displayed within
/// a crm file.
pub trait ConvertEffortToCRM {
    /// Generate CRM representation
    fn to_crm(&self) -> String;
}

/// A Wattage that should be executed.
#[derive(Debug, Clone, PartialEq)]
pub struct Watts {
    watts: PositiveFloat,
}

impl Watts {
    /// Creating a Wattage
    pub fn new(watts: PositiveFloat) -> Self {
        Self { watts }
    }
}

impl GenerateCRMEffortHeader for Watts {
    fn generate_effort_header() -> &'static str {
        "MINUTES WATTS"
    }
}

impl ConvertEffortToCRM for Watts {
    fn to_crm(&self) -> String {
        self.watts.to_crm()
    }
}

/// Percentage of FTP that should be execute.
/// Then the effort is proportional to the FTP
/// of the athlete.
#[derive(Debug, Clone, PartialEq)]
pub struct PercentOfFTP {
    percentage: PositiveFloat,
}
impl PercentOfFTP {
    /// Create a new percentage of FTP.
    pub fn new(percentage: PositiveFloat) -> Self {
        Self { percentage }
    }
}

impl GenerateCRMEffortHeader for PercentOfFTP {
    fn generate_effort_header() -> &'static str {
        "MINUTES PERCENTAGE"
    }
}

impl ConvertEffortToCRM for PercentOfFTP {
    fn to_crm(&self) -> String {
        self.percentage.to_crm()
    }
}

#[cfg(test)]
mod test {
    use super::{ConvertEffortToCRM, GenerateCRMEffortHeader};
    struct TestActivity;
    impl GenerateCRMEffortHeader for TestActivity {
        fn generate_effort_header() -> &'static str {
            "MINUTES EFFORT-UNIT"
        }
    }
    impl ConvertEffortToCRM for TestActivity {
        fn to_crm(&self) -> String {
            String::from("100.00")
        }
    }
    mod workout {
        use super::super::{Effort, EffortUnit, Watts, Workout};
        use super::TestActivity;
        use crate::workout_data::positive_float::PositiveFloat;

        #[test]
        fn construct_workout() {
            let _ = Workout::new(
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

        #[test]
        fn create_crm_header_watts() {
            let workout: Workout<TestActivity> =
                Workout::new("test_workout", "Workout for testing", vec![]);

            assert_eq!(
                workout.crm_header(),
                "[COURSE HEADER]\n\
            DESCRIPTION = Workout for testing\n\
            MINUTES EFFORT-UNIT\n\
            [END COURSE HEADER]"
            )
        }
    }

    mod effort {
        use super::super::{Effort, EffortUnit, PercentOfFTP, Watts};
        use crate::workout_data::positive_float::PositiveFloat;
        #[test]
        fn create_single_effort_with_watts() {
            let _ = Effort::SingleEffort(EffortUnit::new(
                PositiveFloat::new(300.0).unwrap(),
                Watts::new(PositiveFloat::new(100.0).unwrap()),
            ));
        }

        #[test]
        fn create_single_effort_with_percentage() {
            let _ = Effort::SingleEffort(EffortUnit::new(
                PositiveFloat::new(300.0).unwrap(),
                PercentOfFTP::new(PositiveFloat::new(100.0).unwrap()),
            ));
        }
        #[test]
        fn construct_group_of_effort() {
            let _ = Effort::GroupEffort {
                efforts: vec![
                    EffortUnit::new(
                        PositiveFloat::new(300.0).unwrap(),
                        PercentOfFTP::new(PositiveFloat::new(100.0).unwrap()),
                    ),
                    EffortUnit::new(
                        PositiveFloat::new(60.0).unwrap(),
                        PercentOfFTP::new(PositiveFloat::new(150.0).unwrap()),
                    ),
                ],
            };
        }
    }
    mod effort_unit {
        use super::super::{EffortUnit, PercentOfFTP};
        use super::TestActivity;
        use crate::workout_data::positive_float::PositiveFloat;

        #[test]
        fn construct() {
            let _ = EffortUnit::new(
                PositiveFloat::new(60.0).unwrap(),
                PercentOfFTP::new(PositiveFloat::new(150.0).unwrap()),
            );
        }

        #[test]
        fn effort_crm() {
            assert_eq!(
                EffortUnit::new(
                    PositiveFloat::new(60.0).expect("A positive duration can be created."),
                    TestActivity {}
                )
                .effort_in_crm(),
                "100.00"
            )
        }
        #[test]
        fn to_crm() {
            assert_eq!(
                EffortUnit::new(PositiveFloat::new(5.0).unwrap(), TestActivity {})
                    .to_crm(PositiveFloat::new(5.0).unwrap()),
                "5.00	100.00\n\
                10.00	100.00"
                )
        }

                ),
                "5.00	100.00\n\
                10.00	100.00"
            )
        }
    }
    mod individual_efforts {
        use super::super::{PercentOfFTP, Watts};
        use crate::workout_data::positive_float::PositiveFloat;
        use crate::workout_data::workout::ConvertEffortToCRM;

        #[test]
        fn construct_watts() {
            let _ = Watts::new(PositiveFloat::new(300.0).unwrap());
        }
        #[test]
        fn display_watts_in_crm() {
            assert_eq!(
                Watts::new(PositiveFloat::new(300.0).unwrap(),).to_crm(),
                "300.00"
            )
        }

        #[test]
        fn construct_percentage_of_ftp() {
            let _ = PercentOfFTP::new(PositiveFloat::new(100.0).unwrap());
        }

        #[test]
        fn display_percentage_of_ftp_in_crm() {
            assert_eq!(
                PercentOfFTP::new(PositiveFloat::new(100.0).unwrap(),).to_crm(),
                "100.00"
            )
        }
    }
}
