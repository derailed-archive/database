use mongodb::{Collection, Database};
use actix_web::{post, web, Result, Responder, HttpResponse};
use crate::structs::{User, Settings};

#[post("/users")]
pub async fn create_user(db: web::Data<Database>, json: web::Json<User>) -> Result<impl Responder> {
    let data = json.into_inner();

    let col = db.collection("users");
    let settings_col: Collection<Settings> = db.collection("settings");

    match col.insert_one(data.clone(), None).await {
        Ok(_) => {
            // TODO: handle any possible errors here
            settings_col.insert_one(Settings {user_id: data.id.to_string(), status: "online".to_string(), guild_order: Vec::new()}, None).await.unwrap();
            Ok(HttpResponse::Created())
        },
        Err(_) => Ok(HttpResponse::InternalServerError())
    }
}
