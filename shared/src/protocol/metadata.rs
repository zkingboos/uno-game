use jsonrpc_core::Metadata;

#[derive(Clone)]
pub struct Meta {
    pub token: String,
}

impl Meta {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
        }
    }
}

impl Metadata for Meta {

}