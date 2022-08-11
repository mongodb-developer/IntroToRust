use std::fmt::Display;

use crate::task::Task;

pub trait TaskRepo: Display {
    fn add(&mut self, task: Task);
    fn list(&self) -> Vec<Task>;
}
