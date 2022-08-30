mod in_memory_task_repo;
mod task;
mod task_repo;

use std::{fmt::Debug, time::Duration};
use tokio::join;
use tokio::time::sleep;

use task::Task;

use crate::in_memory_task_repo::InMemoryTaskRepo;
use crate::task_repo::TaskRepo;

#[tokio::main]
async fn main() {
    // let repo = InMemoryTaskRepo::new();
    // use_task_repo(repo);
    let value1_future = do_long_lasting_op(2500);
    let value2_future = do_long_lasting_op(1500);
    let (value1, value2) = join!(value1_future, value2_future);
    println!("Value: {value1}");
    println!("Value: {value2}");
}

fn use_task_repo<T: TaskRepo + Debug>(mut repo: T) {
    let task = Task::new("Do homework".to_string(), None, false);
    println!("Task: {task:?}");
    repo.add(task);
    println!("Repo: {repo:?}");
    println!("Tasks in repo: {:?}", repo.list());
    println!("Repo info: {repo}");
}

async fn do_long_lasting_op(ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("Hola from the future ({ms} ms)")
}