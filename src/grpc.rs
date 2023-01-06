use gateway::guild_client::GuildClient;
use gateway::user_client::UserClient;
use tonic::transport::Channel;

pub mod gateway {
    tonic::include_proto!("derailed.grpc");
}

pub async fn connect() -> (GuildClient<Channel>, UserClient<Channel>) {
    // TODO: remove unwrap and actually handle possible errors
    let guild_client = GuildClient::connect("http://localhost:50051").await.unwrap();
    let user_client = UserClient::connect("http://localhost:50052").await.unwrap();

    (guild_client, user_client)
}
