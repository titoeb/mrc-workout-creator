use crm_workout_creator::gui::crm_creator;
use iced::Sandbox;

pub fn main() -> iced::Result {
    crm_creator::CRMCreator::run(crm_creator::settings())
}
