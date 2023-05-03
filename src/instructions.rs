use serde::{Deserialize, Serialize};
use crate::errors::LoadError;

use crate::instruction::Instruction;


// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instructions {
    tasks: Vec<Instruction>,
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

impl Instructions {
    pub fn path() -> std::path::PathBuf {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        path.push("instructions.jsonl");
        println!("path: {:?}", path);

        path
    }

    async fn load() -> Result<Instructions, LoadError> {
        Self::load_jsonl().await
    }

    async fn load_jsonl() -> Result<Instructions, LoadError> {
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

        Ok(Instructions {
            tasks: instructions,
        })
    }

    async fn load_json() -> Result<Instructions, LoadError> {
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
