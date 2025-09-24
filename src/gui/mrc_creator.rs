use crate::gui::workout_design::app::{WorkoutDesigner, WorkoutDesignerMessage};
use iced::event::listen_with;
use iced::window::settings::PlatformSpecific;
use iced::Task;
use iced::{window, Element, Settings, Theme};
use iced_core::Size;
/// Holding the state of the overall CRMCreator Application.
pub enum MRCCreator {
    WorkoutDesign(WorkoutDesigner),
}

impl Default for MRCCreator {
    fn default() -> Self {
        MRCCreator::WorkoutDesign(WorkoutDesigner::default())
    }
}

#[derive(Debug, Clone)]
pub enum WorkoutMessage {
    Design(WorkoutDesignerMessage),
    IcedEvent(iced::Event),
}

impl From<WorkoutDesignerMessage> for WorkoutMessage {
    fn from(workout_designer_message: WorkoutDesignerMessage) -> Self {
        Self::Design(workout_designer_message)
    }
}

impl MRCCreator {
    pub fn new() -> (Self, Task<WorkoutMessage>) {
        (Self::default(), Task::none())
    }

    pub fn update(&mut self, message: WorkoutMessage) -> Task<WorkoutMessage> {
        match message {
            WorkoutMessage::Design(_) => self.handle_subpage_messages(message),
            WorkoutMessage::IcedEvent(event) => self.handle_iced_events(event),
        }
    }

    pub fn view(&'_ self) -> Element<'_, WorkoutMessage> {
        match self {
            MRCCreator::WorkoutDesign(workout_designer) => workout_designer.view(),
        }
    }
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn subscription(&self) -> iced::Subscription<WorkoutMessage> {
        listen_with(|event, _, _| Some(event)).map(WorkoutMessage::IcedEvent)
    }
}

impl MRCCreator {
    fn handle_subpage_messages(&mut self, message: WorkoutMessage) -> Task<WorkoutMessage> {
        match self {
            MRCCreator::WorkoutDesign(workout_designer) => {
                if let WorkoutMessage::Design(design_message) = message {
                    workout_designer.update(design_message)
                } else {
                    Task::none()
                }
            }
        }
    }

    fn handle_iced_events(&mut self, event: iced::Event) -> Task<WorkoutMessage> {
        match self {
            MRCCreator::WorkoutDesign(workout_designer) => {
                workout_designer.update(WorkoutDesignerMessage::IcedEvent(event))
            }
        }
    }
}

pub fn settings() -> Settings {
    Settings {
        default_text_size: iced::Pixels(20.0),
        antialiasing: false,
        ..Settings::default()
    }
}

pub fn window_settings() -> window::settings::Settings {
    window::Settings {
        size: Size {
            width: 1550.0,
            height: 800.0,
        },
        position: window::Position::default(),
        min_size: None,
        max_size: None,
        resizable: true,
        decorations: true,
        transparent: true,
        visible: true,
        level: window::Level::AlwaysOnTop,
        platform_specific: PlatformSpecific::default(),
        icon: None,
        exit_on_close_request: true,
    }
}
