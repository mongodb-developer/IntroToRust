use crate::task::Task;

pub struct InMemoryTaskRepo {
    tasks: Vec<Task>,
}

impl InMemoryTaskRepo {
    pub fn new() -> InMemoryTaskRepo {
        InMemoryTaskRepo { tasks: vec![] }
    }
}
