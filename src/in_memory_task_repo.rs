use crate::task::Task;

struct InMemoryTaskRepo {
    tasks: Vec<Task>,
}

impl InMemoryTaskRepo {
    fn new() -> InMemoryTaskRepo {
        InMemoryTaskRepo { tasks: vec![] }
    }
}
