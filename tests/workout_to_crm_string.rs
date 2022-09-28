use crm_workout_creator::workout_data::positive_float::PositiveFloat;
use crm_workout_creator::workout_data::workout::{
    Effort, EffortUnit, PercentOfFTP, Watts, Workout,
};

// #[test]
// fn simple_watt_workout_to_crm() {
//     assert_eq!(
//         Workout::new(
//             "test_workout",
//             "test-1",
//             vec![
//                 Effort::SingleEffort(EffortUnit::new(
//                     PositiveFloat::new(5.0).unwrap(),
//                     Watts::new(
//                         PositiveFloat::new(80.0).unwrap()
//                     ),
//                 )),
//                 Effort::SingleEffort(EffortUnit::new(
//                     PositiveFloat::new(5.0)..unwrap(),
//                     Watts::new(
//                         PositiveFloat::new(80.0)..unwrap()
//                     ),
//                 )),
//             ],
//         )
//         .to_crm(),
//         "[COURSE HEADER]\n\
//     DESCRIPTION = test-1\n\
//     MINUTES PERCENTAGE\n\
//     [END COURSE HEADER]\n\
//     [COURSE DATA]\n\
//     0.00	80.00\n\
//     5.00	80.00\n\
//     5.00	100.00\n\
//     10.00	100.00\n\
//     [END COURSE DATA]"
//     )
// }

// #[test]
// fn simple_percent_of_ftp_workout_to_crm() {
//     assert_eq!(
//         Workout::new(
//             "test_workout",
//             "test-1",
//             vec![
//                 Effort::SingleEffort(EffortUnit::new(
//                     PositiveFloat::new(5.0).unwrap(),
//                     PercentOfFTP::new(
//                         PositiveFloat::new(80.0).unwrap()
//                     ),
//                 )),
//                 Effort::SingleEffort(EffortUnit::new(
//                     PositiveFloat::new(5.0).unwrap(),
//                     PercentOfFTP::new(
//                         PositiveFloat::new(80.0).unwrap()
//                     ),
//                 )),
//             ],
//         )
//         .to_crm(),
//         "[COURSE HEADER]\n\
//         DESCRIPTION = test-1\n\
//         MINUTES PERCENTAGE\n\
//         [END COURSE HEADER]\n\
//         [COURSE DATA]\n\
//         0.0	80.0\n\
//         5.0	80.0\n\
//         5.0	100.0\n\
//         10.0	100.0\n\
//         [END COURSE DATA]"
//     )
// }
