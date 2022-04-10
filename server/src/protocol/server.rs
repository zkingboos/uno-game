use std::sync::Arc;

use jsonrpc_core::Result;
use uno_game_shared::{
    payload::handshake::{HandshakeResponse, HandshakeStatus},
    protocol::{rpc::Rpc, metadata::Meta},
};

use crate::service::user_service::UserService;

pub struct GameRpc {
    pub user_service: UserService,
}

impl GameRpc {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }
}

impl Rpc for GameRpc {
    type Metadata = Arc<Meta>;

    fn protocol_version(&self) -> Result<String> {
        Ok("0.1.0".to_owned())
    }

    fn handshake(
        &self,
        status: HandshakeStatus,
    ) -> Result<HandshakeResponse> {
        Ok(HandshakeResponse::new(
            format!("OK {}", status.username).as_str(),
        ))
    }
}
