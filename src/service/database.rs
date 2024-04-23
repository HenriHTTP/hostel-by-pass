use mongodb::error::Error;
use crate::repository::reservation_repository::ReservationRepository;

pub async fn connect(collection_name: String, db_name: String, uri: String) -> Result<ReservationRepository,Error>{
    let repository: Result<ReservationRepository, Error> = ReservationRepository::new(
        &uri,
        &db_name,
        &collection_name,
    ).await;
    repository
}