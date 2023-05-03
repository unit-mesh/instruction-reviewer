use serde::{Deserialize, Serialize};

use crate::instructions::TaskState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    instruction: String,
    input: String,
    output: String,

    #[serde(skip)]
    state: TaskState,
}
