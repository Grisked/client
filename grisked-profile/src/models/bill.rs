use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Bill {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub due_date: u16,
}

impl Bill {
    pub fn new(id: u64, name: String, price: f64, due_date: u16) -> Self {
        Self {
            id,
            name,
            price,
            due_date,
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
