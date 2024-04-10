use mongodb::Collection;
use mongodb::Cursor;
use mongodb::Database;
use mongodb::error::Error;
use mongodb::Client;
use mongodb::bson::doc;
use futures::StreamExt;
use serde_json::Value;
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
    pub async fn get_reservation_from_email(&self, reservation_email: ReservationEmail) -> Result<Vec<Value>, Error> {
        let email: String = reservation_email.email;
        let mut cursor: Cursor<Reservation> = self.collection.find(doc! {"email": &email}, None).await.unwrap();
        let mut reservation_result: Vec<Value> = Vec::new();
        while let Some(document) = cursor.next().await {
            match document {
                Ok(document_from_database) => {
                    reservation_result.push(
                        serde_json::to_value(&document_from_database).unwrap()
                    );
                }
                Err(err) => return Err(err.into()),
            }
        }
        Ok(reservation_result)
    }
}