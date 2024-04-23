use anyhow::{anyhow, Result};
use rpc::proto;

pub fn get_supermaven_api_key(
    request: proto::GetSupermavenApiKey,
) -> Result<supermaven::GetApiKeyRequest> {
    Ok(supermaven::GetApiKeyRequest {
        user_id: request.user_id,
    })
}
