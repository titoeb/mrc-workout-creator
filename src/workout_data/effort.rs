use crate::workout_data::ToMRC;
use iced::{button, text_input};
use serde::{Deserialize, Serialize};

/// Combining a type of effort with a duration
/// for which it should be executed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Effort {
    pub(crate) duration_in_minutes: f64,
    pub(crate) starting_value: f64,
    pub(crate) ending_value: f64,

    #[serde(skip)]
    pub gui_state: EffortState,
}

#[derive(Debug, Clone)]
pub enum EffortState {
    Idle {
        edit_button: button::State,
        delete_button: button::State,
    },
    Editing {
        starting_value_state: text_input::State,
        starting_value: String,
        ending_value_state: text_input::State,
        ending_value: String,
        duration_in_minutes_state: text_input::State,
        duration_in_minutes: String,
    },
}

impl PartialEq for EffortState {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl Default for EffortState {
    fn default() -> Self {
        EffortState::Idle {
            edit_button: button::State::new(),
            delete_button: button::State::new(),
        }
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
    pub fn to_mrc(&self, starting_minute: f64) -> (String, f64) {
        let end_of_effort = starting_minute + self.duration_in_minutes;
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
            starting_value_state: text_input::State::default(),
            starting_value: self.starting_value.to_mrc(),
            ending_value_state: text_input::State::default(),
            ending_value: self.ending_value.to_mrc(),
            duration_in_minutes_state: text_input::State::default(),
            duration_in_minutes: self.duration_in_minutes.to_mrc(),
        }
    }
    pub fn to_idle(&mut self) {
        if let EffortState::Editing {
            starting_value_state: _,
            starting_value,
            ending_value_state: _,
            ending_value,
            duration_in_minutes_state: _,
            duration_in_minutes,
        } = &mut self.gui_state
        {
            let new_ending_value = if ending_value.is_empty() {
                None
            } else {
                Some(
                    ending_value
                        .parse()
                        .expect("Please provide a valid positive float."),
                )
            };
            *self = Effort::new(
                duration_in_minutes
                    .parse()
                    .expect("Please provide a valid positive float."),
                starting_value
                    .parse()
                    .expect("Please provide a valid positive float."),
                new_ending_value,
            );
        }
    }
    pub fn update_duration_of_effort(&mut self, updated_duration_of_effort: String) {
        if let EffortState::Editing {
            starting_value_state: _,
            starting_value: _,
            ending_value_state: _,
            ending_value: _,
            duration_in_minutes_state: _,
            duration_in_minutes,
        } = &mut self.gui_state
        {
            *duration_in_minutes = updated_duration_of_effort;
        }
    }
    pub fn update_starting_value(&mut self, updated_starting_value: String) {
        if let EffortState::Editing {
            starting_value_state: _,
            starting_value,
            ending_value: _,
            ending_value_state: _,
            duration_in_minutes_state: _,
            duration_in_minutes: _,
        } = &mut self.gui_state
        {
            *starting_value = updated_starting_value;
        }
    }
    pub fn update_ending_value(&mut self, updated_ending_value: String) {
        if let EffortState::Editing {
            starting_value_state: _,
            starting_value: _,
            ending_value,
            ending_value_state: _,
            duration_in_minutes_state: _,
            duration_in_minutes: _,
        } = &mut self.gui_state
        {
            *ending_value = updated_ending_value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Effort;
    mod effort_unit {
        use super::Effort;
        use crate::testing::serialize_deserialize;
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
        fn test_serialization() {
            let effort_unit_to_serialize = Effort::new(60.0, 100.0, None);

            assert_eq!(
                effort_unit_to_serialize,
                serialize_deserialize(&effort_unit_to_serialize)
            )
        }
    }
}
