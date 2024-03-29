use mongodb::Collection;
use mongodb::error::Error;
use crate::entity::reservation::Reservation;

pub struct ReservationRepository {
    collection: Collection<Reservation>,
}

impl ReservationRepository {
    pub fn new(collection: Collection<Reservation>) -> Self {
        ReservationRepository { collection }
    }
    pub async fn insert(&self, reservation: Reservation) -> Result<(), Error> {
        self.collection.insert_one(reservation, None).await?;
        Ok(())
    }
}