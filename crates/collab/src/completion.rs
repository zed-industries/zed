use anyhow::Result;
use rpc::proto;

pub fn get_supermaven_api_key(
    _request: proto::GetSupermavenApiKey,
) -> Result<supermaven_api::GetExternalUserRequest> {
    #[allow(unreachable_code)]
    Ok(supermaven_api::GetExternalUserRequest {
        user_id: todo!("Get the user id from collab server."),
    })
}
