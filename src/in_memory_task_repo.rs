use crate::task::Task;

#[derive(Debug)]
pub struct InMemoryTaskRepo {
    tasks: Vec<Task>,
}

impl InMemoryTaskRepo {
    pub fn new() -> InMemoryTaskRepo {
        InMemoryTaskRepo { tasks: vec![] }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn list(&self) -> Vec<Task> {
        self.tasks.clone()
    }
}
