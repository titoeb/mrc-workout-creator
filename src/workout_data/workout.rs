use crate::workout_data::positive_float::PositiveFloat;
use serde::{Deserialize, Serialize};
/// A planed workout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// Generate the crm representation of a workout.
    pub fn to_crm(&self) -> String {
        format!("{}\n{}", self.crm_head(), self.crm_body())
    }

    fn crm_head(&self) -> String {
        format! {
            "[COURSE HEADER]\n\
            DESCRIPTION = {}\n\
            {}\n\
            [END COURSE HEADER]",
            self.description,
            EffortType::generate_effort_header()
        }
    }
    fn crm_body(&self) -> String {
        format!(
            "[COURSE DATA]\n\
            {}\n\
            [END COURSE DATA]",
            self.crm_body_workouts()
        )
    }
    fn crm_body_workouts(&self) -> String {
        let mut efforts_as_crm = Vec::new();
        let mut current_starting_minute = PositiveFloat::new(0.0).unwrap();

        for effort in &self.efforts {
            let (effort_as_crm, new_starting_minute) = effort.to_crm(current_starting_minute);
            efforts_as_crm.push(effort_as_crm);
            current_starting_minute = new_starting_minute;
        }

        efforts_as_crm.join("\n")
    }
}

/// Individual efforts in a training.
/// Can either be a single unit, or contain
/// multiple ones.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl<EffortType> Effort<EffortType>
where
    EffortType: ConvertEffortToCRM + GenerateCRMEffortHeader,
{
    fn to_crm(&self, starting_minute: PositiveFloat) -> (String, PositiveFloat) {
        match self {
            Self::SingleEffort(single_effort) => single_effort.to_crm(starting_minute),
            Self::GroupEffort { efforts } => efforts_to_crm(efforts, &starting_minute),
        }
    }
}

fn efforts_to_crm<EffortType>(
    efforts: &Vec<EffortUnit<EffortType>>,
    starting_minute: &PositiveFloat,
) -> (String, PositiveFloat)
where
    EffortType: GenerateCRMEffortHeader + ConvertEffortToCRM,
{
    let starting_minutes = extract_initial_starting_minutes(efforts, starting_minute);
    let effort_string_with_final_minute = efforts
        .iter()
        .zip(starting_minutes.into_iter())
        .map(|(effort, starting_minute)| effort.to_crm(starting_minute))
        .collect::<Vec<(String, PositiveFloat)>>();

    (
        effort_string_with_final_minute
            .iter()
            .map(|(effort_string, _)| effort_string.clone())
            .collect::<Vec<String>>()
            .join("\n"),
        effort_string_with_final_minute
            .last()
            .unwrap_or(&(String::from(""), starting_minute.clone()))
            .1
            .clone(),
    )
}

fn extract_initial_starting_minutes<EffortType>(
    efforts: &Vec<EffortUnit<EffortType>>,
    starting_minute: &PositiveFloat,
) -> Vec<PositiveFloat>
where
    EffortType: GenerateCRMEffortHeader + ConvertEffortToCRM,
{
    let mut starting_times = Vec::new();
    let mut current_starting_time = starting_minute.clone();

    for effort in efforts {
        starting_times.push(current_starting_time.clone());
        current_starting_time = current_starting_time + effort.duration_in_minutes.clone();
    }

    starting_times
}

/// Combining a type of effort with a duration
/// for which it should be executed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    fn to_crm(&self, starting_minute: PositiveFloat) -> (String, PositiveFloat) {
        let end_of_effort = starting_minute.clone() + self.duration_in_minutes.clone();
        (
            format! {
                "{}\t{}\n\
                {}\t{}", starting_minute.to_crm(), self.effort_in_crm(), end_of_effort.to_crm(), self.effort_in_crm()
            },
            end_of_effort,
        )
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    use serde::{Deserialize, Serialize};
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        use crate::testing::serialize_deserialize;
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
                workout.crm_head(),
                "[COURSE HEADER]\n\
            DESCRIPTION = Workout for testing\n\
            MINUTES EFFORT-UNIT\n\
            [END COURSE HEADER]"
            )
        }

        #[test]
        fn workout_to_crm() {
            assert_eq!(
                Workout::new(
                    "test_workout",
                    "test-1",
                    vec![
                        Effort::SingleEffort(EffortUnit::new(
                            PositiveFloat::new(5.0).unwrap(),
                            Watts::new(PositiveFloat::new(80.0).unwrap()),
                        )),
                        Effort::SingleEffort(EffortUnit::new(
                            PositiveFloat::new(10.0).unwrap(),
                            Watts::new(PositiveFloat::new(100.0).unwrap()),
                        )),
                    ],
                )
                .to_crm(),
                "[COURSE HEADER]\n\
                DESCRIPTION = test-1\n\
                MINUTES WATTS\n\
                [END COURSE HEADER]\n\
                [COURSE DATA]\n\
                0.00\t80.00\n\
                5.00\t80.00\n\
                5.00\t100.00\n\
                15.00\t100.00\n\
                [END COURSE DATA]"
            )
        }
        #[test]
        fn test_serialization() {
            let workout_to_test_serialization = Workout::new(
                "test_workout",
                "test-1",
                vec![
                    Effort::SingleEffort(EffortUnit::new(
                        PositiveFloat::new(5.0).unwrap(),
                        Watts::new(PositiveFloat::new(80.0).unwrap()),
                    )),
                    Effort::SingleEffort(EffortUnit::new(
                        PositiveFloat::new(10.0).unwrap(),
                        Watts::new(PositiveFloat::new(100.0).unwrap()),
                    )),
                ],
            );
            assert_eq!(
                workout_to_test_serialization,
                serialize_deserialize(&workout_to_test_serialization)
            )
        }
    }

    mod effort {
        use super::super::{Effort, EffortUnit, PercentOfFTP, Watts};
        use crate::{testing::serialize_deserialize, workout_data::positive_float::PositiveFloat};
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

        #[test]
        fn test_serialization() {
            let effort_to_serialize = Effort::SingleEffort(EffortUnit::new(
                PositiveFloat::new(300.0).unwrap(),
                PercentOfFTP::new(PositiveFloat::new(100.0).unwrap()),
            ));

            assert_eq!(
                effort_to_serialize,
                serialize_deserialize(&effort_to_serialize)
            )
        }
    }
    mod effort_unit {
        use super::super::{
            efforts_to_crm, extract_initial_starting_minutes, EffortUnit, PercentOfFTP, Watts,
        };
        use super::TestActivity;
        use crate::testing::serialize_deserialize;
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
                (
                    String::from(
                        "5.00	100.00\n\
                10.00	100.00"
                    ),
                    PositiveFloat::new(10.0).unwrap()
                )
            )
        }

        #[test]
        fn test_extract_starting_minutes_from_efforts() {
            assert_eq!(
                extract_initial_starting_minutes(
                    &vec![
                        EffortUnit::new(PositiveFloat::new(7.0).unwrap(), TestActivity {}),
                        EffortUnit::new(PositiveFloat::new(9.0).unwrap(), TestActivity {})
                    ],
                    &PositiveFloat::new(5.0).unwrap()
                ),
                vec![
                    PositiveFloat::new(5.0).unwrap(),
                    PositiveFloat::new(12.0).unwrap()
                ]
            )
        }

        #[test]
        fn test_efforts_to_crm() {
            assert_eq!(
                efforts_to_crm(
                    &vec![
                        EffortUnit::new(
                            PositiveFloat::new(5.0).unwrap(),
                            Watts::new(PositiveFloat::new(100.0).unwrap())
                        ),
                        EffortUnit::new(
                            PositiveFloat::new(10.0).unwrap(),
                            Watts::new(PositiveFloat::new(150.0).unwrap())
                        ),
                        EffortUnit::new(
                            PositiveFloat::new(15.0).unwrap(),
                            Watts::new(PositiveFloat::new(200.0).unwrap())
                        ),
                        EffortUnit::new(
                            PositiveFloat::new(5.0).unwrap(),
                            Watts::new(PositiveFloat::new(120.0).unwrap())
                        ),
                    ],
                    &PositiveFloat::new(5.0).unwrap()
                ),
                (
                    String::from(
                        "5.00\t100.00\n\
                10.00\t100.00\n\
                10.00\t150.00\n\
                20.00\t150.00\n\
                20.00\t200.00\n\
                35.00\t200.00\n\
                35.00\t120.00\n\
                40.00\t120.00"
                    ),
                    PositiveFloat::new(40.0).unwrap()
                )
            )
        }

        #[test]
        fn test_serialization() {
            let effort_unit_to_serialize = EffortUnit::new(
                PositiveFloat::new(60.0).expect("A positive duration can be created."),
                TestActivity {},
            );

            assert_eq!(
                effort_unit_to_serialize,
                serialize_deserialize(&effort_unit_to_serialize)
            )
        }
    }
    mod individual_efforts {
        use super::super::{PercentOfFTP, Watts};
        use crate::testing::serialize_deserialize;
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
        fn watts_serialization() {
            let watts_to_serialize = Watts::new(PositiveFloat::new(300.0).unwrap()).to_crm();
            assert_eq!(
                watts_to_serialize,
                serialize_deserialize(&watts_to_serialize)
            );
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

        #[test]
        fn percentage_of_ftp_serialization() {
            let percentage_to_serialize =
                PercentOfFTP::new(PositiveFloat::new(100.0).unwrap()).to_crm();
            assert_eq!(
                percentage_to_serialize,
                serialize_deserialize(&percentage_to_serialize)
            );
        }
    }
}
