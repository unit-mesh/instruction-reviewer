pub mod instructions;
pub mod instruction;
pub mod errors;

use std::sync::Arc;
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppState {
    instruction: Arc<String>,
    input: Arc<String>,
    output: Arc<String>,
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());

    // create the initial app state
    let initial_state = AppState {
        instruction: "".to_string().into(),
        input: "".to_string().into(),
        output: "".to_string().into(),
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
}

fn ui_builder() -> impl Widget<AppState> {
    let instruction = TextBox::new()
        .with_placeholder("Enter instruction")
        .lens(AppState::instruction)
        .expand_width();

    let input = TextBox::multiline()
        .with_placeholder("Input")
        .lens(AppState::input)
        .expand_width();

    let output = TextBox::multiline()
        .with_placeholder("Output")
        .lens(AppState::output)
        .expand_width();

    Flex::column()
        .with_flex_child(
            Flex::row()
                .with_child(instruction),
            1.0,
        )
        .with_flex_child(
            Flex::row()
                .with_child(input)
                .with_child(output),
            1.0,
        )
}