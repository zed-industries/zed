use anyhow::{anyhow, Result};
use rpc::proto;

pub fn get_supermaven_api_key(
    _request: proto::GetSupermavenApiKey,
) -> Result<supermaven_api::GetExternalUserRequest> {
    Err(anyhow!("User is not authorized to access Supermaven API."))
    // Ok(supermaven_api::GetExternalUserRequest {
    //     user_id: todo!("Get the user id from collab server."),
    // })
}
