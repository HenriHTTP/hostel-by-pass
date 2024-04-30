use crate::entity::check_out_date::ReservationCheckOutDate;
use crate::repository::reservation_repository::ReservationRepository;
use crate::service::database;
use crate::helper::message_json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use std::env;


pub async fn get_reservation_check_out_date(Json(reservation_date): Json<ReservationCheckOutDate>) -> impl IntoResponse {
    let collection_name: String = env::var("COLLECTION_NAME").unwrap_or_default();
    let db_name: String = env::var("DATABASE_NAME").unwrap_or_default();
    let uri: String = env::var("MONGO_URL").unwrap_or_default();
    let repository: ReservationRepository = match database::connect(collection_name, db_name, uri).await {
        Ok(repository) => repository,
        Err(error) => {
            let error_json: Value = message_json::send_message_error(400, &error.to_string().as_str()).await;
            return Err((StatusCode::BAD_REQUEST, Json(error_json)));
        }
    };
    let result_from_check_out_date: Vec<Value> = match repository.get_reservation_from_check_out_date(reservation_date).await {
        Ok(content_from_query) => content_from_query,
        Err(error) => {
            let error_json: Value = message_json::send_message_error(500, &error.to_string().as_str()).await;
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_json)));
        }
    };

    if let Err(error) = vec_is_empty(result_from_check_out_date.clone()).await {
        return Err(error);
    }
    Ok((StatusCode::CREATED, Json(json!({"content": result_from_check_out_date}))))
}

async fn vec_is_empty( result_query: Vec<Value>) ->  Result<(), (StatusCode, Json<Value>)>{
    let json_error: Value = message_json::send_message_error(500,"date is not found").await;
    if result_query.is_empty(){
        return Err((StatusCode::BAD_REQUEST,Json(json_error)))
    }
    Ok(())
}