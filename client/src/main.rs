use jsonrpc_core_client::{transports::http::connect, RpcError};
use uno_game_shared::{payload::handshake::HandshakeStatus, protocol::rpc::gen_client};

#[tokio::main]
async fn main() -> Result<(), RpcError> {
    // let handshake = HandshakeStatus::new("localhost", 3000, "king");
    // let client = connect::<gen_client::Client>("http://127.0.0.1:3000").await?;
    // let response = client.handshake(handshake).await?;
    // dbg!(response);
    Ok(())
}
