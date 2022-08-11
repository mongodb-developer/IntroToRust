mod in_memory_task_repo;
mod task;
mod task_repo;

use std::fmt::Debug;

use task::Task;

use crate::in_memory_task_repo::InMemoryTaskRepo;
use crate::task_repo::TaskRepo;

fn main() {
    let repo = InMemoryTaskRepo::new();
    use_task_repo(repo);
}

fn use_task_repo<T: TaskRepo + Debug>(mut repo: T) {
    let task = Task::new("Do homework".to_string(), None, false);
    println!("Task: {task:?}");
    repo.add(task);
    println!("Repo: {repo:?}");
    println!("Tasks in repo: {:?}", repo.list());
    println!("Repo info: {repo}");
}
