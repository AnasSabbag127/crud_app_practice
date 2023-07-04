use serde::{ Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User{
    user_id : Uuid,
    user_name : String,
}