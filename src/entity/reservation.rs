use serde::Deserialize;
use serde::Serialize;
use mongodb::bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reservation {
    name: String,
    email: String,
    room_number: u32,
    check_in_date: String,
    check_out_date: String,
    num_guests: u32,
    special_requests: Option<Vec<String>>,
}
