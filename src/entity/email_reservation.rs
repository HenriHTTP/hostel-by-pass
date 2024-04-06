use serde::Deserialize;
use serde::Serialize;
use mongodb::bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationEmail {
    pub email: String
}
