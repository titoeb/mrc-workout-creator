use crate::workout_data::workout::WorkoutType;

impl WorkoutType {
    pub const ALL: [WorkoutType; 2] = [WorkoutType::Watts, WorkoutType::PercentOfFTP];
}

impl std::fmt::Display for WorkoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkoutType::Watts => "Watts",
                WorkoutType::PercentOfFTP => "Percentage of FTP",
            }
        )
    }
}
