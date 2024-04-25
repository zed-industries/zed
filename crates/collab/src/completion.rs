use anyhow::Result;
use rpc::proto;

pub fn get_supermaven_api_key(
    _request: proto::GetSupermavenApiKey,
) -> Result<supermaven_api::GetApiKeyRequest> {
    #[allow(unreachable_code)]
    Ok(supermaven_api::GetApiKeyRequest {
        user_id: todo!("Get the user id from collab server."),
    })
}
