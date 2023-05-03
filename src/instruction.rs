use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    instruction: String,
    input: String,
    output: String,
}
