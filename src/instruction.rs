use serde::{Deserialize, Serialize};
use druid::Data;

#[derive(Debug, Clone, Data, Eq, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub instruction: String,
    pub input: String,
    pub output: String,
}
