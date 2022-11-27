use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Bill {
    bill_id: u64,
    bill_price: f64,
}
