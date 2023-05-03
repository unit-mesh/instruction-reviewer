use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use druid::{im, AppDelegate, AppLauncher, BoxConstraints, Color, Command, commands, Data, DelegateCtx, Env, Event, EventCtx, FileDialogOptions, FileSpec, Handled, Insets, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, LocalizedString, PaintCtx, PlatformError, RadialGradient, Rect, RenderContext, Size, Target, UnitPoint, UpdateCtx, Widget, WidgetExt, WindowDesc};
use druid::kurbo::Circle;
use druid::widget::{Button, Flex, Label, List, Padding, Painter, Scroll, TextBox};
use polars::frame::DataFrame;
use polars::prelude::{JsonFormat, JsonLineReader, JsonReader, JsonWriter, SerReader, SerWriter};

use ins_viewer_delegate::Delegate;

use crate::instruction::Instruction;

pub mod instruction;
pub mod errors;
pub mod ins_viewer_delegate;

#[derive(Clone, Data, Lens)]
struct AppState {
    count: u32,
    #[data(eq)]
    instructions: im::Vector<Instruction>,
    #[data(ignore)]
    df: Arc<DataFrame>,
}

fn main() -> Result<(), PlatformError> {
    let initial_state = AppState {
        count: 0,
        instructions: im::Vector::new(),
        df: Arc::new(DataFrame::default()),
    };

    let main_window = WindowDesc::new(ui_builder()).title("InsViewer");

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(initial_state)
}


const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

fn ui_builder() -> impl Widget<AppState> {
    Scroll::new(
        Flex::column().with_child(label_widget(
            List::new(|| {
                Label::new(|data: &Instruction, _: &_| format!("{}", data.instruction))
                    .center()
                    .background(Color::hlc(230.0, 50.0, 50.0))
                    .fix_height(40.0)
                    .expand_width()
            })
                .lens(AppState::instructions),
            "List",
        ))
    )
}

fn label_widget<T: Data>(widget: impl Widget<T> + 'static, label: &str) -> impl Widget<T> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_flex_child(widget.center(), 1.0)
        .with_child(
            Painter::new(|ctx, _: &_, _: &_| {
                let size = ctx.size().to_rect();
                ctx.fill(size, &Color::WHITE)
            })
                .fix_height(1.0),
        )
        .with_child(Label::new(label).center().fix_height(40.0))
        .border(Color::WHITE, 1.0)
}
