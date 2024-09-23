use std::collections::HashMap;

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct Expenses(pub HashMap<Ulid, Expense>);

#[derive(Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: Ulid,
    pub description: String,
    pub amount: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Expenses {
    pub fn read() -> Self {
        let path = dirs::home_dir().unwrap().join(".expenses.json");

        if path.exists() {
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
        } else {
            Self(HashMap::new())
        }
    }

    pub fn write(&self) {
        std::fs::write(
            dirs::home_dir().unwrap().join(".expenses.json"),
            serde_json::to_vec(self).unwrap(),
        )
        .unwrap();
    }

    pub fn add(&mut self, description: String, amount: usize) -> Ulid {
        let id = Ulid::new();

        self.0.insert(
            id,
            Expense {
                id,
                description,
                amount,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        );

        id
    }

    pub fn delete(&mut self, id: Ulid) {
        self.0.remove(&id);
    }

    pub fn update(&mut self, id: Ulid, description: String, amount: usize) {
        let expense = self.0.get_mut(&id).unwrap();

        expense.description = description;
        expense.amount = amount;
    }

    pub fn list(&self) -> Vec<Expense> {
        self.0.values().cloned().collect()
    }

    pub fn summary(&self, month: Option<u32>) -> usize {
        let curr_year = Utc::now().year();

        self.0
            .values()
            .filter_map(|expense| {
                month
                    .is_some_and(|month| {
                        month == expense.updated_at.month()
                            && curr_year == expense.updated_at.year()
                    })
                    .then_some(expense.amount)
            })
            .sum()
    }
}
