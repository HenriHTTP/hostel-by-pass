use serde::Deserialize;
use serde::Serialize;
use mongodb::bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reservation {
    pub name: String,
    pub email: String,
    pub room_number: u32,
    pub check_in_date: String,
    pub check_out_date: String,
    pub num_guests: u32,
    pub special_requests: Option<Vec<String>>,
}
