use crate::workout_data::ToMRC;

const SPLITTING_THRESHOLD_IN_MINUTES: f64 = 0.2;

fn is_ramp_effort(effort: &Effort) -> bool {
    effort.starting_value != effort.ending_value
}
fn effort_can_be_split(effort: &Effort) -> bool {
    effort.duration_in_minutes > SPLITTING_THRESHOLD_IN_MINUTES
}
/// Combining a type of effort with a duration
/// for which it should be executed.
#[derive(Debug, Clone, PartialEq)]
pub struct Effort {
    pub(crate) duration_in_minutes: f64,
    pub(crate) starting_value: f64,
    pub(crate) ending_value: f64,
    pub gui_state: EffortState,
}

#[derive(Debug, Clone, Default)]
pub enum EffortState {
    #[default]
    Idle,
    Editing {
        starting_value: String,
        ending_value: String,
        duration_in_minutes: String,
    },
}

impl PartialEq for EffortState {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl Effort {
    /// Creating a new Effort unit.
    pub fn new(duration_in_minutes: f64, starting_value: f64, ending_value: Option<f64>) -> Self {
        Self {
            duration_in_minutes,
            starting_value,
            ending_value: ending_value.unwrap_or(starting_value),
            gui_state: EffortState::default(),
        }
    }
    // TODO: It would be better to have a seperate class `RampEffort` for this.
    pub fn to_mrc_for_ramp_effort(&self, starting_minute: f64) -> String {
        self.split_ramp_effort_into_constant_chunks()
            .iter()
            .fold(
                (String::new(), starting_minute),
                |(mrc_representation, starting_minute), effort| {
                    let (mrc_representation_of_effort, starting_minute) =
                        effort.to_mrc(starting_minute);
                    (
                        if mrc_representation.is_empty() {
                            mrc_representation_of_effort
                        } else {
                            format!("{}\n{}", mrc_representation, mrc_representation_of_effort)
                        },
                        starting_minute,
                    )
                },
            )
            .0
    }
    pub fn split_ramp_effort_into_constant_chunks(&self) -> Vec<Effort> {
        assert!(self.duration_in_minutes > 0.0);
        assert!(self.starting_value != self.ending_value);

        let steps = (self.duration_in_minutes / SPLITTING_THRESHOLD_IN_MINUTES).ceil() as usize;
        let step_size = (self.ending_value - self.starting_value) / (steps as f64);
        let mut result = Vec::with_capacity(steps);

        for index in 0..steps {
            let interpolated_effort_value =
                self.starting_value + (index as f64 * step_size) + step_size / 2.0;

            result.push(Effort {
                duration_in_minutes: SPLITTING_THRESHOLD_IN_MINUTES,
                starting_value: interpolated_effort_value,
                ending_value: interpolated_effort_value,
                gui_state: self.gui_state.clone(),
            });
        }
        result
    }
    pub fn to_mrc(&self, starting_minute: f64) -> (String, f64) {
        let end_of_effort = starting_minute + self.duration_in_minutes;
        if is_ramp_effort(self) && effort_can_be_split(self) {
            return (self.to_mrc_for_ramp_effort(starting_minute), end_of_effort);
        }
        (
            format! {
                "{}\t{}\n\
                {}\t{}", starting_minute.to_mrc(), self.starting_value.to_mrc(), end_of_effort.to_mrc(), self.ending_value.to_mrc()
            },
            end_of_effort,
        )
    }

    pub fn to_edit(&mut self) {
        self.gui_state = EffortState::Editing {
            starting_value: self.starting_value.to_mrc(),
            ending_value: self.ending_value.to_mrc(),
            duration_in_minutes: self.duration_in_minutes.to_mrc(),
        }
    }
    pub fn to_idle(&self) -> Option<Effort> {
        if let EffortState::Editing {
            starting_value,
            ending_value,
            duration_in_minutes,
        } = &self.gui_state
        {
            let new_ending_value = if ending_value.is_empty() {
                None
            } else {
                Some(ending_value.parse().ok()?)
            };

            return Some(Effort::new(
                duration_in_minutes.parse().ok()?,
                starting_value.parse().ok()?,
                new_ending_value,
            ));
        }
        None
    }
    pub fn update_duration_of_effort(&mut self, updated_duration_of_effort: String) {
        if let EffortState::Editing {
            starting_value: _,
            ending_value: _,
            duration_in_minutes,
        } = &mut self.gui_state
        {
            *duration_in_minutes = updated_duration_of_effort;
        }
    }
    pub fn update_starting_value(&mut self, updated_starting_value: String) {
        if let EffortState::Editing {
            starting_value,
            ending_value: _,
            duration_in_minutes: _,
        } = &mut self.gui_state
        {
            *starting_value = updated_starting_value;
        }
    }
    pub fn update_ending_value(&mut self, updated_ending_value: String) {
        if let EffortState::Editing {
            starting_value: _,
            ending_value,
            duration_in_minutes: _,
        } = &mut self.gui_state
        {
            *ending_value = updated_ending_value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{effort_can_be_split, is_ramp_effort, Effort};
    mod effort_unit {
        use super::{effort_can_be_split, is_ramp_effort, Effort};
        use crate::workout_data::workout::{efforts_to_mrc, extract_initial_starting_minutes};
        use crate::workout_data::ToMRC;

        #[test]
        fn construct() {
            let _ = Effort::new(60.0, 150.0, None);
        }

        #[test]
        fn effort_mrc() {
            assert_eq!(
                Effort::new(60.0, 100.0, None,).starting_value.to_mrc(),
                "100.00"
            )
        }
        #[test]
        fn to_mrc() {
            assert_eq!(
                Effort::new(5.0, 100.0, None,).to_mrc(5.0),
                (
                    String::from(
                        "5.00	100.00\n\
                10.00	100.00"
                    ),
                    10.0
                )
            )
        }

        #[test]
        fn test_extract_starting_minutes_from_efforts() {
            assert_eq!(
                extract_initial_starting_minutes(
                    &vec![Effort::new(7.0, 100.0, None), Effort::new(9.0, 100.0, None)],
                    5.0
                ),
                vec![5.0, 12.0]
            )
        }

        #[test]
        fn test_efforts_to_mrc() {
            assert_eq!(
                efforts_to_mrc(
                    &vec![
                        Effort::new(5.0, 100.0, None),
                        Effort::new(10.0, 150.0, None),
                        Effort::new(15.0, 200.0, None),
                        Effort::new(5.0, 120.0, None),
                    ],
                    5.0
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
                    40.0
                )
            )
        }
        #[test]
        fn ramp_efforts_to_mrc() {
            assert_eq!(
                efforts_to_mrc(
                    &vec![
                        Effort::new(5.0, 100.0, None),
                        Effort::new(1.0, 100.0, Some(150.0))
                    ],
                    5.0
                ),
                (
                    String::from(
                        "5.00\t100.00\n\
                10.00\t100.00\n\
                10.00\t105.00\n\
                10.20\t105.00\n\
                10.20\t115.00\n\
                10.40\t115.00\n\
                10.40\t125.00\n\
                10.60\t125.00\n\
                10.60\t135.00\n\
                10.80\t135.00\n\
                10.80\t145.00\n\
                11.00\t145.00"
                    ),
                    11.0
                )
            )
        }

        #[test]
        fn no_ramp_effort() {
            assert!(!is_ramp_effort(&Effort::new(5.0, 100.0, None)));
        }
        #[test]
        fn no_ramp_effort_explicit() {
            assert!(!is_ramp_effort(&Effort::new(5.0, 100.0, Some(100.0))));
        }
        #[test]
        fn ramp_effort() {
            assert!(is_ramp_effort(&Effort::new(5.0, 100.0, Some(150.0))));
        }
        #[test]
        fn effort_that_cannot_be_split() {
            assert!(!effort_can_be_split(&Effort::new(0.1, 100.0, None)));
        }
        #[test]
        fn effort_that_can_be_split() {
            assert!(effort_can_be_split(&Effort::new(5.0, 100.0, Some(150.0))));
        }
        #[test]
        fn split_effort_into_five_constant_pieces() {
            assert_eq!(
                Effort::new(1.0, 100.0, Some(150.0)).split_ramp_effort_into_constant_chunks(),
                vec![
                    Effort::new(0.2, 105.0, Some(105.0)),
                    Effort::new(0.2, 115.0, Some(115.0)),
                    Effort::new(0.2, 125.0, Some(125.0)),
                    Effort::new(0.2, 135.0, Some(135.0)),
                    Effort::new(0.2, 145.0, Some(145.0))
                ]
            )
        }
    }
}
