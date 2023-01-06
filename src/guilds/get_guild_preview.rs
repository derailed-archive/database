use mongodb::{bson::doc, Collection};
use actix_web::{get, web::{Data, Path}, HttpResponse};
use crate::structs::{Guild, CoreStruct};
use crate::grpc::gateway::{GetGuildInfo};

#[get("/guilds/{guild_id}/preview")]
pub async fn get_guild_preview(cs: Data<CoreStruct>, guild_id: Path<String>) -> HttpResponse {
    let col: Collection<Guild> = cs.db.collection("guilds");
    let guild_id = guild_id.to_owned();

    match col.find_one(
        doc! {"_id": &guild_id}, None)
        .await
    {
        Ok(Some(mut guild)) => {
            let guild_info = cs.gc.to_owned().get_guild_info(tonic::Request::new(GetGuildInfo {
                guild_id
            })).await.unwrap();

            let guild_info = guild_info.into_inner();
            guild.available = Some(guild_info.available);
            guild.approximate_presence_count = Some(guild_info.presences);
            HttpResponse::Ok().json(guild)
        },
        Ok(None) => HttpResponse::NotFound().body("Not Found".to_string()),
        Err(_) => HttpResponse::InternalServerError().body("".to_string())
    }
}
