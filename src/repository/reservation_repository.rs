use mongodb::Collection;
use crate::entity::reservation::Reservation;

pub struct ReservationRepository {
    collection: Collection<Reservation>,
}