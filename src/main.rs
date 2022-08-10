mod task;

use task::Task;

fn main() {
    let task = Task{
        name: "Do homework".to_string(),
        done: false
    };
    println!("Task: {task:?}");
}
