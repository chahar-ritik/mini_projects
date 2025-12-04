use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub task: String,
    pub is_done: String,
}

pub fn save_task(tasks: &Vec<Task>) {
    let json = serde_json::to_string(&tasks).unwrap();
    fs::write("taskmanager.json", json).unwrap();
}

pub fn load_task() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string("taskmanager.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}
