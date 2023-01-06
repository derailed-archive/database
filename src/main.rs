use actix_web::{HttpServer, App, web};
use mongodb::Client;
mod users;
mod structs;
mod grpc;
mod guilds;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("Unable to connect to Database");
    let db = client.database("derailed");
    let (guild_channel, user_channel) = grpc::connect().await;

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(structs::CoreStruct {
            db: db.clone(),
            gc: guild_channel.clone(),
            uc: user_channel.clone()
        }))
        .service(users::create_user::create_user)
        .service(users::get_user::get_user)
        .service(guilds::get_guild_preview::get_guild_preview)
    })
    .bind(("0.0.0.0", 4700))?
    .run()
    .await
}