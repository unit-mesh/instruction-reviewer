use std::io::Bytes;
use iced::alignment::{self, Alignment};
use iced::event::{self, Event};
use iced::keyboard::{self, KeyCode, Modifiers};
use iced::{Renderer, subscription};
use iced::theme::{self, Theme};
use iced::widget::{self, button, checkbox, column, container, row, scrollable, text, text_input, Text, TextInput};
use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Font, Length, Settings, Subscription};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;

pub fn main() -> iced::Result {
    let raw = font_kit::source::SystemSource::new();
    let families = vec![
        FamilyName::Title("PingFang SC".to_string()),
        FamilyName::Serif, FamilyName::SansSerif, FamilyName::Monospace,
    ];
    let font = raw.select_best_match(
        &families,
        &font_kit::properties::Properties::default(),
    ).expect("no found font");

    let mut buf: Vec<u8> = Vec::new();

    match font {
        font_kit::handle::Handle::Path { path, .. } => {
            use std::io::Read;

            let mut reader = std::fs::File::open(path).expect("Read font");
            let _ = reader.read_to_end(&mut buf);
        }
        font_kit::handle::Handle::Memory { bytes, .. } => {
            buf = bytes.to_vec();
        }
    }

    let buf = Box::leak(buf.to_vec().into_boxed_slice());

    run_main(buf)
}

fn run_main(buf: &'static [u8]) -> Result<(), iced::Error> {
    InsViewer::run(Settings {
        window: window::Settings {
            size: (1600, 900),
            ..window::Settings::default()
        },
        default_font: Some(buf),
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct State {
    tasks: Vec<Instruction>,
    dirty: bool,
    current_page: u32,
    saving: bool,
}

#[derive(Debug)]
enum InsViewer {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Instruction {
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

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    TabPressed { shift: bool },
    TaskMessage(usize, TaskMessage),
    ToggleFullscreen(window::Mode),
}

impl Application for InsViewer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
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
            InsViewer::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = InsViewer::Loaded(State {
                            tasks: state.tasks,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = InsViewer::Loaded(State::default());
                    }
                    _ => {}
                }

                Command::none()
            }
            InsViewer::Loaded(state) => {
                let command = match message {
                    Message::Saved(Ok(())) => {
                        state.saving = false;
                        Command::none()
                    }
                    Message::Saved(Err(_)) => {
                        state.saving = false;
                        Command::none()
                    }
                    Message::InputChanged(input) => {
                        state.dirty = true;
                        state.tasks[state.current_page as usize].input = input;
                        Command::none()
                    }
                    Message::TabPressed { shift } => {
                        if shift {
                            if state.current_page > 0 {
                                state.current_page -= 1;
                            }
                        } else {
                            state.current_page += 1;
                        }

                        Command::none()
                    }
                    Message::TaskMessage(i, task_message) => {
                        // state.dirty = true;
                        // state.tasks[i].update(task_message);
                        Command::none()
                    }
                    Message::ToggleFullscreen(mode) => {
                        window::change_mode(mode)
                    }
                    _ => {
                        Command::none()
                    }
                };

                Command::batch(vec![command])
            }
        }
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
            InsViewer::Loaded(State { tasks, current_page, .. }) => {
                let title = text("Reviewer")
                    .width(Length::Fill)
                    .size(24)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(alignment::Horizontal::Center);

                let tasks: Vec<_> = tasks
                    .iter()
                    .skip((current_page * 50) as usize)
                    .take(50)
                    .collect();

                let tasks: Element<_> = column(
                    tasks
                        .iter()
                        .enumerate()
                        .map(|(i, task)| {
                            task.view(i).map(move |message| {
                                Message::TaskMessage(i, message)
                            })
                        })
                        .collect(),
                )
                    .spacing(10)
                    .into();

                let content = column![title, tasks]
                    .spacing(20)
                    .max_width(1800);

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

impl Instruction {
    fn view(&self, i: usize) -> Element<TaskMessage> {
        let instruction = TextInput::new("instruction", &self.instruction)
            .on_input(TaskMessage::InstructionChanged)
            .width(Length::Fill);

        let input = TextInput::new("input", &self.input)
            .on_input(TaskMessage::InputChanged)
            .width(Length::Fill);

        let output = Text::new(&self.output)
            .width(Length::Fill);

        column![
                instruction,
                row![
                    input, output
                ]
            ]
            .spacing(20)
            .align_items(Alignment::Center)
            .into()
    }
}

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    tasks: Vec<Instruction>,
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        path.push("instructions.jsonl");
        println!("path: {:?}", path);

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        Self::load_jsonl().await
    }

    async fn load_jsonl() -> Result<SavedState, LoadError> {
        use async_std::fs::File;
        use async_std::io::BufReader;
        use async_std::prelude::*;

        let mut instructions: Vec<Instruction> = Vec::new();
        let mut lines = BufReader::new(File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?)
            .lines();

        while let Some(line) = lines.next().await {
            let instruction = serde_json::from_str(&line.map_err(|_| LoadError::File)?).map_err(|_| LoadError::Format)?;
            instructions.push(instruction);
        }

        Ok(SavedState {
            tasks: instructions,
        })
    }

    async fn load_json() -> Result<SavedState, LoadError> {
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
pub enum TaskMessage {
    Completed(bool),
    Edit,
    InstructionChanged(String),
    InputChanged(String),
    FinishEdition,
    Delete,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}
