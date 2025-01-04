use iced::application;
use mrc_creator::MRCCreator;
use mrc_workout_creator::gui::mrc_creator;

pub fn main() -> iced::Result {
    application("Workout Generator", MRCCreator::update, MRCCreator::view)
        .settings(mrc_creator::settings())
        .window(mrc_creator::window_settings())
        .theme(MRCCreator::theme)
        .subscription(MRCCreator::subscription)
        .run_with(MRCCreator::new)
}
