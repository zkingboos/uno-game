use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

use crate::payload::handshake::{HandshakeResponse, HandshakeStatus};

#[rpc]
pub trait Rpc {
    type Metadata;

    #[rpc(name = "protocolVersion")]
    fn protocol_version(&self) -> Result<String>;

    #[rpc(name = "handshake")]
    fn handshake(&self, status: HandshakeStatus) -> Result<HandshakeResponse>;
}
