use serde::{Deserialize, Serialize};
use mongodb::{bson::{doc, Document}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reservation {
    name: String,
    email: String,
    room_number: u32,
    check_in_date: String,
    check_out_date: String,
    num_guests: u32,
    special_requests: Option<Vec<String>>
}

impl Reservation {
    fn to_document(&self) -> Document {
        doc! {
            "name": &self.name,
            "email": &self.email,
            "room_number": &self.room_number,
            "check_in_date": &self.check_in_date,
            "check_out_date": &self.check_out_date,
            "num_guests": &self.num_guests,
        }
    }
}