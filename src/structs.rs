use serde::{Serialize, Deserialize};
use mongodb::{bson::oid::ObjectId, Database};
use time::OffsetDateTime;
use tonic::transport::Channel;

use crate::grpc::gateway::{guild_client::GuildClient, user_client::UserClient};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub email: String,
    pub password: String,
    pub flags: i32,
    pub system: bool,
    pub deletor_job_id: Option<String>,
    pub suspended: bool
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Settings {
    #[serde(rename = "_id")]
    pub user_id: String,
    pub status: String,
    pub guild_order: Vec<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Permissions {
    pub allow: String,
    pub deny: String
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Guild {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub flags: i32,
    pub owner_id: String,
    pub permissions: Permissions,
    pub approximate_presence_count: Option<i32>,
    pub available: Option<bool>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Member {
    #[serde(skip_serializing, rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub guild_id: String,
    pub nick: Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Invite {
    #[serde(rename = "_id")]
    pub id: String,
    pub guild: Guild,
    pub author: User
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: i32,
    pub created_at: String,
    pub content: Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    Online,
    Dnd,
    Invisible,
    Idle
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Presence {
    #[serde(skip_serializing, rename = "_id")]
    pub id: String,
    pub device: String,
    pub activities: Vec<Activity>,
    pub status: Status
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DBChannel {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: i32,
    pub name: Option<String>,
    pub last_message_id: Option<String>,
    pub parent_id: Option<String>,
    pub guild_id: Option<String>,
    pub message_deletor_job_id: Option<String>,
    pub members: Option<Vec<User>>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,
    pub author: Option<Member>,
    pub content: String,
    pub channel_id: String,
    pub timestamp: OffsetDateTime,
    pub edited_timestamp: OffsetDateTime
}

#[derive(Clone, Debug)]
pub struct CoreStruct {
    pub db: Database,
    pub gc: GuildClient<Channel>,
    pub uc: UserClient<Channel>
}
