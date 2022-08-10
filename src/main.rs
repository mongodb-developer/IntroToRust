mod in_memory_task_repo;
mod task;

use task::Task;

use crate::in_memory_task_repo::InMemoryTaskRepo;

fn main() {
    let task = Task::new("Do homework".to_string(), None, false);
    println!("Task: {task:?}");
    let _ = InMemoryTaskRepo::new();
}
