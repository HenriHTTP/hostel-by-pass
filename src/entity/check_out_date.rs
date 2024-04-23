use serde::Deserialize;
use serde::Serialize;
use mongodb::bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationCheckOutDate {
    pub check_out_date: String
}