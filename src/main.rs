mod in_memory_task_repo;
mod task;
mod task_repo;

use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::Client;
use std::{env, error::Error, fmt::Debug, time::Duration};
use tokio::time::sleep;
use tokio::{select, spawn};

use task::Task;

use crate::in_memory_task_repo::InMemoryTaskRepo;
use crate::task_repo::TaskRepo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let repo = InMemoryTaskRepo::new();
    // use_task_repo(repo);
    // let value1_future = do_long_lasting_op(2500);
    // let value2_future = do_long_lasting_op(1500);
    // let (value1, value2) = join!(value1_future, value2_future);
    // println!("Value: {value1}");
    // println!("Value: {value2}");

    let client_uri = env::var("MONGODB_URI").expect("You must set MONGODB_URI environment var!");
    let options =
        ClientOptions::parse_with_resolver_config(client_uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;
    let jh_db = spawn(async move { client.list_database_names(None, None).await });
    let jh_progress = spawn(async {
        loop {
            sleep(Duration::from_millis(20)).await;
            print!(".");
        }
    });
    print!("Databases");
    select! {
        _ = jh_progress => {
        }
        db_names = jh_db => {
            let db_names = db_names??;
            println!();
            for name in db_names {
                println!("- {name}");
            }
        }
    }

    Ok(())
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
