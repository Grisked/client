use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum BillType {
    Income,
    Invoice,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Bill {
    #[serde(rename = "type")]
    pub bill_type: BillType,
    pub name: String,
    pub price: f64,
    pub due_date: u16,
    pub label_id: Option<usize>,
}

impl Bill {
    pub fn new(
        bill_type: BillType,
        name: String,
        mut price: f64,
        due_date: u16,
        label_id: Option<usize>,
    ) -> Self {
        price = price.abs();

        Self {
            bill_type,
            name,
            price,
            due_date,
            label_id,
        }
    }

    pub fn pretty_price(&self) -> String {
        if self.bill_type == BillType::Invoice {
            format!("-{:0.2} €", self.price)
        } else {
            format!("+{:0.2} €", self.price)
        }
    }
}
