use mongodb::Collection;
use mongodb::Cursor;
use mongodb::Database;
use mongodb::error::Error;
use mongodb::Client;
use mongodb::bson::doc;
use serde_json::to_string;
use futures::StreamExt;
use mongodb::bson::Document;
use mongodb::bson::from_document;
use crate::entity::reservation::Reservation;
use crate::entity::email_reservation::ReservationEmail;


pub struct ReservationRepository {
    collection: Collection<Reservation>,
}

impl ReservationRepository {
    pub async fn new(uri: &str, db_name: &str, collection_name: &str) -> Result<Self, Error> {
        let client: Client = Client::with_uri_str(uri).await?;
        let database: Database = client.database(db_name);
        let collection: Collection<Reservation> = database.collection(collection_name);
        Ok(ReservationRepository { collection })
    }
    pub async fn insert(&self, reservation: Reservation) -> Result<(), Error> {
        self.collection.insert_one(reservation, None).await?;
        Ok(())
    }
    pub async fn get_reservation_by_email(&self, reservation_email: ReservationEmail) -> Result<(), Error> {
        let email = reservation_email.email;
        let _ = self.collection.find(doc! {"email": &email}, None).await;
        Ok(())
    }
}