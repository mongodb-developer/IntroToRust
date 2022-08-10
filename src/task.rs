use chrono::{NaiveDate, Utc};

#[derive(Debug)]
pub struct Task {
    pub name: String,
    created: NaiveDate,
    pub due_date: Option<NaiveDate>,
    pub done: bool,
}

impl Task {
    pub fn new(name: String, due_date: Option<NaiveDate>, done: bool) -> Task {
        Task {
            name,
            created: Utc::today().naive_utc(),
            due_date,
            done,
        }
    }
}
