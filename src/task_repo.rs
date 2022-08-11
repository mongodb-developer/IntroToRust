use crate::task::Task;

trait TaskRepo {
    fn add(&mut self, task: Task);
    fn list(&self) -> Vec<Task>;
}
