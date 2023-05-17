use crate::gui::workout_design::app::{WorkoutDesigner, WorkoutDesignerMessage};
use crate::workout_data::workout;
use iced::executor;
use iced::subscription;
use iced::{window, Application, Command, Element, Settings, Theme};
use rfd::FileDialog;
use std::fs;

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
            WorkoutMessage::Design(WorkoutDesignerMessage::LoadWorkoutPressed) => {
                self.load_workout_from_file();
                Command::none()
            }
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
        subscription::events().map(WorkoutMessage::IcedEvent)
    }
}

impl MRCCreator {
    fn load_workout_from_file(&mut self) {
        if let Some(json_file_to_read) = FileDialog::new()
            .add_filter("Only Select json files", &["json"])
            .pick_file()
        {
            if let Ok(json_to_load) = fs::File::open(json_file_to_read) {
                if let Ok(loaded_workout) =
                    serde_json::from_reader::<fs::File, workout::Workout>(json_to_load)
                {
                    *self = MRCCreator::WorkoutDesign(WorkoutDesigner::from(loaded_workout));
                } else {
                    eprintln!("Invalid Json file.")
                }
            }
        }
    }

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
            size: (1024, 512),
            position: window::Position::default(),
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: true,
            always_on_top: false,
            visible: true,
            platform_specific: window::PlatformSpecific::default(),
            icon: None,
        },
        flags: Default::default(),
        default_font: Default::default(),
        default_text_size: 20.0,
        text_multithreading: false,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    }
}
