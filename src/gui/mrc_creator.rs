use crate::gui::workout_design::app::{WorkoutDesigner, WorkoutDesignerMessage};
use iced::event::listen_with;
use iced::executor;
use iced::window::settings::PlatformSpecific;
use iced::{window, Application, Command, Element, Settings, Theme};
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

impl Application for MRCCreator {
    type Message = WorkoutMessage;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Workout Generator")
    }

    fn update(&mut self, message: WorkoutMessage) -> Command<Self::Message> {
        match message {
            WorkoutMessage::Design(_) => self.handle_subpage_messages(message),
            WorkoutMessage::IcedEvent(event) => self.handle_iced_events(event),
        }
    }

    fn view(&self) -> Element<WorkoutMessage> {
        match self {
            MRCCreator::WorkoutDesign(workout_designer) => workout_designer.view(),
        }
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        listen_with(|event, _| Some(event)).map(WorkoutMessage::IcedEvent)
    }
}

impl MRCCreator {
    fn handle_subpage_messages(&mut self, message: WorkoutMessage) -> Command<WorkoutMessage> {
        match self {
            MRCCreator::WorkoutDesign(workout_designer) => {
                if let WorkoutMessage::Design(design_message) = message {
                    workout_designer.update(design_message)
                } else {
                    Command::none()
                }
            }
        }
    }

    fn handle_iced_events(&mut self, event: iced::Event) -> Command<WorkoutMessage> {
        match self {
            MRCCreator::WorkoutDesign(workout_designer) => {
                workout_designer.update(WorkoutDesignerMessage::IcedEvent(event))
            }
        }
    }
}

pub fn settings<Flags>() -> Settings<Flags>
where
    Flags: Default,
{
    Settings {
        id: None,
        window: window::Settings {
            size: Size {
                width: 1400.0,
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
        },
        flags: Default::default(),
        default_font: Default::default(),
        default_text_size: iced::Pixels(20.0),
        antialiasing: false,
        ..Settings::default()
    }
}
