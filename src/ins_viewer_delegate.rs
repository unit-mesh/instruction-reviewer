use druid::{AppDelegate, Command, commands, DelegateCtx, Env, Handled, Target};
use std::fs::File;
use std::sync::Arc;
use polars::prelude::{JsonFormat, JsonLineReader, JsonWriter};
use polars::io::{SerReader, SerWriter};
use crate::AppState;

pub struct Delegate;

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
                            data.count = df.height() as u32;
                            data.df = Arc::new(df);
                        }
                        Some("jsonl") => {
                            let mut df = JsonLineReader::new(&mut file).finish().unwrap();
                            data.count = df.height() as u32;
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
