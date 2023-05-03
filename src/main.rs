pub mod instruction;
pub mod errors;

use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppDelegate, AppLauncher, Data, DelegateCtx, Lens, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, commands, Command, Env, FileDialogOptions, FileSpec, Handled, Target, UnitPoint};
use polars::frame::DataFrame;
use polars::prelude::{JsonFormat, JsonWriter, SerWriter, JsonReader, JsonLineReader, SerReader};
use crate::instruction::Instruction;

#[derive(Clone, Data, Lens)]
struct AppState {
    count: String,
    #[data(eq)]
    instructions: Vec<Instruction>,
    #[data(ignore)]
    df: Arc<DataFrame>,
}

struct Delegate;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()).title("InsViewer");

    let initial_state = AppState {
        count: 0.to_string(),
        instructions: Vec::new(),
        df: Arc::new(DataFrame::default()),
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(initial_state)
}


const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

fn ui_builder() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _env: &Env| format!("Count: {}", data.count))
        .with_text_size(32.0);

    //
    // let count = TextBox::new()
    //     .with_placeholder("Count")
    //     .lens(AppState::count)
    //     .expand_width();

    // Flex::column()
    //     .with_flex_child(
    //         Flex::row()
    //             .with_child(instruction),
    //         1.0,
    //     )

    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .align_vertical(UnitPoint::CENTER)
}

// TODO: modify this to save the instruction to a file
impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            let mut file = File::open(file_info.path()).unwrap();
            let mut df = Arc::get_mut(&mut data.df).unwrap();
            JsonWriter::new(&mut file)
                .with_json_format(JsonFormat::JsonLines)
                .finish(&mut df)
                .unwrap()
        }

        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            let mut file = File::open(file_info.path()).unwrap();
            match file_info.path.extension() {
                Some(ext) => {
                    match ext.to_str() {
                        Some("json") => {
                            use polars::prelude::*;

                            let mut df = JsonReader::new(&mut file).finish().unwrap();
                            data.count = df.height().to_string();
                            data.df = Arc::new(df);
                        }
                        Some("jsonl") => {
                            let mut df = JsonLineReader::new(&mut file).finish().unwrap();
                            data.count = df.height().to_string();
                            data.df = Arc::new(df);
                        }
                        _ => println!("Invalid file type"),
                    }
                }
                None => println!("Invalid file type"),
            }
            return Handled::Yes;
        }
        Handled::No
    }
}