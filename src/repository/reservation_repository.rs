use mongodb::{Collection, Database};
use mongodb::error::Error;
use mongodb::Client;
use crate::entity::reservation::Reservation;

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
}