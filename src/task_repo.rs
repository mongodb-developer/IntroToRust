use crate::task::Task;

pub trait TaskRepo {
    fn add(&mut self, task: Task);
    fn list(&self) -> Vec<Task>;
}
