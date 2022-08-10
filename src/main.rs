mod task;

use task::Task;

fn main() {
    let task = Task::new("Do homework".to_string(), None, false);
    println!("Task: {task:?}");
}
