//! This crate contains an application to build workouts in the mrc format.
#![allow(clippy::large_enum_variant)]

/// Definitions of all base types to construct a workout.
pub mod workout_data;

/// The GUI of the Workout Generator application.
pub mod gui;

#[cfg(test)]
/// Testing utilities
pub mod testing;
