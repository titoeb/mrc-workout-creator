use crate::workout_data::effort::Effort;
use serde::{Deserialize, Serialize};
/// A planed workout.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Workout {
    /// Name of the workout.
    /// The full name of the file will be <name>.mrc.
    name: String,
    /// Description of the workout.
    /// Will be in the `.mrc`-file
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

    /// Generate the mrc representation of a workout.
    pub fn to_mrc(&self) -> String {
        format!("{}\n{}", self.mrc_head(), self.mrc_body())
    }

    fn mrc_head(&self) -> String {
        format! {
            "[COURSE HEADER]\n\
            DESCRIPTION = {}\n\
            {}\n\
            [END COURSE HEADER]",
            self.description,
            self.workout_type.create_mrc_string()
        }
    }
    fn mrc_body(&self) -> String {
        format!(
            "[COURSE DATA]\n\
            {}\n\
            [END COURSE DATA]",
            self.mrc_body_workouts()
        )
    }
    fn mrc_body_workouts(&self) -> String {
        let mut efforts_as_mrc = Vec::new();
        let mut current_starting_minute = 0.0;

        for effort in &self.efforts {
            let (effort_as_mrc, new_starting_minute) = effort.to_mrc(current_starting_minute);
            efforts_as_mrc.push(effort_as_mrc);
            current_starting_minute = new_starting_minute;
        }

        efforts_as_mrc.join("\n")
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
    pub fn total_time_of_workout(&self) -> f64 {
        self.efforts
            .iter()
            .fold(0.0, |total_minutes, current_effort_length| {
                total_minutes + current_effort_length.duration_in_minutes
            })
    }
    pub fn workout_duration(&self) -> f64 {
        self.total_time_of_workout()
    }
    pub fn average_intensity(&self) -> f64 {
        let workout_duration = self.workout_duration();
        self.efforts
            .iter()
            .map(|effort| {
                (effort.duration_in_minutes / workout_duration)
                    * ((effort.starting_value + effort.ending_value) / 2.0)
            })
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub enum WorkoutType {
    Watts,
    PercentOfFTP,
}

impl WorkoutType {
    fn create_mrc_string(&self) -> String {
        match self {
            WorkoutType::Watts => String::from("MINUTES WATTS"),
            WorkoutType::PercentOfFTP => String::from("MINUTES PERCENTAGE"),
        }
    }
}

pub fn efforts_to_mrc(efforts: &Vec<Effort>, starting_minute: f64) -> (String, f64) {
    let starting_minutes = extract_initial_starting_minutes(efforts, starting_minute);
    let effort_string_with_final_minute = efforts
        .iter()
        .zip(starting_minutes.into_iter())
        .map(|(effort, starting_minute)| effort.to_mrc(starting_minute))
        .collect::<Vec<(String, f64)>>();

    (
        effort_string_with_final_minute
            .iter()
            .map(|(effort_string, _)| effort_string.clone())
            .collect::<Vec<String>>()
            .join("\n"),
        effort_string_with_final_minute
            .last()
            .unwrap_or(&(String::from(""), starting_minute))
            .1,
    )
}

pub fn extract_initial_starting_minutes(efforts: &Vec<Effort>, starting_minute: f64) -> Vec<f64> {
    let mut starting_times = Vec::new();
    let mut current_starting_time = starting_minute;

    for effort in efforts {
        starting_times.push(current_starting_time);
        current_starting_time += effort.duration_in_minutes;
    }

    starting_times
}

#[cfg(test)]
mod test {
    mod workout {
        use super::super::{Effort, Workout, WorkoutType};
        use crate::testing::serialize_deserialize;

        #[test]
        fn construct_workout() {
            let _ = Workout::new(
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

        #[test]
        fn create_mrc_header_watts() {
            let workout: Workout = Workout::new(
                "test_workout",
                "Workout for testing",
                vec![],
                WorkoutType::Watts,
            );

            assert_eq!(
                workout.mrc_head(),
                "[COURSE HEADER]\n\
            DESCRIPTION = Workout for testing\n\
            MINUTES WATTS\n\
            [END COURSE HEADER]"
            )
        }

        #[test]
        fn workout_to_mrc() {
            assert_eq!(
                Workout::new(
                    "test_workout",
                    "test-1",
                    vec![
                        Effort::new(5.0, 80.0, None,),
                        Effort::new(10.0, 100.0, None,),
                    ],
                    WorkoutType::Watts,
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
                15.00\t100.00\n\
                [END COURSE DATA]"
            )
        }
        #[test]
        fn test_serialization() {
            let workout_to_test_serialization = Workout::new(
                "test_workout",
                "test-1",
                vec![Effort::new(5.0, 80.0, None), Effort::new(10.0, 100.0, None)],
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
                vec![Effort::new(5.0, 80.0, None)],
                WorkoutType::Watts,
            );

            workout_to_add_effort.add_effort(Effort::new(10.0, 80.0, None));

            assert_eq!(
                workout_to_add_effort.efforts,
                vec![
                    Effort::new(5.0, 80.0, None,),
                    Effort::new(10.0, 80.0, None,),
                ],
            )
        }
        #[test]
        fn test_total_time_of_workout() {
            let workout_to_count = Workout::new(
                "test_workout",
                "test-1",
                vec![Effort::new(5.0, 80.0, None), Effort::new(15.0, 200.0, None)],
                WorkoutType::Watts,
            );
            assert_eq!(workout_to_count.total_time_of_workout(), 20.0)
        }
        #[test]
        fn workout_duration() {
            let workout = Workout::new(
                "test_workout",
                "test-1",
                vec![
                    Effort::new(5.0, 80.0, None),
                    Effort::new(15.0, 200.0, None),
                    Effort::new(2.0, 200.0, None),
                ],
                WorkoutType::Watts,
            );
            assert_eq!(workout.workout_duration(), 22.0);
        }
        #[test]
        fn average_intensity() {
            let workout = Workout::new(
                "test_workout",
                "test-1",
                vec![
                    Effort::new(5.0, 100.0, None),
                    Effort::new(15.0, 200.0, None),
                    Effort::new(5.0, 300.0, None),
                ],
                WorkoutType::Watts,
            );
            assert_eq!(workout.average_intensity(), 200.0);
        }
    }
}
