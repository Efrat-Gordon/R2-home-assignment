use chrono::{Local, NaiveDate};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct DailyWins {
    pub date: NaiveDate,
    pub count: u32,
}

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<HashMap<String, String>>,
    pub tokens: Arc<Mutex<HashSet<String>>>,
    pub daily_wins: Arc<Mutex<DailyWins>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("a@gmail.com".to_string(), "1234".to_string());

        Self {
            users: Arc::new(users),
            tokens: Arc::new(Mutex::new(HashSet::new())),
            daily_wins: Arc::new(Mutex::new(DailyWins {
                date: Local::now().date_naive(),
                count: 0,
            })),
        }
    }
}
