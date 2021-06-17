use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetRpcAddressResponse {
    pub address: String,
}

pub const GET_RPC_ADDRESS_PATH: &'static str = "/api/rpc-address";
