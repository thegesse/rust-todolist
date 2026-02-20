use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo{
    pub id: u64,
    pub task: String,
    pub completed: bool,
}
