use std::sync::Arc;

use jsonrpc_core::MetaIoHandler;
use jsonrpc_http_server::{
    hyper::{Body, Request},
    ServerBuilder,
};
use uno_game_server::{protocol::server::GameRpc, service::user_service::UserService};
use uno_game_shared::protocol::{metadata::Meta, rpc::Rpc};

fn main() {
    let user_service = UserService::new();

    let mut io = MetaIoHandler::default();
    let game_rpc = GameRpc::new(user_service);
    io.extend_with(Rpc::to_delegate(game_rpc));

    let server = ServerBuilder::with_meta_extractor(io, |request: &Request<Body>| {
        let token = request
            .headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap();

        Arc::new(Meta::new(token))
    })
    .threads(3)
    .start_http(&"127.0.0.1:3000".parse().unwrap())
    .expect("Unable to start game server");

    println!("Server started: {:?}", server.address());
    server.wait();
}
