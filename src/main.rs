pub mod instruction;
pub mod errors;

use std::path::PathBuf;
use std::sync::Arc;
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{
    AppDelegate, AppLauncher, Data, DelegateCtx, Lens, LocalizedString, PlatformError,
    Widget, WidgetExt, WindowDesc, commands, Command, Env, FileDialogOptions, FileSpec, Handled, Target,
};
use crate::instruction::Instruction;

#[derive(Clone, Data, Lens)]
struct AppState {
    demo: String,
    #[data(eq)]
    instructions: Vec<Instruction>,
    #[data(ignore)]
    file_path: Option<PathBuf>,
}

struct Delegate;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()).title("InsViewer");

    let initial_state = AppState {
        demo: "Type here".to_string(),
        instructions: Vec::new(),
        file_path: None,
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(initial_state)
}

fn ui_builder() -> impl Widget<AppState> {
    let instruction = TextBox::new()
        .with_placeholder("Enter instruction")
        .lens(AppState::demo)
        .expand_width();

    Flex::column()
        .with_flex_child(
            Flex::row()
                .with_child(instruction),
            1.0,
        )
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
            todo!("Save the instruction to a file");
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match file_info.path().extension() {
                Some(ext) => {
                    match ext.to_str() {
                        Some("json") => {}
                        Some("jsonl") => {}
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