use mongodb::Collection;
use mongodb::bson::Document;
use mongodb::Cursor;
use mongodb::Database;
use mongodb::error::Error;
use mongodb::Client;
use mongodb::bson::doc;
use futures::StreamExt;
use serde_json::Value;
use crate::entity::reservation::Reservation;
use crate::entity::email_reservation::ReservationEmail;
use crate::entity::check_in_date::ReservationCheckInDate;
use crate::entity::check_out_date::ReservationCheckOutDate;


pub trait ReservationSearch {
    async fn search(&self, query: Document) -> Result<Vec<Value>, Error>;
}

impl ReservationSearch for ReservationRepository {
    async fn search(&self, query: Document) -> Result<Vec<Value>, Error> {
        let cursor: Cursor<Reservation> = self.collection.find(query, None).await.unwrap();
        self.serialize_cursor_to_json(cursor).await
    }
}

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
        let query: Document = doc! {"email": &email};
        self.search(query).await
    }

    pub async fn get_reservation_from_check_in_date(&self, reservation_check_in_date: ReservationCheckInDate) -> Result<Vec<Value>, Error> {
        let check_in_date: String = reservation_check_in_date.check_in_date;
        let query: Document = doc! {"check_in_date": &check_in_date};
        self.search(query).await
    }

    pub async fn get_reservation_from_check_out_date(&self, reservation_check_in_date: ReservationCheckOutDate) -> Result<Vec<Value>, Error> {
        let check_out_date: String = reservation_check_in_date.check_out_date;
        let query: Document = doc! {"check_out_date": &check_out_date};
        self.search(query).await
    }

    async fn serialize_cursor_to_json(&self, mut cursor: Cursor<Reservation>) -> Result<Vec<Value>, Error> {
        let mut reservation_result: Vec<Value> = Vec::new();
        while let Some(document) = cursor.next().await {
            match document {
                Ok(document_from_database) => {
                    reservation_result.push(
                        serde_json::to_value(&document_from_database).unwrap()
                    );
                }
                Err(err) => {
                    return Err(err.into());
                }
            }
        }
        Ok(reservation_result)
    }
}

