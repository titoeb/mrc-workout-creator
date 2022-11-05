use crate::workout_data::positive_float::PositiveFloat;
use iced::{button, text_input};
use serde::{Deserialize, Serialize};

/// Combining a type of effort with a duration
/// for which it should be executed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Effort {
    pub(crate) duration_in_minutes: PositiveFloat,
    pub(crate) starting_value: PositiveFloat,
    pub(crate) ending_value: PositiveFloat,

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
        value_state: text_input::State,
        value: String,
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
    pub fn new(
        duration_in_minutes: PositiveFloat,
        starting_value: PositiveFloat,
        ending_value: Option<PositiveFloat>,
    ) -> Self {
        Self {
            duration_in_minutes,
            starting_value: starting_value.clone(),
            ending_value: ending_value.unwrap_or(starting_value),
            gui_state: EffortState::default(),
        }
    }
    pub fn to_crm(&self, starting_minute: PositiveFloat) -> (String, PositiveFloat) {
        let end_of_effort = starting_minute.clone() + self.duration_in_minutes.clone();
        (
            format! {
                "{}\t{}\n\
                {}\t{}", starting_minute.to_crm(), self.starting_value.to_crm(), end_of_effort.to_crm(), self.ending_value.to_crm()
            },
            end_of_effort,
        )
    }

    pub fn to_edit(&mut self) {
        self.gui_state = EffortState::Editing {
            value_state: text_input::State::default(),
            value: String::from(self.starting_value.clone()),
            duration_in_minutes_state: text_input::State::default(),
            duration_in_minutes: String::from(self.duration_in_minutes.clone()),
        }
    }
    pub fn to_idle(&mut self) {
        if let EffortState::Editing {
            value_state: _,
            value,
            duration_in_minutes_state: _,
            duration_in_minutes,
        } = &mut self.gui_state
        {
            self.duration_in_minutes = PositiveFloat::try_from(duration_in_minutes)
                .expect("Please provide a valid positive float.");
            self.starting_value =
                PositiveFloat::try_from(value).expect("Please provide a valid positive float.");
            self.gui_state = EffortState::Idle {
                edit_button: button::State::new(),
                delete_button: button::State::new(),
            };
        }
    }
    pub fn update_duration_of_effort(&mut self, updated_duration_of_effort: String) {
        if let EffortState::Editing {
            value_state: _,
            value: _,
            duration_in_minutes_state: _,
            duration_in_minutes,
        } = &mut self.gui_state
        {
            *duration_in_minutes = updated_duration_of_effort;
        }
    }
    pub fn update_value(&mut self, updated_value: String) {
        if let EffortState::Editing {
            value_state: _,
            value,
            duration_in_minutes_state: _,
            duration_in_minutes: _,
        } = &mut self.gui_state
        {
            *value = updated_value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Effort;
    mod effort_unit {
        use super::Effort;
        use crate::testing::serialize_deserialize;
        use crate::workout_data::positive_float::PositiveFloat;
        use crate::workout_data::workout::{efforts_to_crm, extract_initial_starting_minutes};

        #[test]
        fn construct() {
            let _ = Effort::new(
                PositiveFloat::new(60.0).unwrap(),
                PositiveFloat::new(150.0).unwrap(),
                None,
            );
        }

        #[test]
        fn effort_crm() {
            assert_eq!(
                Effort::new(
                    PositiveFloat::new(60.0).expect("A positive duration can be created."),
                    PositiveFloat::new(100.0).unwrap(),
                    None,
                )
                .starting_value
                .to_crm(),
                "100.00"
            )
        }
        #[test]
        fn to_crm() {
            assert_eq!(
                Effort::new(
                    PositiveFloat::new(5.0).unwrap(),
                    PositiveFloat::new(100.0).unwrap(),
                    None,
                )
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
                        Effort::new(
                            PositiveFloat::new(7.0).unwrap(),
                            PositiveFloat::new(100.0).unwrap(),
                            None
                        ),
                        Effort::new(
                            PositiveFloat::new(9.0).unwrap(),
                            PositiveFloat::new(100.0).unwrap(),
                            None
                        )
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
                        Effort::new(
                            PositiveFloat::new(5.0).unwrap(),
                            PositiveFloat::new(100.0).unwrap(),
                            None
                        ),
                        Effort::new(
                            PositiveFloat::new(10.0).unwrap(),
                            PositiveFloat::new(150.0).unwrap(),
                            None
                        ),
                        Effort::new(
                            PositiveFloat::new(15.0).unwrap(),
                            PositiveFloat::new(200.0).unwrap(),
                            None
                        ),
                        Effort::new(
                            PositiveFloat::new(5.0).unwrap(),
                            PositiveFloat::new(120.0).unwrap(),
                            None
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
            let effort_unit_to_serialize = Effort::new(
                PositiveFloat::new(60.0).expect("A positive duration can be created."),
                PositiveFloat::new(100.0).unwrap(),
                None,
            );

            assert_eq!(
                effort_unit_to_serialize,
                serialize_deserialize(&effort_unit_to_serialize)
            )
        }
    }
}
