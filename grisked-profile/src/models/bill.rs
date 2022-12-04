use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Bill {
    pub name: String,
    pub price: f64,
    pub due_date: u16,
    pub label_id: Option<usize>,
}

impl Bill {
    pub fn new(name: String, price: f64, due_date: u16, label_id: Option<usize>) -> Self {
        Self {
            name,
            price,
            due_date,
            label_id,
        }
    }

    pub fn pretty_price(&self) -> String {
        if self.price < 0.0 {
            format!("{:0.2} €", self.price)
        } else {
            format!("+{:0.2} €", self.price)
        }
    }
}
