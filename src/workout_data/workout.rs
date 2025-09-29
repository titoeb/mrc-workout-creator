use crate::workout_data::effort::Effort;
use crate::workout_data::{from_mrc, from_plan_format};

#[derive(PartialEq, Debug)]
pub enum ExtractWorkoutError {
    Description(from_mrc::ExtractDescriptionError),
    Efforts(from_mrc::ExtractEffortError),
    FromPlanFormatError,
}
impl From<from_mrc::ExtractDescriptionError> for ExtractWorkoutError {
    fn from(value: from_mrc::ExtractDescriptionError) -> Self {
        Self::Description(value)
    }
}
impl From<from_mrc::ExtractEffortError> for ExtractWorkoutError {
    fn from(value: from_mrc::ExtractEffortError) -> Self {
        Self::Efforts(value)
    }
}
impl From<from_plan_format::ExtractPlanFormatError> for ExtractWorkoutError {
    fn from(_: from_plan_format::ExtractPlanFormatError) -> Self {
        Self::FromPlanFormatError
    }
}

/// A planed workout.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Workout {
    /// Name of the workout.
    /// The full name of the file will be <name>.mrc.
    name: String,
    /// Description of the workout.
    /// Will be in the `.mrc`-file
    description: String,
    /// The individual efforst of the Workout.
    pub(crate) efforts: Vec<Effort>,
}
impl Workout {
    /// Create a new Workout
    pub fn new(name: &'_ str, description: &'_ str, efforts: Vec<Effort>) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            efforts,
        }
    }
    /// Create a new workout without any efforts.
    pub fn empty(name: &'_ str, description: &'_ str) -> Self {
        Self::new(name, description, vec![])
    }

    /// Generate the mrc representation of a workout.
    pub fn to_mrc(&self) -> String {
        format!("{}\n{}", self.mrc_head(), self.mrc_body())
    }

    pub fn to_plan_format(&self) -> String {
        format!("{}\n{}", self.plan_format_head(), self.plan_format_body())
    }

    fn plan_format_body(&self) -> String {
        format!(
            "=STREAM=\n\
                {}",
            self.efforts_in_plan_format()
        )
    }

    fn efforts_in_plan_format(&self) -> String {
        self.efforts
            .iter()
            .map(|effort| effort.to_plan_format())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn plan_format_head(&self) -> String {
        format! {
            "=HEADER=\n\
             NAME={}\n\
            WORKOUT_TYPE=0",
            self.name,
        }
    }

    fn mrc_head(&self) -> String {
        format! {
            "[COURSE HEADER]\n\
            DESCRIPTION = {}\n\
            MINUTES WATTS\n\
            [END COURSE HEADER]",
            self.description,
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
        self.efforts[index].to_edit()
    }
    /// Make an effort editable in the gui.
    pub fn to_idle(&mut self, index: usize) {
        if let Some(new_effort) = self.efforts[index].to_idle() {
            self.efforts[index] = new_effort;
        }
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
    pub fn from_mrc(mrc: &str) -> Result<Self, ExtractWorkoutError> {
        let description = match from_mrc::extract_description(mrc) {
            Ok(description) => description,
            Err(_) => "".to_string(),
        };
        let efforts = from_mrc::extract_efforts(mrc)?;
        Ok(Self {
            name: String::from(""),
            description,
            efforts,
        })
    }
    pub fn from_plan_format(workout_in_plan_format: &str) -> Result<Self, ExtractWorkoutError> {
        Ok(from_plan_format::extract_workout(workout_in_plan_format)?)
    }
}

pub fn efforts_to_mrc(efforts: &Vec<Effort>, starting_minute: f64) -> (String, f64) {
    let starting_minutes = extract_initial_starting_minutes(efforts, starting_minute);
    let effort_string_with_final_minute = efforts
        .iter()
        .zip(starting_minutes)
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
    use super::*;
    mod workout {
        use super::*;

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
            );
        }

        #[test]
        fn create_mrc_header_watts() {
            let workout: Workout = Workout::new("test_workout", "Workout for testing", vec![]);

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
        fn test_add_effort() {
            let mut workout_to_add_effort =
                Workout::new("test_workout", "test-1", vec![Effort::new(5.0, 80.0, None)]);

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
            );
            assert_eq!(workout.average_intensity(), 200.0);
        }
    }
    mod from_mrc {
        use super::*;

        #[test]
        fn complete_workout() {
            let workout_as_mrc = "[COURSE HEADER]
DESCRIPTION = no description
MINUTES WATTS
[END COURSE HEADER]
[COURSE DATA]
0.00	80.00
10.00	150.00
10.00	300.00
15.00	300.00
15.00	150.00
19.00	150.00
19.00	300.00
26.00	300.00
[END COURSE DATA]";
            let workout = Workout::from_mrc(workout_as_mrc);

            assert_eq!(
                workout,
                Ok(Workout::new(
                    "",
                    "",
                    vec![
                        Effort::new(10.0, 80.0, Some(150.0)),
                        Effort::new(5.0, 300.0, None),
                        Effort::new(4.0, 150.0, None),
                        Effort::new(7.0, 300.0, None),
                    ],
                ))
            );
        }
        #[test]
        fn workout_without_description() {
            let workout_as_mrc = "[COURSE HEADER]
DESCRIPTION =
MINUTES WATTS
[END COURSE HEADER]
[COURSE DATA]
0.00	80.00
10.00	150.00
10.00	300.00
15.00	300.00
15.00	150.00
19.00	150.00
19.00	300.00
26.00	300.00
[END COURSE DATA]";
            let workout = Workout::from_mrc(workout_as_mrc);

            assert_eq!(
                workout,
                Ok(Workout::new(
                    "",
                    "",
                    vec![
                        Effort::new(10.0, 80.0, Some(150.0)),
                        Effort::new(5.0, 300.0, None),
                        Effort::new(4.0, 150.0, None),
                        Effort::new(7.0, 300.0, None),
                    ],
                ))
            );
        }

        #[test]
        fn to_mrc_from_mrc() {
            let workout = Workout::new(
                "",
                "Workout for testing",
                vec![
                    Effort::new(300.0, 100.0, None),
                    Effort::new(300.0, 100.0, None),
                    Effort::new(60.0, 150.0, None),
                ],
            );

            let reserialized_workout =
                Workout::from_mrc(&workout.to_mrc()).expect("Simple workout should be loadable");
            assert_eq!(workout, reserialized_workout)
        }
        #[test]
        fn to_mrc_from_mrc_no_description() {
            let workout = Workout::new(
                "",
                "",
                vec![
                    Effort::new(300.0, 100.0, None),
                    Effort::new(300.0, 100.0, None),
                    Effort::new(60.0, 150.0, None),
                ],
            );

            let reserialized_workout =
                Workout::from_mrc(&workout.to_mrc()).expect("Simple workout should be loadable");
            assert_eq!(workout, reserialized_workout)
        }
    }
    mod to_plan_format {
        use super::super::{Effort, Workout};

        #[test]
        fn to_header() {
            assert_eq!(
                Workout::new("Test Workout", "Test Workout Creation", vec![]).plan_format_head(),
                "=HEADER=
NAME=Test Workout
WORKOUT_TYPE=0"
            )
        }
        #[test]
        fn to_body() {
            assert_eq!(
                Workout::new(
                    "Test Workout",
                    "Test Workout Creation",
                    vec![
                        Effort::new(0.5, 50.0, None),
                        Effort::new(1.0, 100.0, None),
                        Effort::new(1.5, 150.0, None),
                        Effort::new(0.5, 200.0, None)
                    ]
                )
                .plan_format_body(),
                "=STREAM=
=INTERVAL=
PWR_LO=50
PWR_HI=50
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=100
PWR_HI=100
MESG_DURATION_SEC>=60?EXIT
=INTERVAL=
PWR_LO=150
PWR_HI=150
MESG_DURATION_SEC>=90?EXIT
=INTERVAL=
PWR_LO=200
PWR_HI=200
MESG_DURATION_SEC>=30?EXIT"
            )
        }
        #[test]
        fn to_plan_format() {
            assert_eq!(
                Workout::new(
                    "Test Workout",
                    "Test Workout Creation",
                    vec![
                        Effort::new(0.5, 50.0, None),
                        Effort::new(1.0, 100.0, None),
                        Effort::new(1.5, 150.0, None),
                        Effort::new(0.5, 200.0, None)
                    ]
                )
                .to_plan_format(),
                "=HEADER=
NAME=Test Workout
WORKOUT_TYPE=0
=STREAM=
=INTERVAL=
PWR_LO=50
PWR_HI=50
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=100
PWR_HI=100
MESG_DURATION_SEC>=60?EXIT
=INTERVAL=
PWR_LO=150
PWR_HI=150
MESG_DURATION_SEC>=90?EXIT
=INTERVAL=
PWR_LO=200
PWR_HI=200
MESG_DURATION_SEC>=30?EXIT"
            )
        }
    }
}
