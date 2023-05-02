use iced::alignment::{self, Alignment};
use iced::event::{self, Event};
use iced::keyboard::{self, KeyCode, Modifiers};
use iced::{Renderer, subscription};
use iced::theme::{self, Theme};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text,
    text_input, Text,
};
use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Font, Length, Settings, Subscription};


use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub fn main() -> iced::Result {
    InsViewer::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    tasks: Vec<Task>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug)]
enum InsViewer {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    instruction: String,
    input: String,
    output: String,

    #[serde(skip)]
    state: TaskState,
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Editing,
    Done,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}

impl Application for InsViewer {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            InsViewer::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("InsViewer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            InsViewer::Loading => {}
            InsViewer::Loaded(_) => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        match self {
            InsViewer::Loading => {
                container(
                    text("Loading...")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .size(50),
                )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_y()
                    .into()
            }
            InsViewer::Loaded(_) => {
                let title = text("todos")
                    .width(Length::Fill)
                    .size(100)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(alignment::Horizontal::Center);

                let content = column![title]
                    .spacing(20)
                    .max_width(800);

                scrollable(
                    container(content)
                        .width(Length::Fill)
                        .padding(40)
                        .center_x(),
                )
                    .into()
            }
        }
    }
}

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
    tasks: Vec<Task>,
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "Iced", "Todos")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("instructions.json");

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}
