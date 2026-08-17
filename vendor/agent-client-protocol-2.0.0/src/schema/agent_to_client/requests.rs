#[cfg(feature = "unstable_elicitation")]
use crate::schema::v1::{CreateElicitationRequest, CreateElicitationResponse};
use crate::schema::v1::{
    CreateTerminalRequest, CreateTerminalResponse, KillTerminalRequest, KillTerminalResponse,
    ReadTextFileRequest, ReadTextFileResponse, ReleaseTerminalRequest, ReleaseTerminalResponse,
    RequestPermissionRequest, RequestPermissionResponse, TerminalOutputRequest,
    TerminalOutputResponse, WaitForTerminalExitRequest, WaitForTerminalExitResponse,
    WriteTextFileRequest, WriteTextFileResponse,
};

impl_jsonrpc_request!(
    RequestPermissionRequest,
    RequestPermissionResponse,
    "session/request_permission"
);
impl_jsonrpc_request!(
    WriteTextFileRequest,
    WriteTextFileResponse,
    "fs/write_text_file"
);
impl_jsonrpc_request!(
    ReadTextFileRequest,
    ReadTextFileResponse,
    "fs/read_text_file"
);
impl_jsonrpc_request!(
    CreateTerminalRequest,
    CreateTerminalResponse,
    "terminal/create"
);
impl_jsonrpc_request!(
    TerminalOutputRequest,
    TerminalOutputResponse,
    "terminal/output"
);
impl_jsonrpc_request!(
    ReleaseTerminalRequest,
    ReleaseTerminalResponse,
    "terminal/release"
);
impl_jsonrpc_request!(
    WaitForTerminalExitRequest,
    WaitForTerminalExitResponse,
    "terminal/wait_for_exit"
);
impl_jsonrpc_request!(KillTerminalRequest, KillTerminalResponse, "terminal/kill");
#[cfg(feature = "unstable_elicitation")]
impl_jsonrpc_request!(
    CreateElicitationRequest,
    CreateElicitationResponse,
    "elicitation/create"
);
