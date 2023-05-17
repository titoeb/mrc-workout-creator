use iced::Application;
use mrc_workout_creator::gui::mrc_creator;

pub fn main() -> iced::Result {
    mrc_creator::MRCCreator::run(mrc_creator::settings())
}
