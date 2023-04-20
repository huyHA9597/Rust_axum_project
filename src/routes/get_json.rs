use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    userName: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "I am Data".to_owned(),
        count: 4,
        userName: "Huy.HaAn".to_owned(),
    };

    Json(data)
}
