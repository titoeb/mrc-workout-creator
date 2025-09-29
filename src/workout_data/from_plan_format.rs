use crate::workout_data::effort::Effort;
use crate::workout_data::workout::Workout;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum ExtractPlanFormatError {
    InvalidFormat,
    InvalidValue,
}
impl From<std::num::ParseFloatError> for ExtractPlanFormatError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ExtractPlanFormatError::InvalidValue
    }
}

fn split_header_and_intervals(
    plan_format: &str,
) -> Result<(String, String), ExtractPlanFormatError> {
    let mut parts = plan_format.splitn(2, "=STREAM=");
    let header = parts
        .next()
        .ok_or(ExtractPlanFormatError::InvalidFormat)?
        .trim()
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("NAME="))
        .and_then(|line| line.strip_prefix("NAME="))
        .ok_or(ExtractPlanFormatError::InvalidFormat)?
        .into();

    let intervals = parts
        .next()
        .ok_or(ExtractPlanFormatError::InvalidFormat)?
        .trim()
        .into();

    Ok((header, intervals))
}
fn split_efforts(intervals_in_plan_format: &str) -> Vec<&str> {
    intervals_in_plan_format
        .split("=INTERVAL=")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect()
}
pub fn extract_workout(workout_as_plan: &str) -> Result<Workout, ExtractPlanFormatError> {
    let (workout_name, efforts) = split_header_and_intervals(workout_as_plan)?;
    let efforts: Vec<Effort> = split_efforts(&efforts)
        .iter()
        .map(|effort| extract_effort_from_string(effort))
        .collect::<Result<Vec<Effort>, ExtractPlanFormatError>>()?;

    Ok(Workout::new(&workout_name, "", efforts))
}

fn extract_effort_from_string(effort_as_string: &str) -> Result<Effort, ExtractPlanFormatError> {
    let extract_metrics_from_workout =
        Regex::new(r"PWR_LO=(\d+)\s+PWR_HI=(\d+)\s+MESG_DURATION_SEC>=(\d+)\?EXIT").unwrap();
    let caps = extract_metrics_from_workout
        .captures(effort_as_string)
        .ok_or(ExtractPlanFormatError::InvalidFormat)?;

    let higher_wattage: f64 = caps
        .get(1)
        .ok_or(ExtractPlanFormatError::InvalidFormat)?
        .as_str()
        .parse()?;
    let lower_wattage: f64 = caps
        .get(2)
        .ok_or(ExtractPlanFormatError::InvalidFormat)?
        .as_str()
        .parse()?;
    let average_wattage = (higher_wattage + lower_wattage) / 2.0;

    let duration_in_seconds: f64 = caps
        .get(3)
        .ok_or(ExtractPlanFormatError::InvalidFormat)?
        .as_str()
        .parse()?;
    let duration_in_minutes = duration_in_seconds / 60.0;
    Ok(Effort::new(duration_in_minutes, average_wattage, None))
}

#[cfg(test)]
mod test {
    use super::{
        extract_effort_from_string, extract_workout, split_efforts, split_header_and_intervals,
        Effort, Workout,
    };

    mod test_extract_description {
        use super::{
            extract_effort_from_string, split_efforts, split_header_and_intervals, Effort,
        };
        #[test]
        fn split_header_and_interval_simple_case() {
            let workout_in_plan_format = "=HEADER=
NAME=20 Minute FTP Test
WORKOUT_TYPE=0
=STREAM=
=INTERVAL=
PWR_LO=50
PWR_HI=50
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=100
PWR_HI=100
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=150
PWR_HI=150
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=200
PWR_HI=200
MESG_DURATION_SEC>=30?EXIT";
            assert_eq!(
                split_header_and_intervals(workout_in_plan_format),
                Ok((
                    "20 Minute FTP Test".into(),
                    "=INTERVAL=
PWR_LO=50
PWR_HI=50
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=100
PWR_HI=100
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=150
PWR_HI=150
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=200
PWR_HI=200
MESG_DURATION_SEC>=30?EXIT"
                        .into()
                ))
            )
        }
        #[test]
        fn split_intervals_simple_case() {
            let intervals_in_plan_format = "=INTERVAL=
PWR_LO=50
PWR_HI=50
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=100
PWR_HI=100
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=150
PWR_HI=150
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=200
PWR_HI=200
MESG_DURATION_SEC>=30?EXIT";
            assert_eq!(
                split_efforts(intervals_in_plan_format),
                vec![
                    "PWR_LO=50
PWR_HI=50
MESG_DURATION_SEC>=30?EXIT",
                    "PWR_LO=100
PWR_HI=100
MESG_DURATION_SEC>=30?EXIT",
                    "PWR_LO=150
PWR_HI=150
MESG_DURATION_SEC>=30?EXIT",
                    "PWR_LO=200
PWR_HI=200
MESG_DURATION_SEC>=30?EXIT"
                ]
            )
        }
        #[test]
        fn extract_effort_from_string_simple_case() {
            let interval_in_plan_format = "PWR_LO=50
PWR_HI=50
MESG_DURATION_SEC>=30?EXIT";
            assert_eq!(
                extract_effort_from_string(interval_in_plan_format),
                Ok(Effort::new(0.5, 50.0, None))
            )
        }
    }
    #[test]
    pub fn extract_workout_simple_case() {
        let workout_in_plan_format = "=HEADER=
NAME=20 Minute FTP Test
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
MESG_DURATION_SEC>=30?EXIT
=INTERVAL=
PWR_LO=200
PWR_HI=200
MESG_DURATION_SEC>=90?EXIT";

        assert_eq!(
            extract_workout(workout_in_plan_format),
            Ok(Workout::new(
                "20 Minute FTP Test",
                "",
                vec![
                    Effort::new(0.5, 50.0, None),
                    Effort::new(1.0, 100.0, None),
                    Effort::new(0.5, 150.0, None),
                    Effort::new(1.5, 200.0, None)
                ]
            ))
        )
    }
}
