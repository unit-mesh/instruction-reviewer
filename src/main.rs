use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use druid::{AppDelegate, AppLauncher, Color, Command, commands, Data, DelegateCtx, Env, FileDialogOptions, FileSpec, Handled, Lens, LocalizedString, PlatformError, Target, UnitPoint, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Flex, Label, TextBox};
use polars::frame::DataFrame;
use polars::prelude::{JsonFormat, JsonLineReader, JsonReader, JsonWriter, SerReader, SerWriter};

use ins_viewer_delegate::Delegate;

use crate::instruction::Instruction;

pub mod instruction;
pub mod errors;
pub mod ins_viewer_delegate;

#[derive(Clone, Data, Lens)]
struct AppState {
    count: String,
    #[data(eq)]
    instructions: Vec<Instruction>,
    #[data(ignore)]
    df: Arc<DataFrame>,
}

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
    let mut col = Flex::column();

    col.scroll()
}

