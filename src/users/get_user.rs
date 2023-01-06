use mongodb::{Database, bson::doc, Collection};
use actix_web::{get, web, HttpResponse};
use crate::structs::User;

#[get("/users/{user_id}")]
pub async fn get_user(db: web::Data<Database>, user_id: web::Path<i64>) -> HttpResponse {
    let col: Collection<User> = db.collection("users");
    let user_id = user_id.to_owned().to_string();

    match col.find_one(
        doc! {"_id": &user_id}, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("Not Found".to_string()),
        Err(msg) => HttpResponse::InternalServerError().body(msg.to_string())
    }
}
