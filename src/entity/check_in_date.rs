use serde::Deserialize;
use serde::Serialize;
use mongodb::bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationCheckInDate {
    pub check_in_date: String
}
