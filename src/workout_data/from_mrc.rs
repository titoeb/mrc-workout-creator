use crate::workout_data::effort::Effort;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum ExtractDescriptionError {
    NoDescription,
}
pub fn extract_description(mrc: &str) -> Result<String, ExtractDescriptionError> {
    let capture_description = Regex::new(r"DESCRIPTION = (.+)\n").expect("This regex is valid.");
    Ok(
        match capture_description
            .captures(mrc)
            .ok_or(ExtractDescriptionError::NoDescription)?
            .get(1)
            .ok_or(ExtractDescriptionError::NoDescription)?
            .as_str()
        {
            "no description" => String::from(""),
            other_string => String::from(other_string),
        },
    )
}
#[derive(PartialEq, Debug)]
pub enum ExtractEffortError {
    NoEffortsGiven,
    EffortNotValid(EffortUnvalidError),
}
impl From<EffortUnvalidError> for ExtractEffortError {
    fn from(value: EffortUnvalidError) -> Self {
        ExtractEffortError::EffortNotValid(value)
    }
}
pub fn extract_efforts(mrc: &str) -> Result<Vec<Effort>, ExtractEffortError> {
    let efforts_as_strings = extract_efforts_string(mrc)?;
    Ok(split_by_every_other_newline(&efforts_as_strings)
        .into_iter()
        .map(extract_effort_from_string)
        .collect::<Result<Vec<Effort>, EffortUnvalidError>>()?)
}

fn split_by_every_other_newline(input: &str) -> Vec<String> {
    let mut split_strings = Vec::new();
    let lines: Vec<_> = input.trim().lines().collect();
    let mut line_buffer = Vec::new();

    for (index, line) in lines.into_iter().enumerate() {
        line_buffer.push(line.to_string());
        if index % 2 == 1 {
            split_strings.push(line_buffer.join("\n"));
            line_buffer.clear()
        }
    }

    split_strings
}

#[derive(PartialEq, Debug)]
pub enum EffortUnvalidError {
    SyntaxDoesNotMatch(String),
    NumberMissing(usize, String),
    NumberInvalid(usize, String, String),
}
fn extract_effort_from_string(effort_as_string: String) -> Result<Effort, EffortUnvalidError> {
    let capture_numbers_in_effort_representation =
        Regex::new(r"(\d*\.\d*)\t(\d*\.\d*)\n(\d*\.\d*)\t(\d*\.\d*)").expect("This regex is valid");
    let caputered_numbers = capture_numbers_in_effort_representation
        .captures(&effort_as_string)
        .ok_or(EffortUnvalidError::SyntaxDoesNotMatch(
            effort_as_string.clone(),
        ))?;
    let (start_time, start_wattage, end_time, end_wattage) = (
        caputered_numbers
            .get(1)
            .ok_or(EffortUnvalidError::NumberMissing(
                0,
                effort_as_string.clone(),
            ))?
            .as_str()
            .parse::<f64>()
            .map_err(|parse_error| {
                EffortUnvalidError::NumberInvalid(
                    0,
                    format!("{}", parse_error),
                    effort_as_string.clone(),
                )
            })?,
        caputered_numbers
            .get(2)
            .ok_or(EffortUnvalidError::NumberMissing(
                0,
                effort_as_string.clone(),
            ))?
            .as_str()
            .parse::<f64>()
            .map_err(|parse_error| {
                EffortUnvalidError::NumberInvalid(
                    1,
                    format!("{}", parse_error),
                    effort_as_string.clone(),
                )
            })?,
        caputered_numbers
            .get(3)
            .ok_or(EffortUnvalidError::NumberMissing(
                0,
                effort_as_string.clone(),
            ))?
            .as_str()
            .parse::<f64>()
            .map_err(|parse_error| {
                EffortUnvalidError::NumberInvalid(
                    2,
                    format!("{}", parse_error),
                    effort_as_string.clone(),
                )
            })?,
        caputered_numbers
            .get(4)
            .ok_or(EffortUnvalidError::NumberMissing(
                0,
                effort_as_string.clone(),
            ))?
            .as_str()
            .parse::<f64>()
            .map_err(|parse_error| {
                EffortUnvalidError::NumberInvalid(
                    3,
                    format!("{}", parse_error),
                    effort_as_string.clone(),
                )
            })?,
    );
    let duration_in_minutes = end_time - start_time;
    Ok(Effort {
        duration_in_minutes,
        starting_value: start_wattage,
        ending_value: end_wattage,
        gui_state: super::effort::EffortState::default(),
    })
}

pub fn extract_efforts_string(mrc: &str) -> Result<String, ExtractEffortError> {
    let capture_effort_string = Regex::new(r"\[COURSE DATA\]\n((.|\n)+?)\[END COURSE DATA\]")
        .expect("This regex is valid.");
    Ok(capture_effort_string
        .captures(mrc)
        .ok_or(ExtractEffortError::NoEffortsGiven)?
        .get(1)
        .ok_or(ExtractEffortError::NoEffortsGiven)?
        .as_str()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_extract_description {
        use super::*;

        #[test]
        fn actual_description() {
            assert_eq!(
                extract_description(
                    "[COURSE HEADER]\n\
DESCRIPTION = A very good workout.\n\
MINUTES WATTS\n\
[END COURSE HEADER]\n\
[COURSE DATA]\n\
0.00	80.00\n\
10.00	150.00\n\
10.00	300.00\n\
15.00	300.00\n\
15.00	150.00\n\
19.00	150.00\n\
19.00	300.00\n\
26.00	300.00\n\
[END COURSE DATA]"
                ),
                Ok(String::from("A very good workout."))
            )
        }

        #[test]
        fn no_value_for_description() {
            assert_eq!(
                extract_description(
                    "[COURSE HEADER]\n\
DESCRIPTION = no description\n\
MINUTES WATTS\n\
[END COURSE HEADER]\n\
[COURSE DATA]\n\
0.00	80.00\n\
10.00	150.00\n\
10.00	300.00\n\
15.00	300.00\n\
15.00	150.00\n\
19.00	150.00\n\
19.00	300.00\n\
26.00	300.00\n\
[END COURSE DATA]"
                ),
                Ok(String::from(""))
            )
        }
        #[test]
        fn no_description_given() {
            assert_eq!(
                extract_description(
                    "[COURSE HEADER]\n\
MINUTES WATTS\n\
[END COURSE HEADER]\n\
[COURSE DATA]\n\
0.00	80.00\n\
10.00	150.00\n\
10.00	300.00\n\
15.00	300.00\n\
15.00	150.00\n\
19.00	150.00\n\
19.00	300.00\n\
26.00	300.00\n\
[END COURSE DATA]"
                ),
                Err(ExtractDescriptionError::NoDescription)
            )
        }
    }
    mod test_extract_efforts {
        use super::*;

        #[test]
        pub fn extract_first_effort_in_workout() {
            assert_eq!(
                extract_effort_from_string(String::from(
                    "0.00	80.00
10.00	150.00"
                )),
                Ok(Effort {
                    duration_in_minutes: 10.0,
                    starting_value: 80.0,
                    ending_value: 150.0,
                    gui_state: crate::workout_data::effort::EffortState::default()
                })
            )
        }

        #[test]
        pub fn extract_second_effort_in_workout() {
            assert_eq!(
                extract_effort_from_string(String::from(
                    "10.00	300.00
15.00	300.00"
                )),
                Ok(Effort {
                    duration_in_minutes: 5.0,
                    starting_value: 300.0,
                    ending_value: 300.0,
                    gui_state: crate::workout_data::effort::EffortState::default()
                })
            )
        }

        #[test]
        pub fn effort_missing_ending_wattage() {
            assert_eq!(
                extract_effort_from_string(String::from(
                    "10.00	300.00
15.00"
                )),
                Err(EffortUnvalidError::SyntaxDoesNotMatch(String::from(
                    "10.00	300.00
15.00"
                )))
            )
        }
        #[test]
        pub fn effort_is_not_valid_number() {
            assert_eq!(
                extract_effort_from_string(String::from(
                    "10.00	30.0.00
15.00	300.00"
                )),
                Err(EffortUnvalidError::SyntaxDoesNotMatch(String::from(
                    "10.00	30.0.00
15.00	300.00"
                )))
            )
        }

        #[test]
        fn test_split_by_every_other_newline() {
            assert_eq!(
                split_by_every_other_newline(
                    "0.00	80.00
10.00	150.00
10.00	300.00
15.00	300.00
15.00	150.00
19.00	150.00
19.00	300.00
26.00	300.00"
                ),
                vec![
                    "0.00	80.00
10.00	150.00",
                    "10.00	300.00
15.00	300.00",
                    "15.00	150.00
19.00	150.00",
                    "19.00	300.00
26.00	300.00",
                ]
            )
        }

        #[test]
        fn extract_effort_strings() {
            assert_eq!(
                extract_efforts_string(
                    "[COURSE HEADER]
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
[END COURSE DATA]"
                ),
                Ok(String::from(
                    "0.00	80.00
10.00	150.00
10.00	300.00
15.00	300.00
15.00	150.00
19.00	150.00
19.00	300.00
26.00	300.00
"
                ))
            )
        }

        #[test]
        fn simple_workout() {
            assert_eq!(
                extract_efforts(
                    "[COURSE HEADER]
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
[END COURSE DATA]"
                ),
                Ok(vec![
                    Effort::new(10.0, 80.0, Some(150.0)),
                    Effort::new(5.0, 300.0, None),
                    Effort::new(4.0, 150.0, None),
                    Effort::new(7.0, 300.0, None),
                ],)
            )
        }
    }
}
