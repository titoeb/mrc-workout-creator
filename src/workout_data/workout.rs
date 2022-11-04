use crate::workout_data::effort::Effort;
use crate::workout_data::positive_float::PositiveFloat;
use serde::{Deserialize, Serialize};
use std::ops::Add;
/// A planed workout.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Workout {
    /// Name of the workout.
    /// The full name of the file will be <name>.crm.
    name: String,
    /// Description of the workout.
    /// Will be in the `.crm`-file
    description: String,
    /// The individual efforst of the Workout.
    pub(crate) efforts: Vec<Effort>,
    /// Is this a Watts based or PercentageOfFTP based
    /// workout?
    pub(crate) workout_type: WorkoutType,
}
impl Workout {
    /// Create a new Workout
    pub fn new(
        name: &'_ str,
        description: &'_ str,
        efforts: Vec<Effort>,
        workout_type: WorkoutType,
    ) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            efforts,
            workout_type,
        }
    }
    /// Create a new workout without any efforts.
    pub fn empty(name: &'_ str, description: &'_ str, workout_type: WorkoutType) -> Self {
        Self::new(name, description, vec![], workout_type)
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
            self.workout_type.create_crm_string()
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
    /// Add a new effort to the workout.
    pub fn add_effort(&mut self, effort: Effort) {
        self.efforts.push(effort);
    }
    /// Remove an effort from a workout.
    pub fn remove(&mut self, index: usize) {
        self.efforts.remove(index);
    }
    /// Make an effort editable in the gui.
    pub fn to_edit(&mut self, index: usize) {
        self.efforts[index].to_edit();
    }
    /// Make an effort editable in the gui.
    pub fn to_idle(&mut self, index: usize) {
        self.efforts[index].to_idle();
    }
    pub fn update_duration_of_effort(&mut self, index: usize, updated_duration_in_minutes: String) {
        self.efforts[index].update_duration_of_effort(updated_duration_in_minutes);
    }
    pub fn total_time_of_workout(&self) -> PositiveFloat {
        self.efforts.iter().fold(
            PositiveFloat::new(0.0).expect("0.0 is a valid positive float."),
            |total_minutes, current_effort_length| {
                total_minutes.add(current_effort_length.duration_in_minutes.clone())
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub enum WorkoutType {
    Watts,
    PercentOfFTP,
}

impl WorkoutType {
    fn create_crm_string(&self) -> String {
        match self {
            WorkoutType::Watts => String::from("MINUTES WATTS"),
            WorkoutType::PercentOfFTP => String::from("MINUTES PERCENTAGE"),
        }
    }
}

pub fn efforts_to_crm(
    efforts: &Vec<Effort>,
    starting_minute: &PositiveFloat,
) -> (String, PositiveFloat) {
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

pub fn extract_initial_starting_minutes(
    efforts: &Vec<Effort>,
    starting_minute: &PositiveFloat,
) -> Vec<PositiveFloat> {
    let mut starting_times = Vec::new();
    let mut current_starting_time = starting_minute.clone();

    for effort in efforts {
        starting_times.push(current_starting_time.clone());
        current_starting_time = current_starting_time + effort.duration_in_minutes.clone();
    }

    starting_times
}

#[cfg(test)]
mod test {
    mod workout {
        use super::super::{Effort, Workout, WorkoutType};
        use crate::testing::serialize_deserialize;
        use crate::workout_data::positive_float::PositiveFloat;

        #[test]
        fn construct_workout() {
            let _ = Workout::new(
                "test_workout",
                "Workout for testing",
                vec![
                    Effort::new(
                        PositiveFloat::new(300.0).unwrap(),
                        PositiveFloat::new(100.0).unwrap(),
                    ),
                    Effort::new(
                        PositiveFloat::new(300.0).unwrap(),
                        PositiveFloat::new(100.0).unwrap(),
                    ),
                    Effort::new(
                        PositiveFloat::new(60.0).unwrap(),
                        PositiveFloat::new(150.0).unwrap(),
                    ),
                ],
                WorkoutType::Watts,
            );
        }

        #[test]
        fn create_crm_header_watts() {
            let workout: Workout = Workout::new(
                "test_workout",
                "Workout for testing",
                vec![],
                WorkoutType::Watts,
            );

            assert_eq!(
                workout.crm_head(),
                "[COURSE HEADER]\n\
            DESCRIPTION = Workout for testing\n\
            MINUTES WATTS\n\
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
                        Effort::new(
                            PositiveFloat::new(5.0).unwrap(),
                            PositiveFloat::new(80.0).unwrap(),
                        ),
                        Effort::new(
                            PositiveFloat::new(10.0).unwrap(),
                            PositiveFloat::new(100.0).unwrap(),
                        ),
                    ],
                    WorkoutType::Watts,
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
                    Effort::new(
                        PositiveFloat::new(5.0).unwrap(),
                        PositiveFloat::new(80.0).unwrap(),
                    ),
                    Effort::new(
                        PositiveFloat::new(10.0).unwrap(),
                        PositiveFloat::new(100.0).unwrap(),
                    ),
                ],
                WorkoutType::Watts,
            );
            assert_eq!(
                workout_to_test_serialization,
                serialize_deserialize(&workout_to_test_serialization)
            )
        }
        #[test]
        fn test_add_effort() {
            let mut workout_to_add_effort = Workout::new(
                "test_workout",
                "test-1",
                vec![Effort::new(
                    PositiveFloat::new(5.0).unwrap(),
                    PositiveFloat::new(80.0).unwrap(),
                )],
                WorkoutType::Watts,
            );

            workout_to_add_effort.add_effort(Effort::new(
                PositiveFloat::new(10.0).unwrap(),
                PositiveFloat::new(80.0).unwrap(),
            ));

            assert_eq!(
                workout_to_add_effort.efforts,
                vec![
                    Effort::new(
                        PositiveFloat::new(5.0).unwrap(),
                        PositiveFloat::new(80.0).unwrap(),
                    ),
                    Effort::new(
                        PositiveFloat::new(10.0).unwrap(),
                        PositiveFloat::new(80.0).unwrap()
                    ),
                ],
            )
        }
        #[test]
        fn test_total_time_of_workout() {
            let workout_to_count = Workout::new(
                "test_workout",
                "test-1",
                vec![
                    Effort::new(
                        PositiveFloat::new(5.0).unwrap(),
                        PositiveFloat::new(80.0).unwrap(),
                    ),
                    Effort::new(
                        PositiveFloat::new(15.0).unwrap(),
                        PositiveFloat::new(200.0).unwrap(),
                    ),
                ],
                WorkoutType::Watts,
            );
            assert_eq!(
                workout_to_count.total_time_of_workout(),
                PositiveFloat::new(20.0).unwrap()
            )
        }
    }
}
