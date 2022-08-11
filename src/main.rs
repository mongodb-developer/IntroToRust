mod in_memory_task_repo;
mod task;
mod task_repo;

use task::Task;

use crate::in_memory_task_repo::InMemoryTaskRepo;

fn main() {
    let task = Task::new("Do homework".to_string(), None, false);
    println!("Task: {task:?}");
    let mut repo = InMemoryTaskRepo::new();
    repo.add(task);
    println!("Repo: {repo:?}");
    println!("Tasks in repo: {:?}", repo.list())
}
