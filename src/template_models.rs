use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IndexModel{
    pub data_table: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataTableModel{
    pub ip: String,
    pub visitor_rank: u32,
    pub last_start_time: String,
    pub total_visitors: u32,
}

