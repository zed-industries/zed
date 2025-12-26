#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerId {
    #[prost(uint32, tag="1")]
    pub owner_id: u32,
    #[prost(uint32, tag="2")]
    pub id: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub github_login: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub avatar_url: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nonce {
    #[prost(uint64, tag="1")]
    pub upper_half: u64,
    #[prost(uint64, tag="2")]
    pub lower_half: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collaborator {
    #[prost(message, optional, tag="1")]
    pub peer_id: ::core::option::Option<PeerId>,
    #[prost(uint32, tag="2")]
    pub replica_id: u32,
    #[prost(uint64, tag="3")]
    pub user_id: u64,
    #[prost(bool, tag="4")]
    pub is_host: bool,
    #[prost(string, optional, tag="5")]
    pub committer_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub committer_email: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(uint64, tag="1")]
    pub seconds: u64,
    #[prost(uint32, tag="2")]
    pub nanos: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(uint64, tag="1")]
    pub worktree_id: u64,
    #[prost(uint64, optional, tag="2")]
    pub entry_id: ::core::option::Option<u64>,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub mtime: ::core::option::Option<Timestamp>,
    #[prost(bool, tag="5")]
    pub is_deleted: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(bool, tag="2")]
    pub is_dir: bool,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub inode: u64,
    #[prost(message, optional, tag="5")]
    pub mtime: ::core::option::Option<Timestamp>,
    #[prost(bool, tag="7")]
    pub is_ignored: bool,
    #[prost(bool, tag="8")]
    pub is_external: bool,
    #[prost(bool, tag="10")]
    pub is_fifo: bool,
    #[prost(uint64, optional, tag="11")]
    pub size: ::core::option::Option<u64>,
    #[prost(string, optional, tag="12")]
    pub canonical_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag="13")]
    pub is_hidden: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddWorktree {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub project_id: u64,
    #[prost(bool, tag="3")]
    pub visible: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddWorktreeResponse {
    #[prost(uint64, tag="1")]
    pub worktree_id: u64,
    #[prost(string, tag="2")]
    pub canonicalized_path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWorktree {
    #[prost(uint64, tag="1")]
    pub worktree_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPathMetadata {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPathMetadataResponse {
    #[prost(bool, tag="1")]
    pub exists: bool,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_dir: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorktreeMetadata {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub root_name: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub visible: bool,
    #[prost(string, tag="4")]
    pub abs_path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Hash, Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectPath {
    #[prost(uint64, tag="1")]
    pub worktree_id: u64,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemoteDirectoryConfig {
    #[prost(bool, tag="1")]
    pub is_dir: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemoteDirectory {
    #[prost(uint64, tag="1")]
    pub dev_server_id: u64,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub config: ::core::option::Option<ListRemoteDirectoryConfig>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryInfo {
    #[prost(bool, tag="1")]
    pub is_dir: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemoteDirectoryResponse {
    #[prost(string, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub entry_info: ::prost::alloc::vec::Vec<EntryInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectEntry {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub is_directory: bool,
    #[prost(bytes="vec", optional, tag="5")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameProjectEntry {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub entry_id: u64,
    #[prost(string, tag="3")]
    pub new_path: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub new_worktree_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyProjectEntry {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub entry_id: u64,
    #[prost(string, tag="3")]
    pub new_path: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub new_worktree_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectEntry {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub entry_id: u64,
    #[prost(bool, tag="3")]
    pub use_trash: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandProjectEntry {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub entry_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandProjectEntryResponse {
    #[prost(uint64, tag="1")]
    pub worktree_scan_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandAllForProjectEntry {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub entry_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandAllForProjectEntryResponse {
    #[prost(uint64, tag="1")]
    pub worktree_scan_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectEntryResponse {
    #[prost(message, optional, tag="1")]
    pub entry: ::core::option::Option<Entry>,
    #[prost(uint64, tag="2")]
    pub worktree_scan_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorktreeSettings {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="LocalSettingsKind", optional, tag="5")]
    pub kind: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserSettings {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub contents: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustWorktrees {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, repeated, tag="2")]
    pub trusted_paths: ::prost::alloc::vec::Vec<PathTrust>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathTrust {
    #[prost(oneof="path_trust::Content", tags="2, 3")]
    pub content: ::core::option::Option<path_trust::Content>,
}
/// Nested message and enum types in `PathTrust`.
pub mod path_trust {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(uint64, tag="2")]
        WorktreeId(u64),
        #[prost(string, tag="3")]
        AbsPath(::prost::alloc::string::String),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestrictWorktrees {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, repeated, tag="3")]
    pub worktree_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocalSettingsKind {
    Settings = 0,
    Tasks = 1,
    Editorconfig = 2,
    Debug = 3,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenNewBuffer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenBufferResponse {
    #[prost(uint64, tag="1")]
    pub buffer_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBufferForPeer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub peer_id: ::core::option::Option<PeerId>,
    #[prost(oneof="create_buffer_for_peer::Variant", tags="3, 4")]
    pub variant: ::core::option::Option<create_buffer_for_peer::Variant>,
}
/// Nested message and enum types in `CreateBufferForPeer`.
pub mod create_buffer_for_peer {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="3")]
        State(super::BufferState),
        #[prost(message, tag="4")]
        Chunk(super::BufferChunk),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBuffer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenBufferByPath {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenBufferById {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBufferFile {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub file: ::core::option::Option<File>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveBuffer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(message, optional, tag="4")]
    pub new_path: ::core::option::Option<ProjectPath>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseBuffer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferSaved {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(message, optional, tag="4")]
    pub mtime: ::core::option::Option<Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferReloaded {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(message, optional, tag="4")]
    pub mtime: ::core::option::Option<Timestamp>,
    #[prost(enumeration="LineEnding", tag="6")]
    pub line_ending: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadBuffers {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, repeated, tag="2")]
    pub buffer_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadBuffersResponse {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<ProjectTransaction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynchronizeBuffers {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, repeated, tag="2")]
    pub buffers: ::prost::alloc::vec::Vec<BufferVersion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynchronizeBuffersResponse {
    #[prost(message, repeated, tag="1")]
    pub buffers: ::prost::alloc::vec::Vec<BufferVersion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferVersion {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, repeated, tag="2")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferState {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, optional, tag="2")]
    pub file: ::core::option::Option<File>,
    #[prost(string, tag="3")]
    pub base_text: ::prost::alloc::string::String,
    #[prost(enumeration="LineEnding", tag="5")]
    pub line_ending: i32,
    #[prost(message, repeated, tag="6")]
    pub saved_version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(message, optional, tag="8")]
    pub saved_mtime: ::core::option::Option<Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferChunk {
    #[prost(uint64, tag="1")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="2")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
    #[prost(bool, tag="3")]
    pub is_last: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VectorClockEntry {
    #[prost(uint32, tag="1")]
    pub replica_id: u32,
    #[prost(uint32, tag="2")]
    pub timestamp: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoMapEntry {
    #[prost(uint32, tag="1")]
    pub replica_id: u32,
    #[prost(uint32, tag="2")]
    pub local_timestamp: u32,
    #[prost(message, repeated, tag="3")]
    pub counts: ::prost::alloc::vec::Vec<UndoCount>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoCount {
    #[prost(uint32, tag="1")]
    pub replica_id: u32,
    #[prost(uint32, tag="2")]
    pub lamport_timestamp: u32,
    #[prost(uint32, tag="3")]
    pub count: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    #[prost(oneof="operation::Variant", tags="1, 2, 3, 4, 5, 6")]
    pub variant: ::core::option::Option<operation::Variant>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Edit {
        #[prost(uint32, tag="1")]
        pub replica_id: u32,
        #[prost(uint32, tag="2")]
        pub lamport_timestamp: u32,
        #[prost(message, repeated, tag="3")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
        #[prost(message, repeated, tag="4")]
        pub ranges: ::prost::alloc::vec::Vec<super::Range>,
        #[prost(string, repeated, tag="5")]
        pub new_text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Undo {
        #[prost(uint32, tag="1")]
        pub replica_id: u32,
        #[prost(uint32, tag="2")]
        pub lamport_timestamp: u32,
        #[prost(message, repeated, tag="3")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
        #[prost(message, repeated, tag="4")]
        pub counts: ::prost::alloc::vec::Vec<super::UndoCount>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateSelections {
        #[prost(uint32, tag="1")]
        pub replica_id: u32,
        #[prost(uint32, tag="2")]
        pub lamport_timestamp: u32,
        #[prost(message, repeated, tag="3")]
        pub selections: ::prost::alloc::vec::Vec<super::Selection>,
        #[prost(bool, tag="4")]
        pub line_mode: bool,
        #[prost(enumeration="super::CursorShape", tag="5")]
        pub cursor_shape: i32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateCompletionTriggers {
        #[prost(uint32, tag="1")]
        pub replica_id: u32,
        #[prost(uint32, tag="2")]
        pub lamport_timestamp: u32,
        #[prost(string, repeated, tag="3")]
        pub triggers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(uint64, tag="4")]
        pub language_server_id: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateLineEnding {
        #[prost(uint32, tag="1")]
        pub replica_id: u32,
        #[prost(uint32, tag="2")]
        pub lamport_timestamp: u32,
        #[prost(enumeration="super::LineEnding", tag="3")]
        pub line_ending: i32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="1")]
        Edit(Edit),
        #[prost(message, tag="2")]
        Undo(Undo),
        #[prost(message, tag="3")]
        UpdateSelections(UpdateSelections),
        #[prost(message, tag="4")]
        UpdateDiagnostics(super::UpdateDiagnostics),
        #[prost(message, tag="5")]
        UpdateCompletionTriggers(UpdateCompletionTriggers),
        #[prost(message, tag="6")]
        UpdateLineEnding(UpdateLineEnding),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectTransaction {
    #[prost(uint64, repeated, tag="1")]
    pub buffer_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="2")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<LamportTimestamp>,
    #[prost(message, repeated, tag="2")]
    pub edit_ids: ::prost::alloc::vec::Vec<LamportTimestamp>,
    #[prost(message, repeated, tag="3")]
    pub start: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LamportTimestamp {
    #[prost(uint32, tag="1")]
    pub replica_id: u32,
    #[prost(uint32, tag="2")]
    pub value: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    #[prost(uint64, tag="1")]
    pub start: u64,
    #[prost(uint64, tag="2")]
    pub end: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Selection {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<EditorAnchor>,
    #[prost(message, optional, tag="3")]
    pub end: ::core::option::Option<EditorAnchor>,
    #[prost(bool, tag="4")]
    pub reversed: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditorAnchor {
    #[prost(uint64, tag="1")]
    pub excerpt_id: u64,
    #[prost(message, optional, tag="2")]
    pub anchor: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDiagnostics {
    #[prost(uint32, tag="1")]
    pub replica_id: u32,
    #[prost(uint32, tag="2")]
    pub lamport_timestamp: u32,
    #[prost(uint64, tag="3")]
    pub server_id: u64,
    #[prost(message, repeated, tag="4")]
    pub diagnostics: ::prost::alloc::vec::Vec<Diagnostic>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Hash, Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Anchor {
    #[prost(uint32, tag="1")]
    pub replica_id: u32,
    #[prost(uint32, tag="2")]
    pub timestamp: u32,
    #[prost(uint64, tag="3")]
    pub offset: u64,
    #[prost(enumeration="Bias", tag="4")]
    pub bias: i32,
    #[prost(uint64, optional, tag="5")]
    pub buffer_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnchorRange {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(uint64, tag="1")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="3")]
    pub end: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Diagnostic {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<Anchor>,
    #[prost(string, optional, tag="3")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub registration_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="diagnostic::SourceKind", tag="16")]
    pub source_kind: i32,
    #[prost(enumeration="diagnostic::Severity", tag="4")]
    pub severity: i32,
    #[prost(string, tag="5")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, optional, tag="6")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="7")]
    pub group_id: u64,
    #[prost(bool, tag="8")]
    pub is_primary: bool,
    #[prost(bool, tag="10")]
    pub is_disk_based: bool,
    #[prost(bool, tag="11")]
    pub is_unnecessary: bool,
    #[prost(bool, tag="15")]
    pub underline: bool,
    #[prost(string, optional, tag="12")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub code_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub markdown: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Diagnostic`.
pub mod diagnostic {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SourceKind {
        Pulled = 0,
        Pushed = 1,
        Other = 2,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        None = 0,
        Error = 1,
        Warning = 2,
        Information = 3,
        Hint = 4,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub regex: bool,
    #[prost(bool, tag="4")]
    pub whole_word: bool,
    #[prost(bool, tag="5")]
    pub case_sensitive: bool,
    #[prost(string, repeated, tag="10")]
    pub files_to_include: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="11")]
    pub files_to_exclude: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="9")]
    pub match_full_paths: bool,
    #[prost(bool, tag="8")]
    pub include_ignored: bool,
    #[prost(string, tag="6")]
    pub files_to_include_legacy: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub files_to_exclude_legacy: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindSearchCandidates {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub query: ::core::option::Option<SearchQuery>,
    #[prost(uint64, tag="3")]
    pub limit: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindSearchCandidatesResponse {
    #[prost(uint64, repeated, tag="1")]
    pub buffer_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LineEnding {
    Unix = 0,
    Windows = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CursorShape {
    CursorBar = 0,
    CursorBlock = 1,
    CursorUnderscore = 2,
    CursorHollow = 3,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Bias {
    Left = 0,
    Right = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskContextForLocation {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub location: ::core::option::Option<Location>,
    #[prost(map="string, string", tag="3")]
    pub task_variables: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskContext {
    #[prost(string, optional, tag="1")]
    pub cwd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(map="string, string", tag="2")]
    pub task_variables: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(map="string, string", tag="3")]
    pub project_env: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shell {
    #[prost(oneof="shell::ShellType", tags="1, 2, 3")]
    pub shell_type: ::core::option::Option<shell::ShellType>,
}
/// Nested message and enum types in `Shell`.
pub mod shell {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WithArguments {
        #[prost(string, tag="1")]
        pub program: ::prost::alloc::string::String,
        #[prost(string, repeated, tag="2")]
        pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ShellType {
        #[prost(message, tag="1")]
        System(super::System),
        #[prost(string, tag="2")]
        Program(::prost::alloc::string::String),
        #[prost(message, tag="3")]
        WithArguments(WithArguments),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct System {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpawnInTerminal {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, string", tag="4")]
    pub env: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub cwd: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDirectoryEnvironment {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub shell: ::core::option::Option<Shell>,
    #[prost(string, tag="3")]
    pub directory: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectoryEnvironment {
    #[prost(map="string, string", tag="1")]
    pub environment: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RevealStrategy {
    RevealAlways = 0,
    RevealNever = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HideStrategy {
    HideAlways = 0,
    HideNever = 1,
    HideOnSuccess = 2,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<ContextOperation>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextMetadata {
    #[prost(string, tag="1")]
    pub context_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextMessageStatus {
    #[prost(oneof="context_message_status::Variant", tags="1, 2, 3, 4")]
    pub variant: ::core::option::Option<context_message_status::Variant>,
}
/// Nested message and enum types in `ContextMessageStatus`.
pub mod context_message_status {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Done {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pending {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        #[prost(string, tag="1")]
        pub message: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Canceled {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="1")]
        Done(Done),
        #[prost(message, tag="2")]
        Pending(Pending),
        #[prost(message, tag="3")]
        Error(Error),
        #[prost(message, tag="4")]
        Canceled(Canceled),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextMessage {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<LamportTimestamp>,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(enumeration="LanguageModelRole", tag="3")]
    pub role: i32,
    #[prost(message, optional, tag="4")]
    pub status: ::core::option::Option<ContextMessageStatus>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlashCommandOutputSection {
    #[prost(message, optional, tag="1")]
    pub range: ::core::option::Option<AnchorRange>,
    #[prost(string, tag="2")]
    pub icon_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub metadata: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThoughtProcessOutputSection {
    #[prost(message, optional, tag="1")]
    pub range: ::core::option::Option<AnchorRange>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextOperation {
    #[prost(oneof="context_operation::Variant", tags="1, 2, 3, 5, 6, 7, 8, 9")]
    pub variant: ::core::option::Option<context_operation::Variant>,
}
/// Nested message and enum types in `ContextOperation`.
pub mod context_operation {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InsertMessage {
        #[prost(message, optional, tag="1")]
        pub message: ::core::option::Option<super::ContextMessage>,
        #[prost(message, repeated, tag="2")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateMessage {
        #[prost(message, optional, tag="1")]
        pub message_id: ::core::option::Option<super::LamportTimestamp>,
        #[prost(enumeration="super::LanguageModelRole", tag="2")]
        pub role: i32,
        #[prost(message, optional, tag="3")]
        pub status: ::core::option::Option<super::ContextMessageStatus>,
        #[prost(message, optional, tag="4")]
        pub timestamp: ::core::option::Option<super::LamportTimestamp>,
        #[prost(message, repeated, tag="5")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateSummary {
        #[prost(string, tag="1")]
        pub summary: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub done: bool,
        #[prost(message, optional, tag="3")]
        pub timestamp: ::core::option::Option<super::LamportTimestamp>,
        #[prost(message, repeated, tag="4")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SlashCommandStarted {
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::LamportTimestamp>,
        #[prost(message, optional, tag="2")]
        pub output_range: ::core::option::Option<super::AnchorRange>,
        #[prost(string, tag="3")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="4")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SlashCommandOutputSectionAdded {
        #[prost(message, optional, tag="1")]
        pub timestamp: ::core::option::Option<super::LamportTimestamp>,
        #[prost(message, optional, tag="2")]
        pub section: ::core::option::Option<super::SlashCommandOutputSection>,
        #[prost(message, repeated, tag="3")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SlashCommandCompleted {
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::LamportTimestamp>,
        #[prost(message, optional, tag="3")]
        pub timestamp: ::core::option::Option<super::LamportTimestamp>,
        #[prost(string, optional, tag="4")]
        pub error_message: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag="5")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThoughtProcessOutputSectionAdded {
        #[prost(message, optional, tag="1")]
        pub timestamp: ::core::option::Option<super::LamportTimestamp>,
        #[prost(message, optional, tag="2")]
        pub section: ::core::option::Option<super::ThoughtProcessOutputSection>,
        #[prost(message, repeated, tag="3")]
        pub version: ::prost::alloc::vec::Vec<super::VectorClockEntry>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BufferOperation {
        #[prost(message, optional, tag="1")]
        pub operation: ::core::option::Option<super::Operation>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="1")]
        InsertMessage(InsertMessage),
        #[prost(message, tag="2")]
        UpdateMessage(UpdateMessage),
        #[prost(message, tag="3")]
        UpdateSummary(UpdateSummary),
        #[prost(message, tag="5")]
        BufferOperation(BufferOperation),
        #[prost(message, tag="6")]
        SlashCommandStarted(SlashCommandStarted),
        #[prost(message, tag="7")]
        SlashCommandOutputSectionAdded(SlashCommandOutputSectionAdded),
        #[prost(message, tag="8")]
        SlashCommandCompleted(SlashCommandCompleted),
        #[prost(message, tag="9")]
        ThoughtProcessOutputSectionAdded(ThoughtProcessOutputSectionAdded),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertiseContexts {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, repeated, tag="2")]
    pub contexts: ::prost::alloc::vec::Vec<ContextMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenContext {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub context_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenContextResponse {
    #[prost(message, optional, tag="1")]
    pub context: ::core::option::Option<Context>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContext {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContextResponse {
    #[prost(string, tag="1")]
    pub context_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub context: ::core::option::Option<Context>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContext {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub context_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub operation: ::core::option::Option<ContextOperation>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextVersion {
    #[prost(string, tag="1")]
    pub context_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub context_version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(message, repeated, tag="3")]
    pub buffer_version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynchronizeContexts {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, repeated, tag="2")]
    pub contexts: ::prost::alloc::vec::Vec<ContextVersion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynchronizeContextsResponse {
    #[prost(message, repeated, tag="1")]
    pub contexts: ::prost::alloc::vec::Vec<ContextVersion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgentServerCommand {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub root_dir: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentServerCommand {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, string", tag="3")]
    pub env: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub root_dir: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub login: ::core::option::Option<SpawnInTerminal>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAgentsUpdated {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, repeated, tag="2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalExtensionAgentTarget {
    #[prost(string, tag="1")]
    pub archive: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub sha256: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(map="string, string", tag="5")]
    pub env: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalExtensionAgent {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub icon_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub extension_id: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="4")]
    pub targets: ::std::collections::HashMap<::prost::alloc::string::String, ExternalExtensionAgentTarget>,
    #[prost(map="string, string", tag="5")]
    pub env: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalExtensionAgentsUpdated {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, repeated, tag="2")]
    pub agents: ::prost::alloc::vec::Vec<ExternalExtensionAgent>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAgentLoadingStatusUpdated {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub status: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewExternalAgentVersionAvailable {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LanguageModelRole {
    LanguageModelUser = 0,
    LanguageModelAssistant = 1,
    LanguageModelSystem = 2,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInviteInfo {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub count: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownRemoteServer {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Toast {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub notification_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HideToast {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub notification_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenServerSettings {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCrashFiles {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCrashFilesResponse {
    #[prost(message, repeated, tag="1")]
    pub crashes: ::prost::alloc::vec::Vec<CrashReport>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrashReport {
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub minidump_contents: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extension {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub dev: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncExtensions {
    #[prost(message, repeated, tag="1")]
    pub extensions: ::prost::alloc::vec::Vec<Extension>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncExtensionsResponse {
    #[prost(string, tag="1")]
    pub tmp_dir: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub missing_extensions: ::prost::alloc::vec::Vec<Extension>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallExtension {
    #[prost(message, optional, tag="1")]
    pub extension: ::core::option::Option<Extension>,
    #[prost(string, tag="2")]
    pub tmp_dir: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AskPassRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(uint64, tag="4")]
    pub askpass_id: u64,
    #[prost(string, tag="5")]
    pub prompt: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AskPassResponse {
    #[prost(string, tag="1")]
    pub response: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupermavenApiKey {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupermavenApiKeyResponse {
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefinition {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefinitionResponse {
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<LocationLink>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeclaration {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeclarationResponse {
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<LocationLink>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTypeDefinition {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTypeDefinitionResponse {
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<LocationLink>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImplementation {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImplementationResponse {
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<LocationLink>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferences {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferencesResponse {
    #[prost(message, repeated, tag="1")]
    pub locations: ::prost::alloc::vec::Vec<Location>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentHighlights {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentHighlightsResponse {
    #[prost(message, repeated, tag="1")]
    pub highlights: ::prost::alloc::vec::Vec<DocumentHighlight>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationLink {
    #[prost(message, optional, tag="1")]
    pub origin: ::core::option::Option<Location>,
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<Location>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentHighlight {
    #[prost(enumeration="document_highlight::Kind", tag="1")]
    pub kind: i32,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="3")]
    pub end: ::core::option::Option<Anchor>,
}
/// Nested message and enum types in `DocumentHighlight`.
pub mod document_highlight {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        Text = 0,
        Read = 1,
        Write = 2,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectSymbols {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectSymbolsResponse {
    #[prost(message, repeated, tag="4")]
    pub symbols: ::prost::alloc::vec::Vec<Symbol>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symbol {
    #[prost(uint64, tag="1")]
    pub source_worktree_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(string, tag="3")]
    pub language_server_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="5")]
    pub kind: i32,
    #[prost(string, tag="6")]
    pub path: ::prost::alloc::string::String,
    /// Cannot use generate anchors for unopened files,
    /// so we are forced to use point coords instead
    #[prost(message, optional, tag="7")]
    pub start: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="8")]
    pub end: ::core::option::Option<PointUtf16>,
    #[prost(bytes="vec", tag="9")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="10")]
    pub language_server_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentSymbols {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentSymbolsResponse {
    #[prost(message, repeated, tag="1")]
    pub symbols: ::prost::alloc::vec::Vec<DocumentSymbol>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentSymbol {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub kind: i32,
    /// Cannot use generate anchors for unopened files,
    /// so we are forced to use point coords instead
    #[prost(message, optional, tag="3")]
    pub start: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="4")]
    pub end: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="5")]
    pub selection_start: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="6")]
    pub selection_end: ::core::option::Option<PointUtf16>,
    #[prost(message, repeated, tag="7")]
    pub children: ::prost::alloc::vec::Vec<DocumentSymbol>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlayHints {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="4")]
    pub end: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="5")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlayHintsResponse {
    #[prost(message, repeated, tag="1")]
    pub hints: ::prost::alloc::vec::Vec<InlayHint>,
    #[prost(message, repeated, tag="2")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointUtf16 {
    #[prost(uint32, tag="1")]
    pub row: u32,
    #[prost(uint32, tag="2")]
    pub column: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtExpandMacro {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtExpandMacroResponse {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub expansion: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtOpenDocs {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtOpenDocsResponse {
    #[prost(string, optional, tag="1")]
    pub web: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub local: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtSwitchSourceHeader {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtSwitchSourceHeaderResponse {
    #[prost(string, tag="1")]
    pub target_file: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtGoToParentModule {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtGoToParentModuleResponse {
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<LocationLink>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompletionsResponse {
    #[prost(message, repeated, tag="1")]
    pub completions: ::prost::alloc::vec::Vec<Completion>,
    #[prost(message, repeated, tag="2")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    /// `!is_complete`, inverted for a default of `is_complete = true`
    #[prost(bool, tag="3")]
    pub can_reuse: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCompletionAdditionalEdits {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub completion: ::core::option::Option<Completion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCompletionAdditionalEditsResponse {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Completion {
    #[prost(message, optional, tag="1")]
    pub old_replace_start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="2")]
    pub old_replace_end: ::core::option::Option<Anchor>,
    #[prost(string, tag="3")]
    pub new_text: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub server_id: u64,
    #[prost(bytes="vec", tag="5")]
    pub lsp_completion: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub resolved: bool,
    #[prost(enumeration="completion::Source", tag="7")]
    pub source: i32,
    #[prost(bytes="vec", optional, tag="8")]
    pub lsp_defaults: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="9")]
    pub buffer_word_start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="10")]
    pub buffer_word_end: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="11")]
    pub old_insert_start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="12")]
    pub old_insert_end: ::core::option::Option<Anchor>,
    #[prost(string, optional, tag="13")]
    pub sort_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Completion`.
pub mod completion {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Source {
        Lsp = 0,
        Custom = 1,
        BufferWord = 2,
        Dap = 3,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCodeActions {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="4")]
    pub end: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="5")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCodeActionsResponse {
    #[prost(message, repeated, tag="1")]
    pub actions: ::prost::alloc::vec::Vec<CodeAction>,
    #[prost(message, repeated, tag="2")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignatureHelp {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignatureHelpResponse {
    #[prost(message, optional, tag="1")]
    pub signature_help: ::core::option::Option<SignatureHelp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureHelp {
    #[prost(message, repeated, tag="1")]
    pub signatures: ::prost::alloc::vec::Vec<SignatureInformation>,
    #[prost(uint32, optional, tag="2")]
    pub active_signature: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub active_parameter: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureInformation {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub documentation: ::core::option::Option<Documentation>,
    #[prost(message, repeated, tag="3")]
    pub parameters: ::prost::alloc::vec::Vec<ParameterInformation>,
    #[prost(uint32, optional, tag="4")]
    pub active_parameter: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Documentation {
    #[prost(oneof="documentation::Content", tags="1, 2")]
    pub content: ::core::option::Option<documentation::Content>,
}
/// Nested message and enum types in `Documentation`.
pub mod documentation {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(string, tag="1")]
        Value(::prost::alloc::string::String),
        #[prost(message, tag="2")]
        MarkupContent(super::MarkupContent),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterInformation {
    #[prost(message, optional, tag="3")]
    pub documentation: ::core::option::Option<Documentation>,
    #[prost(oneof="parameter_information::Label", tags="1, 2")]
    pub label: ::core::option::Option<parameter_information::Label>,
}
/// Nested message and enum types in `ParameterInformation`.
pub mod parameter_information {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Label {
        #[prost(string, tag="1")]
        Simple(::prost::alloc::string::String),
        #[prost(message, tag="2")]
        LabelOffsets(super::LabelOffsets),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelOffsets {
    #[prost(uint32, tag="1")]
    pub start: u32,
    #[prost(uint32, tag="2")]
    pub end: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHover {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="5")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHoverResponse {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="3")]
    pub contents: ::prost::alloc::vec::Vec<HoverBlock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HoverBlock {
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub language: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag="3")]
    pub is_markdown: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCodeAction {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub action: ::core::option::Option<CodeAction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCodeActionResponse {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<ProjectTransaction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareRename {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareRenameResponse {
    #[prost(bool, tag="1")]
    pub can_rename: bool,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="3")]
    pub end: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(bool, tag="5")]
    pub only_unprepared_rename_supported: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerformRename {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(string, tag="4")]
    pub new_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnTypeFormatting {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(string, tag="4")]
    pub trigger: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnTypeFormattingResponse {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedEditingRange {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedEditingRangeResponse {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AnchorRange>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlayHint {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="2")]
    pub label: ::core::option::Option<InlayHintLabel>,
    #[prost(string, optional, tag="3")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag="4")]
    pub padding_left: bool,
    #[prost(bool, tag="5")]
    pub padding_right: bool,
    #[prost(message, optional, tag="6")]
    pub tooltip: ::core::option::Option<InlayHintTooltip>,
    #[prost(message, optional, tag="7")]
    pub resolve_state: ::core::option::Option<ResolveState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlayHintLabel {
    #[prost(oneof="inlay_hint_label::Label", tags="1, 2")]
    pub label: ::core::option::Option<inlay_hint_label::Label>,
}
/// Nested message and enum types in `InlayHintLabel`.
pub mod inlay_hint_label {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Label {
        #[prost(string, tag="1")]
        Value(::prost::alloc::string::String),
        #[prost(message, tag="2")]
        LabelParts(super::InlayHintLabelParts),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlayHintLabelParts {
    #[prost(message, repeated, tag="1")]
    pub parts: ::prost::alloc::vec::Vec<InlayHintLabelPart>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlayHintLabelPart {
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub tooltip: ::core::option::Option<InlayHintLabelPartTooltip>,
    #[prost(string, optional, tag="3")]
    pub location_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub location_range_start: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="5")]
    pub location_range_end: ::core::option::Option<PointUtf16>,
    #[prost(uint64, optional, tag="6")]
    pub language_server_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlayHintTooltip {
    #[prost(oneof="inlay_hint_tooltip::Content", tags="1, 2")]
    pub content: ::core::option::Option<inlay_hint_tooltip::Content>,
}
/// Nested message and enum types in `InlayHintTooltip`.
pub mod inlay_hint_tooltip {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(string, tag="1")]
        Value(::prost::alloc::string::String),
        #[prost(message, tag="2")]
        MarkupContent(super::MarkupContent),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlayHintLabelPartTooltip {
    #[prost(oneof="inlay_hint_label_part_tooltip::Content", tags="1, 2")]
    pub content: ::core::option::Option<inlay_hint_label_part_tooltip::Content>,
}
/// Nested message and enum types in `InlayHintLabelPartTooltip`.
pub mod inlay_hint_label_part_tooltip {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(string, tag="1")]
        Value(::prost::alloc::string::String),
        #[prost(message, tag="2")]
        MarkupContent(super::MarkupContent),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveState {
    #[prost(enumeration="resolve_state::State", tag="1")]
    pub state: i32,
    #[prost(message, optional, tag="2")]
    pub lsp_resolve_state: ::core::option::Option<resolve_state::LspResolveState>,
}
/// Nested message and enum types in `ResolveState`.
pub mod resolve_state {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LspResolveState {
        #[prost(string, optional, tag="1")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint64, tag="2")]
        pub server_id: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Resolved = 0,
        CanResolve = 1,
        Resolving = 2,
    }
}
/// This type is used to resolve more than just
/// the documentation, but for backwards-compatibility
/// reasons we can't rename the type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveCompletionDocumentation {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub language_server_id: u64,
    #[prost(bytes="vec", tag="3")]
    pub lsp_completion: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub buffer_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveCompletionDocumentationResponse {
    #[prost(string, tag="1")]
    pub documentation: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub documentation_is_markdown: bool,
    #[prost(message, optional, tag="3")]
    pub old_replace_start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="4")]
    pub old_replace_end: ::core::option::Option<Anchor>,
    #[prost(string, tag="5")]
    pub new_text: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub lsp_completion: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub old_insert_start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="8")]
    pub old_insert_end: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveInlayHint {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(uint64, tag="3")]
    pub language_server_id: u64,
    #[prost(message, optional, tag="4")]
    pub hint: ::core::option::Option<InlayHint>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveInlayHintResponse {
    #[prost(message, optional, tag="1")]
    pub hint: ::core::option::Option<InlayHint>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshInlayHints {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub server_id: u64,
    #[prost(uint64, optional, tag="3")]
    pub request_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeLens {
    #[prost(bytes="vec", tag="1")]
    pub lsp_lens: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCodeLens {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCodeLensResponse {
    #[prost(message, repeated, tag="1")]
    pub lens_actions: ::prost::alloc::vec::Vec<CodeAction>,
    #[prost(message, repeated, tag="2")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshCodeLens {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkupContent {
    #[prost(bool, tag="1")]
    pub is_markdown: bool,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerformRenameResponse {
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<ProjectTransaction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeAction {
    #[prost(uint64, tag="1")]
    pub server_id: u64,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="3")]
    pub end: ::core::option::Option<Anchor>,
    #[prost(bytes="vec", tag="4")]
    pub lsp_action: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="code_action::Kind", tag="5")]
    pub kind: i32,
    #[prost(bool, tag="6")]
    pub resolved: bool,
}
/// Nested message and enum types in `CodeAction`.
pub mod code_action {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        Action = 0,
        Command = 1,
        CodeLens = 2,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageServer {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="3")]
    pub worktree_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartLanguageServer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub server: ::core::option::Option<LanguageServer>,
    #[prost(string, tag="3")]
    pub capabilities: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDiagnosticSummary {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(message, optional, tag="3")]
    pub summary: ::core::option::Option<DiagnosticSummary>,
    #[prost(message, repeated, tag="4")]
    pub more_summaries: ::prost::alloc::vec::Vec<DiagnosticSummary>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnosticSummary {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub language_server_id: u64,
    #[prost(uint32, tag="3")]
    pub error_count: u32,
    #[prost(uint32, tag="4")]
    pub warning_count: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLanguageServer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub language_server_id: u64,
    #[prost(string, optional, tag="8")]
    pub server_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="update_language_server::Variant", tags="3, 4, 5, 6, 7, 9, 10, 11")]
    pub variant: ::core::option::Option<update_language_server::Variant>,
}
/// Nested message and enum types in `UpdateLanguageServer`.
pub mod update_language_server {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="3")]
        WorkStart(super::LspWorkStart),
        #[prost(message, tag="4")]
        WorkProgress(super::LspWorkProgress),
        #[prost(message, tag="5")]
        WorkEnd(super::LspWorkEnd),
        #[prost(message, tag="6")]
        DiskBasedDiagnosticsUpdating(super::LspDiskBasedDiagnosticsUpdating),
        #[prost(message, tag="7")]
        DiskBasedDiagnosticsUpdated(super::LspDiskBasedDiagnosticsUpdated),
        #[prost(message, tag="9")]
        StatusUpdate(super::StatusUpdate),
        #[prost(message, tag="10")]
        RegisteredForBuffer(super::RegisteredForBuffer),
        #[prost(message, tag="11")]
        MetadataUpdated(super::ServerMetadataUpdated),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressToken {
    #[prost(oneof="progress_token::Value", tags="1, 2")]
    pub value: ::core::option::Option<progress_token::Value>,
}
/// Nested message and enum types in `ProgressToken`.
pub mod progress_token {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(int32, tag="1")]
        Number(i32),
        #[prost(string, tag="2")]
        String(::prost::alloc::string::String),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspWorkStart {
    #[prost(string, optional, tag="4")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub percentage: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="5")]
    pub is_cancellable: ::core::option::Option<bool>,
    #[prost(message, optional, tag="6")]
    pub token: ::core::option::Option<ProgressToken>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspWorkProgress {
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub percentage: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="4")]
    pub is_cancellable: ::core::option::Option<bool>,
    #[prost(message, optional, tag="5")]
    pub token: ::core::option::Option<ProgressToken>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspWorkEnd {
    #[prost(message, optional, tag="2")]
    pub token: ::core::option::Option<ProgressToken>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspDiskBasedDiagnosticsUpdating {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspDiskBasedDiagnosticsUpdated {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUpdate {
    #[prost(string, optional, tag="1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="status_update::Status", tags="2, 3")]
    pub status: ::core::option::Option<status_update::Status>,
}
/// Nested message and enum types in `StatusUpdate`.
pub mod status_update {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(enumeration="super::ServerBinaryStatus", tag="2")]
        Binary(i32),
        #[prost(enumeration="super::ServerHealth", tag="3")]
        Health(i32),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredForBuffer {
    #[prost(string, tag="1")]
    pub buffer_abs_path: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageServerBinaryInfo {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMetadataUpdated {
    #[prost(string, optional, tag="1")]
    pub capabilities: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub binary: ::core::option::Option<LanguageServerBinaryInfo>,
    #[prost(string, optional, tag="3")]
    pub configuration: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub workspace_folders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageServerLog {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub language_server_id: u64,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
    #[prost(oneof="language_server_log::LogType", tags="4, 5, 6")]
    pub log_type: ::core::option::Option<language_server_log::LogType>,
}
/// Nested message and enum types in `LanguageServerLog`.
pub mod language_server_log {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LogType {
        #[prost(message, tag="4")]
        Log(super::LogMessage),
        #[prost(message, tag="5")]
        Trace(super::TraceMessage),
        #[prost(message, tag="6")]
        Rpc(super::RpcMessage),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogMessage {
    #[prost(enumeration="log_message::LogLevel", tag="1")]
    pub level: i32,
}
/// Nested message and enum types in `LogMessage`.
pub mod log_message {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LogLevel {
        Log = 0,
        Info = 1,
        Warning = 2,
        Error = 3,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceMessage {
    #[prost(string, optional, tag="1")]
    pub verbose_info: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcMessage {
    #[prost(enumeration="rpc_message::Kind", tag="1")]
    pub kind: i32,
}
/// Nested message and enum types in `RpcMessage`.
pub mod rpc_message {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        Received = 0,
        Sent = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspLogTrace {
    #[prost(string, optional, tag="1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCodeActionKind {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub kind: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="3")]
    pub buffer_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCodeActionKindResponse {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<ProjectTransaction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterBufferWithLanguageServers {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub only_servers: ::prost::alloc::vec::Vec<LanguageServerSelector>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenBufferForSymbol {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub symbol: ::core::option::Option<Symbol>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenBufferForSymbolResponse {
    #[prost(uint64, tag="1")]
    pub buffer_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormatBuffers {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(enumeration="FormatTrigger", tag="2")]
    pub trigger: i32,
    #[prost(uint64, repeated, tag="3")]
    pub buffer_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormatBuffersResponse {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<ProjectTransaction>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompletions {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(message, repeated, tag="4")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(uint64, optional, tag="5")]
    pub server_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelLanguageServerWork {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(oneof="cancel_language_server_work::Work", tags="2, 3")]
    pub work: ::core::option::Option<cancel_language_server_work::Work>,
}
/// Nested message and enum types in `CancelLanguageServerWork`.
pub mod cancel_language_server_work {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Buffers {
        #[prost(uint64, repeated, tag="2")]
        pub buffer_ids: ::prost::alloc::vec::Vec<u64>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LanguageServerWork {
        #[prost(uint64, tag="1")]
        pub language_server_id: u64,
        #[prost(message, optional, tag="3")]
        pub token: ::core::option::Option<super::ProgressToken>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Work {
        #[prost(message, tag="2")]
        Buffers(Buffers),
        #[prost(message, tag="3")]
        LanguageServerWork(LanguageServerWork),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageServerPromptRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="5")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="7")]
    pub lsp_name: ::prost::alloc::string::String,
    #[prost(oneof="language_server_prompt_request::Level", tags="2, 3, 4")]
    pub level: ::core::option::Option<language_server_prompt_request::Level>,
}
/// Nested message and enum types in `LanguageServerPromptRequest`.
pub mod language_server_prompt_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Info {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Warning {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Critical {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Level {
        #[prost(message, tag="2")]
        Info(Info),
        #[prost(message, tag="3")]
        Warning(Warning),
        #[prost(message, tag="4")]
        Critical(Critical),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageServerPromptResponse {
    #[prost(uint64, optional, tag="1")]
    pub action_response: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentColor {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentColorResponse {
    #[prost(message, repeated, tag="1")]
    pub colors: ::prost::alloc::vec::Vec<ColorInformation>,
    #[prost(message, repeated, tag="2")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorInformation {
    #[prost(message, optional, tag="1")]
    pub lsp_range_start: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="2")]
    pub lsp_range_end: ::core::option::Option<PointUtf16>,
    #[prost(float, tag="3")]
    pub red: f32,
    #[prost(float, tag="4")]
    pub green: f32,
    #[prost(float, tag="5")]
    pub blue: f32,
    #[prost(float, tag="6")]
    pub alpha: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetColorPresentation {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub color: ::core::option::Option<ColorInformation>,
    #[prost(uint64, tag="4")]
    pub server_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetColorPresentationResponse {
    #[prost(message, repeated, tag="1")]
    pub presentations: ::prost::alloc::vec::Vec<ColorPresentation>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorPresentation {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub text_edit: ::core::option::Option<TextEdit>,
    #[prost(message, repeated, tag="3")]
    pub additional_text_edits: ::prost::alloc::vec::Vec<TextEdit>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextEdit {
    #[prost(string, tag="1")]
    pub new_text: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub lsp_range_start: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="3")]
    pub lsp_range_end: ::core::option::Option<PointUtf16>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspQuery {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub lsp_request_id: u64,
    #[prost(uint64, optional, tag="15")]
    pub server_id: ::core::option::Option<u64>,
    #[prost(oneof="lsp_query::Request", tags="3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14")]
    pub request: ::core::option::Option<lsp_query::Request>,
}
/// Nested message and enum types in `LspQuery`.
pub mod lsp_query {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="3")]
        GetReferences(super::GetReferences),
        #[prost(message, tag="4")]
        GetDocumentColor(super::GetDocumentColor),
        #[prost(message, tag="5")]
        GetHover(super::GetHover),
        #[prost(message, tag="6")]
        GetCodeActions(super::GetCodeActions),
        #[prost(message, tag="7")]
        GetSignatureHelp(super::GetSignatureHelp),
        #[prost(message, tag="8")]
        GetCodeLens(super::GetCodeLens),
        #[prost(message, tag="9")]
        GetDocumentDiagnostics(super::GetDocumentDiagnostics),
        #[prost(message, tag="10")]
        GetDefinition(super::GetDefinition),
        #[prost(message, tag="11")]
        GetDeclaration(super::GetDeclaration),
        #[prost(message, tag="12")]
        GetTypeDefinition(super::GetTypeDefinition),
        #[prost(message, tag="13")]
        GetImplementation(super::GetImplementation),
        #[prost(message, tag="14")]
        InlayHints(super::InlayHints),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspQueryResponse {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub lsp_request_id: u64,
    #[prost(message, repeated, tag="3")]
    pub responses: ::prost::alloc::vec::Vec<LspResponse>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspResponse {
    #[prost(uint64, tag="7")]
    pub server_id: u64,
    #[prost(oneof="lsp_response::Response", tags="1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13")]
    pub response: ::core::option::Option<lsp_response::Response>,
}
/// Nested message and enum types in `LspResponse`.
pub mod lsp_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        GetHoverResponse(super::GetHoverResponse),
        #[prost(message, tag="2")]
        GetCodeActionsResponse(super::GetCodeActionsResponse),
        #[prost(message, tag="3")]
        GetSignatureHelpResponse(super::GetSignatureHelpResponse),
        #[prost(message, tag="4")]
        GetCodeLensResponse(super::GetCodeLensResponse),
        #[prost(message, tag="5")]
        GetDocumentDiagnosticsResponse(super::GetDocumentDiagnosticsResponse),
        #[prost(message, tag="6")]
        GetDocumentColorResponse(super::GetDocumentColorResponse),
        #[prost(message, tag="8")]
        GetDefinitionResponse(super::GetDefinitionResponse),
        #[prost(message, tag="9")]
        GetDeclarationResponse(super::GetDeclarationResponse),
        #[prost(message, tag="10")]
        GetTypeDefinitionResponse(super::GetTypeDefinitionResponse),
        #[prost(message, tag="11")]
        GetImplementationResponse(super::GetImplementationResponse),
        #[prost(message, tag="12")]
        GetReferencesResponse(super::GetReferencesResponse),
        #[prost(message, tag="13")]
        InlayHintsResponse(super::InlayHintsResponse),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllLanguageServers {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageServerSelector {
    #[prost(oneof="language_server_selector::Selector", tags="1, 2")]
    pub selector: ::core::option::Option<language_server_selector::Selector>,
}
/// Nested message and enum types in `LanguageServerSelector`.
pub mod language_server_selector {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Selector {
        #[prost(uint64, tag="1")]
        ServerId(u64),
        #[prost(string, tag="2")]
        Name(::prost::alloc::string::String),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartLanguageServers {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, repeated, tag="2")]
    pub buffer_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="3")]
    pub only_servers: ::prost::alloc::vec::Vec<LanguageServerSelector>,
    #[prost(bool, tag="4")]
    pub all: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopLanguageServers {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, repeated, tag="2")]
    pub buffer_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="3")]
    pub also_servers: ::prost::alloc::vec::Vec<LanguageServerSelector>,
    #[prost(bool, tag="4")]
    pub all: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtRunnables {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtRunnablesResponse {
    #[prost(message, repeated, tag="1")]
    pub runnables: ::prost::alloc::vec::Vec<LspRunnable>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspRunnable {
    #[prost(bytes="vec", tag="1")]
    pub task_template: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub location: ::core::option::Option<LocationLink>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtCancelFlycheck {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub language_server_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtRunFlycheck {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, optional, tag="2")]
    pub buffer_id: ::core::option::Option<u64>,
    #[prost(uint64, tag="3")]
    pub language_server_id: u64,
    #[prost(bool, tag="4")]
    pub current_file_only: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspExtClearFlycheck {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub language_server_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspDiagnosticRelatedInformation {
    #[prost(string, optional, tag="1")]
    pub location_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub location_range_start: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="3")]
    pub location_range_end: ::core::option::Option<PointUtf16>,
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LspDiagnostic {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<PointUtf16>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<PointUtf16>,
    #[prost(enumeration="lsp_diagnostic::Severity", tag="3")]
    pub severity: i32,
    #[prost(string, optional, tag="4")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub code_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="7")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="8")]
    pub related_information: ::prost::alloc::vec::Vec<LspDiagnosticRelatedInformation>,
    #[prost(enumeration="LspDiagnosticTag", repeated, tag="9")]
    pub tags: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, optional, tag="10")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `LspDiagnostic`.
pub mod lsp_diagnostic {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        None = 0,
        Error = 1,
        Warning = 2,
        Information = 3,
        Hint = 4,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentDiagnostics {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentDiagnosticsResponse {
    #[prost(message, repeated, tag="1")]
    pub pulled_diagnostics: ::prost::alloc::vec::Vec<PulledDiagnostics>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PulledDiagnostics {
    #[prost(uint64, tag="1")]
    pub server_id: u64,
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub result_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag="4")]
    pub changed: bool,
    #[prost(message, repeated, tag="5")]
    pub diagnostics: ::prost::alloc::vec::Vec<LspDiagnostic>,
    #[prost(string, optional, tag="6")]
    pub registration_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullWorkspaceDiagnostics {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub server_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleLspLogs {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(enumeration="toggle_lsp_logs::LogType", tag="2")]
    pub log_type: i32,
    #[prost(uint64, tag="3")]
    pub server_id: u64,
    #[prost(bool, tag="4")]
    pub enabled: bool,
}
/// Nested message and enum types in `ToggleLspLogs`.
pub mod toggle_lsp_logs {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LogType {
        Log = 0,
        Trace = 1,
        Rpc = 2,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarkupKind {
    PlainText = 0,
    Markdown = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerHealth {
    Ok = 0,
    Warning = 1,
    Error = 2,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerBinaryStatus {
    None = 0,
    CheckingForUpdate = 1,
    Downloading = 2,
    Starting = 3,
    Stopping = 4,
    Stopped = 5,
    Failed = 6,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FormatTrigger {
    Save = 0,
    Manual = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LspDiagnosticTag {
    None = 0,
    Unnecessary = 1,
    Deprecated = 2,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="ChannelVisibility", tag="3")]
    pub visibility: i32,
    #[prost(int32, tag="4")]
    pub channel_order: i32,
    #[prost(uint64, repeated, tag="5")]
    pub parent_path: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannels {
    #[prost(message, repeated, tag="1")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
    #[prost(uint64, repeated, tag="4")]
    pub delete_channels: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="5")]
    pub channel_invitations: ::prost::alloc::vec::Vec<Channel>,
    #[prost(uint64, repeated, tag="6")]
    pub remove_channel_invitations: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="7")]
    pub channel_participants: ::prost::alloc::vec::Vec<ChannelParticipants>,
    #[prost(message, repeated, tag="9")]
    pub latest_channel_buffer_versions: ::prost::alloc::vec::Vec<ChannelBufferVersion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserChannels {
    #[prost(message, repeated, tag="2")]
    pub observed_channel_buffer_version: ::prost::alloc::vec::Vec<ChannelBufferVersion>,
    #[prost(message, repeated, tag="3")]
    pub channel_memberships: ::prost::alloc::vec::Vec<ChannelMembership>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMembership {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(enumeration="ChannelRole", tag="2")]
    pub role: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageId {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub message_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPermission {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(enumeration="ChannelRole", tag="3")]
    pub role: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelParticipants {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, repeated, tag="2")]
    pub participant_user_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinChannel {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannel {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelMembers {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub limit: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelMembersResponse {
    #[prost(message, repeated, tag="1")]
    pub members: ::prost::alloc::vec::Vec<ChannelMember>,
    #[prost(message, repeated, tag="2")]
    pub users: ::prost::alloc::vec::Vec<User>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMember {
    #[prost(uint64, tag="1")]
    pub user_id: u64,
    #[prost(enumeration="channel_member::Kind", tag="3")]
    pub kind: i32,
    #[prost(enumeration="ChannelRole", tag="4")]
    pub role: i32,
}
/// Nested message and enum types in `ChannelMember`.
pub mod channel_member {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        Member = 0,
        Invitee = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeToChannels {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannel {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="2")]
    pub parent_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelResponse {
    #[prost(message, optional, tag="1")]
    pub channel: ::core::option::Option<Channel>,
    #[prost(uint64, optional, tag="2")]
    pub parent_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InviteChannelMember {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub user_id: u64,
    #[prost(enumeration="ChannelRole", tag="4")]
    pub role: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveChannelMember {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub user_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetChannelMemberRole {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub user_id: u64,
    #[prost(enumeration="ChannelRole", tag="3")]
    pub role: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetChannelVisibility {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(enumeration="ChannelVisibility", tag="2")]
    pub visibility: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameChannel {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameChannelResponse {
    #[prost(message, optional, tag="1")]
    pub channel: ::core::option::Option<Channel>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinChannelChat {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinChannelChatResponse {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<ChannelMessage>,
    #[prost(bool, tag="2")]
    pub done: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveChannelChat {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendChannelMessage {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(string, tag="2")]
    pub body: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub nonce: ::core::option::Option<Nonce>,
    #[prost(message, repeated, tag="4")]
    pub mentions: ::prost::alloc::vec::Vec<ChatMention>,
    #[prost(uint64, optional, tag="5")]
    pub reply_to_message_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveChannelMessage {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub message_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannelMessage {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub message_id: u64,
    #[prost(message, optional, tag="4")]
    pub nonce: ::core::option::Option<Nonce>,
    #[prost(string, tag="5")]
    pub body: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub mentions: ::prost::alloc::vec::Vec<ChatMention>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckChannelMessage {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub message_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendChannelMessageResponse {
    #[prost(message, optional, tag="1")]
    pub message: ::core::option::Option<ChannelMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageSent {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(message, optional, tag="2")]
    pub message: ::core::option::Option<ChannelMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessageUpdate {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(message, optional, tag="2")]
    pub message: ::core::option::Option<ChannelMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelMessages {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub before_message_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelMessagesResponse {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<ChannelMessage>,
    #[prost(bool, tag="2")]
    pub done: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelMessagesById {
    #[prost(uint64, repeated, tag="1")]
    pub message_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveChannel {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(uint64, tag="2")]
    pub to: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReorderChannel {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(enumeration="reorder_channel::Direction", tag="2")]
    pub direction: i32,
}
/// Nested message and enum types in `ReorderChannel`.
pub mod reorder_channel {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        Up = 0,
        Down = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinChannelBuffer {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBufferVersion {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(message, repeated, tag="2")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(uint64, tag="3")]
    pub epoch: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannelBufferCollaborators {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(message, repeated, tag="2")]
    pub collaborators: ::prost::alloc::vec::Vec<Collaborator>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannelBuffer {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(message, repeated, tag="2")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessage {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub body: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub timestamp: u64,
    #[prost(uint64, tag="4")]
    pub sender_id: u64,
    #[prost(message, optional, tag="5")]
    pub nonce: ::core::option::Option<Nonce>,
    #[prost(message, repeated, tag="6")]
    pub mentions: ::prost::alloc::vec::Vec<ChatMention>,
    #[prost(uint64, optional, tag="7")]
    pub reply_to_message_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="8")]
    pub edited_at: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMention {
    #[prost(message, optional, tag="1")]
    pub range: ::core::option::Option<Range>,
    #[prost(uint64, tag="2")]
    pub user_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinChannelBuffers {
    #[prost(message, repeated, tag="1")]
    pub buffers: ::prost::alloc::vec::Vec<ChannelBufferVersion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinChannelBuffersResponse {
    #[prost(message, repeated, tag="1")]
    pub buffers: ::prost::alloc::vec::Vec<RejoinedChannelBuffer>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckBufferOperation {
    #[prost(uint64, tag="1")]
    pub buffer_id: u64,
    #[prost(uint64, tag="2")]
    pub epoch: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinChannelBufferResponse {
    #[prost(uint64, tag="1")]
    pub buffer_id: u64,
    #[prost(uint32, tag="2")]
    pub replica_id: u32,
    #[prost(string, tag="3")]
    pub base_text: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
    #[prost(message, repeated, tag="5")]
    pub collaborators: ::prost::alloc::vec::Vec<Collaborator>,
    #[prost(uint64, tag="6")]
    pub epoch: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinedChannelBuffer {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(message, repeated, tag="2")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
    #[prost(message, repeated, tag="3")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
    #[prost(message, repeated, tag="4")]
    pub collaborators: ::prost::alloc::vec::Vec<Collaborator>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveChannelBuffer {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespondToChannelInvite {
    #[prost(uint64, tag="1")]
    pub channel_id: u64,
    #[prost(bool, tag="2")]
    pub accept: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelVisibility {
    Public = 0,
    Members = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChannelRole {
    Admin = 0,
    Member = 1,
    Guest = 2,
    Banned = 3,
    Talker = 4,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitBranchesResponse {
    #[prost(message, repeated, tag="1")]
    pub branches: ::prost::alloc::vec::Vec<Branch>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDiffBases {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(string, optional, tag="3")]
    pub staged_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub committed_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="update_diff_bases::Mode", tag="5")]
    pub mode: i32,
}
/// Nested message and enum types in `UpdateDiffBases`.
pub mod update_diff_bases {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// No collaborator is using the unstaged diff.
        HeadOnly = 0,
        /// No collaborator is using the diff from HEAD.
        IndexOnly = 1,
        /// Both the unstaged and uncommitted diffs are demanded,
        /// and the contents of the index and HEAD are the same for this path.
        IndexMatchesHead = 2,
        /// Both the unstaged and uncommitted diffs are demanded,
        /// and the contents of the index and HEAD differ for this path,
        /// where None means the path doesn't exist in that state of the repo.
        IndexAndHead = 3,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenUnstagedDiff {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenUnstagedDiffResponse {
    #[prost(string, optional, tag="1")]
    pub staged_text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenUncommittedDiff {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenUncommittedDiffResponse {
    #[prost(string, optional, tag="1")]
    pub staged_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub committed_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="open_uncommitted_diff_response::Mode", tag="3")]
    pub mode: i32,
}
/// Nested message and enum types in `OpenUncommittedDiffResponse`.
pub mod open_uncommitted_diff_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        IndexMatchesHead = 0,
        IndexAndHead = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIndexText {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermalinkToLine {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub selection: ::core::option::Option<Range>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermalinkToLineResponse {
    #[prost(string, tag="1")]
    pub permalink: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Branch {
    #[prost(bool, tag="1")]
    pub is_head: bool,
    #[prost(string, tag="2")]
    pub ref_name: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="3")]
    pub unix_timestamp: ::core::option::Option<u64>,
    #[prost(message, optional, tag="4")]
    pub upstream: ::core::option::Option<GitUpstream>,
    #[prost(message, optional, tag="5")]
    pub most_recent_commit: ::core::option::Option<CommitSummary>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitUpstream {
    #[prost(string, tag="1")]
    pub ref_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub tracking: ::core::option::Option<UpstreamTracking>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamTracking {
    #[prost(uint64, tag="1")]
    pub ahead: u64,
    #[prost(uint64, tag="2")]
    pub behind: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitSummary {
    #[prost(string, tag="1")]
    pub sha: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subject: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub commit_timestamp: i64,
    #[prost(string, tag="4")]
    pub author_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitBranches {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub repository: ::core::option::Option<ProjectPath>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGitBranch {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub branch_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub repository: ::core::option::Option<ProjectPath>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRepository {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub id: u64,
    #[prost(string, tag="3")]
    pub abs_path: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="4")]
    pub entry_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag="5")]
    pub branch_summary: ::core::option::Option<Branch>,
    #[prost(message, repeated, tag="6")]
    pub updated_statuses: ::prost::alloc::vec::Vec<StatusEntry>,
    #[prost(string, repeated, tag="7")]
    pub removed_statuses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="8")]
    pub current_merge_conflicts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag="9")]
    pub scan_id: u64,
    #[prost(bool, tag="10")]
    pub is_last_update: bool,
    #[prost(message, optional, tag="11")]
    pub head_commit_details: ::core::option::Option<GitCommitDetails>,
    #[prost(string, optional, tag="12")]
    pub merge_message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="13")]
    pub stash_entries: ::prost::alloc::vec::Vec<StashEntry>,
    #[prost(string, optional, tag="14")]
    pub remote_upstream_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub remote_origin_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRepository {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitFileStatus {
    #[prost(oneof="git_file_status::Variant", tags="1, 2, 3, 4")]
    pub variant: ::core::option::Option<git_file_status::Variant>,
}
/// Nested message and enum types in `GitFileStatus`.
pub mod git_file_status {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Untracked {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ignored {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Unmerged {
        #[prost(enumeration="super::GitStatus", tag="1")]
        pub first_head: i32,
        #[prost(enumeration="super::GitStatus", tag="2")]
        pub second_head: i32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tracked {
        #[prost(enumeration="super::GitStatus", tag="1")]
        pub index_status: i32,
        #[prost(enumeration="super::GitStatus", tag="2")]
        pub worktree_status: i32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="1")]
        Untracked(Untracked),
        #[prost(message, tag="2")]
        Ignored(Ignored),
        #[prost(message, tag="3")]
        Unmerged(Unmerged),
        #[prost(message, tag="4")]
        Tracked(Tracked),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitGetBranches {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitCreateBranch {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub branch_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitChangeBranch {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub branch_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitRenameBranch {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(string, tag="3")]
    pub branch: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitCreateRemote {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(string, tag="3")]
    pub remote_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub remote_url: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitRemoveRemote {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(string, tag="3")]
    pub remote_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitDeleteBranch {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(string, tag="3")]
    pub branch_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitDiff {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(enumeration="git_diff::DiffType", tag="4")]
    pub diff_type: i32,
}
/// Nested message and enum types in `GitDiff`.
pub mod git_diff {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiffType {
        HeadToWorktree = 0,
        HeadToIndex = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitDiffResponse {
    #[prost(string, tag="1")]
    pub diff: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitInit {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub abs_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub fallback_branch_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitClone {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub abs_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub remote_repo: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitCloneResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckForPushedCommits {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckForPushedCommitsResponse {
    #[prost(string, repeated, tag="1")]
    pub pushed_to: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitShow {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub commit: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitCommitDetails {
    #[prost(string, tag="1")]
    pub sha: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub commit_timestamp: i64,
    #[prost(string, tag="4")]
    pub author_email: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub author_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadCommitDiff {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub commit: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadCommitDiffResponse {
    #[prost(message, repeated, tag="1")]
    pub files: ::prost::alloc::vec::Vec<CommitFile>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitFile {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub old_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub new_text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitReset {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub commit: ::prost::alloc::string::String,
    #[prost(enumeration="git_reset::ResetMode", tag="5")]
    pub mode: i32,
}
/// Nested message and enum types in `GitReset`.
pub mod git_reset {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResetMode {
        Soft = 0,
        Mixed = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitCheckoutFiles {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub commit: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitFileHistory {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub skip: u64,
    #[prost(uint64, optional, tag="6")]
    pub limit: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitFileHistoryResponse {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<FileHistoryEntry>,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHistoryEntry {
    #[prost(string, tag="1")]
    pub sha: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub commit_timestamp: i64,
    #[prost(string, tag="5")]
    pub author_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub author_email: ::prost::alloc::string::String,
}
/// Move to `git.proto` once collab's min version is >=0.171.0.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusEntry {
    #[prost(string, tag="1")]
    pub repo_path: ::prost::alloc::string::String,
    /// Can be removed once collab's min version is >=0.171.0.
    #[prost(enumeration="GitStatus", tag="2")]
    pub simple_status: i32,
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<GitFileStatus>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StashEntry {
    #[prost(bytes="vec", tag="1")]
    pub oid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub branch: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="4")]
    pub index: u64,
    #[prost(int64, tag="5")]
    pub timestamp: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stage {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, repeated, tag="4")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unstage {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, repeated, tag="4")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stash {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(string, repeated, tag="3")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StashPop {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(uint64, optional, tag="3")]
    pub stash_index: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StashApply {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(uint64, optional, tag="3")]
    pub stash_index: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StashDrop {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(uint64, optional, tag="3")]
    pub stash_index: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, optional, tag="4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub options: ::core::option::Option<commit::CommitOptions>,
    #[prost(uint64, tag="9")]
    pub askpass_id: u64,
}
/// Nested message and enum types in `Commit`.
pub mod commit {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommitOptions {
        #[prost(bool, tag="1")]
        pub amend: bool,
        #[prost(bool, tag="2")]
        pub signoff: bool,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenCommitMessageBuffer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Push {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub remote_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub branch_name: ::prost::alloc::string::String,
    #[prost(enumeration="push::PushOptions", optional, tag="6")]
    pub options: ::core::option::Option<i32>,
    #[prost(uint64, tag="7")]
    pub askpass_id: u64,
}
/// Nested message and enum types in `Push`.
pub mod push {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PushOptions {
        SetUpstream = 0,
        Force = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fetch {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(uint64, tag="4")]
    pub askpass_id: u64,
    #[prost(string, optional, tag="5")]
    pub remote: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemotes {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, optional, tag="4")]
    pub branch_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag="5")]
    pub is_push: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemotesResponse {
    #[prost(message, repeated, tag="1")]
    pub remotes: ::prost::alloc::vec::Vec<get_remotes_response::Remote>,
}
/// Nested message and enum types in `GetRemotesResponse`.
pub mod get_remotes_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Remote {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pull {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub repository_id: u64,
    #[prost(string, tag="4")]
    pub remote_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub branch_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="6")]
    pub askpass_id: u64,
    #[prost(bool, tag="7")]
    pub rebase: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteMessageResponse {
    #[prost(string, tag="1")]
    pub stdout: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub stderr: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlameBuffer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, repeated, tag="3")]
    pub version: ::prost::alloc::vec::Vec<VectorClockEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlameEntry {
    #[prost(bytes="vec", tag="1")]
    pub sha: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub start_line: u32,
    #[prost(uint32, tag="3")]
    pub end_line: u32,
    #[prost(uint32, tag="4")]
    pub original_line_number: u32,
    #[prost(string, optional, tag="5")]
    pub author: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub author_mail: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="7")]
    pub author_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag="8")]
    pub author_tz: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub committer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub committer_mail: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="11")]
    pub committer_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag="12")]
    pub committer_tz: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub previous: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="15")]
    pub filename: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitMessage {
    #[prost(bytes="vec", tag="1")]
    pub oid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitPermalink {
    #[prost(bytes="vec", tag="1")]
    pub oid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub permalink: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlameBufferResponse {
    #[prost(message, optional, tag="5")]
    pub blame_response: ::core::option::Option<blame_buffer_response::BlameResponse>,
}
/// Nested message and enum types in `BlameBufferResponse`.
pub mod blame_buffer_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlameResponse {
        #[prost(message, repeated, tag="1")]
        pub entries: ::prost::alloc::vec::Vec<super::BlameEntry>,
        #[prost(message, repeated, tag="2")]
        pub messages: ::prost::alloc::vec::Vec<super::CommitMessage>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultBranch {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultBranchResponse {
    #[prost(string, optional, tag="1")]
    pub branch: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTreeDiff {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(bool, tag="3")]
    pub is_merge: bool,
    #[prost(string, tag="4")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub head: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTreeDiffResponse {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<TreeDiffStatus>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreeDiffStatus {
    #[prost(enumeration="tree_diff_status::Status", tag="1")]
    pub status: i32,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub oid: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `TreeDiffStatus`.
pub mod tree_diff_status {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Added = 0,
        Modified = 1,
        Deleted = 2,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlobContent {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(string, tag="3")]
    pub oid: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlobContentResponse {
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitGetWorktrees {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitWorktreesResponse {
    #[prost(message, repeated, tag="1")]
    pub worktrees: ::prost::alloc::vec::Vec<Worktree>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Worktree {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ref_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sha: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitCreateWorktree {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub directory: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub commit: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunGitHook {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub repository_id: u64,
    #[prost(enumeration="run_git_hook::GitHook", tag="3")]
    pub hook: i32,
}
/// Nested message and enum types in `RunGitHook`.
pub mod run_git_hook {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GitHook {
        PreCommit = 0,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GitStatus {
    Added = 0,
    Modified = 1,
    Conflict = 2,
    Deleted = 3,
    Updated = 4,
    TypeChanged = 5,
    Renamed = 6,
    Copied = 7,
    Unmodified = 8,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoom {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomResponse {
    #[prost(message, optional, tag="1")]
    pub room: ::core::option::Option<Room>,
    #[prost(message, optional, tag="2")]
    pub live_kit_connection_info: ::core::option::Option<LiveKitConnectionInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinRoom {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinRoomResponse {
    #[prost(message, optional, tag="1")]
    pub room: ::core::option::Option<Room>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub live_kit_connection_info: ::core::option::Option<LiveKitConnectionInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinRoom {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, repeated, tag="2")]
    pub reshared_projects: ::prost::alloc::vec::Vec<UpdateProject>,
    #[prost(message, repeated, tag="3")]
    pub rejoined_projects: ::prost::alloc::vec::Vec<RejoinProject>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinRemoteProjects {
    #[prost(message, repeated, tag="1")]
    pub rejoined_projects: ::prost::alloc::vec::Vec<RejoinProject>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinRemoteProjectsResponse {
    #[prost(message, repeated, tag="1")]
    pub rejoined_projects: ::prost::alloc::vec::Vec<RejoinedProject>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinProject {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, repeated, tag="2")]
    pub worktrees: ::prost::alloc::vec::Vec<RejoinWorktree>,
    #[prost(message, repeated, tag="3")]
    pub repositories: ::prost::alloc::vec::Vec<RejoinRepository>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinWorktree {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(uint64, tag="2")]
    pub scan_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinRepository {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(uint64, tag="2")]
    pub scan_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinRoomResponse {
    #[prost(message, optional, tag="1")]
    pub room: ::core::option::Option<Room>,
    #[prost(message, repeated, tag="2")]
    pub reshared_projects: ::prost::alloc::vec::Vec<ResharedProject>,
    #[prost(message, repeated, tag="3")]
    pub rejoined_projects: ::prost::alloc::vec::Vec<RejoinedProject>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResharedProject {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, repeated, tag="2")]
    pub collaborators: ::prost::alloc::vec::Vec<Collaborator>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinedProject {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, repeated, tag="2")]
    pub worktrees: ::prost::alloc::vec::Vec<WorktreeMetadata>,
    #[prost(message, repeated, tag="3")]
    pub collaborators: ::prost::alloc::vec::Vec<Collaborator>,
    #[prost(message, repeated, tag="4")]
    pub language_servers: ::prost::alloc::vec::Vec<LanguageServer>,
    #[prost(string, repeated, tag="5")]
    pub language_server_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveRoom {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, repeated, tag="2")]
    pub participants: ::prost::alloc::vec::Vec<Participant>,
    #[prost(message, repeated, tag="3")]
    pub pending_participants: ::prost::alloc::vec::Vec<PendingParticipant>,
    #[prost(message, repeated, tag="4")]
    pub followers: ::prost::alloc::vec::Vec<Follower>,
    #[prost(string, tag="5")]
    pub livekit_room: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Participant {
    #[prost(uint64, tag="1")]
    pub user_id: u64,
    #[prost(message, optional, tag="2")]
    pub peer_id: ::core::option::Option<PeerId>,
    #[prost(message, repeated, tag="3")]
    pub projects: ::prost::alloc::vec::Vec<ParticipantProject>,
    #[prost(message, optional, tag="4")]
    pub location: ::core::option::Option<ParticipantLocation>,
    #[prost(uint32, tag="5")]
    pub participant_index: u32,
    #[prost(enumeration="ChannelRole", tag="6")]
    pub role: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingParticipant {
    #[prost(uint64, tag="1")]
    pub user_id: u64,
    #[prost(uint64, tag="2")]
    pub calling_user_id: u64,
    #[prost(uint64, optional, tag="3")]
    pub initial_project_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantProject {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, repeated, tag="2")]
    pub worktree_root_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follower {
    #[prost(message, optional, tag="1")]
    pub leader_id: ::core::option::Option<PeerId>,
    #[prost(message, optional, tag="2")]
    pub follower_id: ::core::option::Option<PeerId>,
    #[prost(uint64, tag="3")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantLocation {
    #[prost(oneof="participant_location::Variant", tags="1, 2, 3")]
    pub variant: ::core::option::Option<participant_location::Variant>,
}
/// Nested message and enum types in `ParticipantLocation`.
pub mod participant_location {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SharedProject {
        #[prost(uint64, tag="1")]
        pub id: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsharedProject {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct External {
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="1")]
        SharedProject(SharedProject),
        #[prost(message, tag="2")]
        UnsharedProject(UnsharedProject),
        #[prost(message, tag="3")]
        External(External),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Call {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(uint64, tag="2")]
    pub called_user_id: u64,
    #[prost(uint64, optional, tag="3")]
    pub initial_project_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomingCall {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(uint64, tag="2")]
    pub calling_user_id: u64,
    #[prost(uint64, repeated, tag="3")]
    pub participant_user_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag="4")]
    pub initial_project: ::core::option::Option<ParticipantProject>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallCanceled {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelCall {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(uint64, tag="2")]
    pub called_user_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclineCall {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateParticipantLocation {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(message, optional, tag="2")]
    pub location: ::core::option::Option<ParticipantLocation>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomUpdated {
    #[prost(message, optional, tag="1")]
    pub room: ::core::option::Option<Room>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveKitConnectionInfo {
    #[prost(string, tag="1")]
    pub server_url: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub can_publish: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareProject {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(message, repeated, tag="2")]
    pub worktrees: ::prost::alloc::vec::Vec<WorktreeMetadata>,
    #[prost(bool, tag="4")]
    pub is_ssh_project: bool,
    #[prost(bool, optional, tag="5")]
    pub windows_paths: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareProjectResponse {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnshareProject {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProject {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, repeated, tag="2")]
    pub worktrees: ::prost::alloc::vec::Vec<WorktreeMetadata>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinProject {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, optional, tag="2")]
    pub committer_email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub committer_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinProjectResponse {
    #[prost(uint64, tag="5")]
    pub project_id: u64,
    #[prost(uint32, tag="1")]
    pub replica_id: u32,
    #[prost(message, repeated, tag="2")]
    pub worktrees: ::prost::alloc::vec::Vec<WorktreeMetadata>,
    #[prost(message, repeated, tag="3")]
    pub collaborators: ::prost::alloc::vec::Vec<Collaborator>,
    #[prost(message, repeated, tag="4")]
    pub language_servers: ::prost::alloc::vec::Vec<LanguageServer>,
    #[prost(string, repeated, tag="8")]
    pub language_server_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="ChannelRole", tag="6")]
    pub role: i32,
    #[prost(bool, tag="9")]
    pub windows_paths: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaveProject {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorktree {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(string, tag="3")]
    pub root_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub updated_entries: ::prost::alloc::vec::Vec<Entry>,
    #[prost(uint64, repeated, tag="5")]
    pub removed_entries: ::prost::alloc::vec::Vec<u64>,
    /// deprecated
    #[prost(message, repeated, tag="6")]
    pub updated_repositories: ::prost::alloc::vec::Vec<RepositoryEntry>,
    /// deprecated
    #[prost(uint64, repeated, tag="7")]
    pub removed_repositories: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag="8")]
    pub scan_id: u64,
    #[prost(bool, tag="9")]
    pub is_last_update: bool,
    #[prost(string, tag="10")]
    pub abs_path: ::prost::alloc::string::String,
}
/// deprecated
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryEntry {
    #[prost(uint64, tag="1")]
    pub repository_id: u64,
    #[prost(message, repeated, tag="3")]
    pub updated_statuses: ::prost::alloc::vec::Vec<StatusEntry>,
    #[prost(string, repeated, tag="4")]
    pub removed_statuses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub current_merge_conflicts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub branch_summary: ::core::option::Option<Branch>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectCollaborator {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub collaborator: ::core::option::Option<Collaborator>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectCollaborator {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub old_peer_id: ::core::option::Option<PeerId>,
    #[prost(message, optional, tag="3")]
    pub new_peer_id: ::core::option::Option<PeerId>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveProjectCollaborator {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub peer_id: ::core::option::Option<PeerId>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUsers {
    #[prost(uint64, repeated, tag="1")]
    pub user_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuzzySearchUsers {
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsersResponse {
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<User>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestContact {
    #[prost(uint64, tag="1")]
    pub responder_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveContact {
    #[prost(uint64, tag="1")]
    pub user_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespondToContactRequest {
    #[prost(uint64, tag="1")]
    pub requester_id: u64,
    #[prost(enumeration="ContactRequestResponse", tag="2")]
    pub response: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContacts {
    #[prost(message, repeated, tag="1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
    #[prost(uint64, repeated, tag="2")]
    pub remove_contacts: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="3")]
    pub incoming_requests: ::prost::alloc::vec::Vec<IncomingContactRequest>,
    #[prost(uint64, repeated, tag="4")]
    pub remove_incoming_requests: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="5")]
    pub outgoing_requests: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag="6")]
    pub remove_outgoing_requests: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowContacts {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomingContactRequest {
    #[prost(uint64, tag="1")]
    pub requester_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follow {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(uint64, optional, tag="2")]
    pub project_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub leader_id: ::core::option::Option<PeerId>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowResponse {
    #[prost(message, optional, tag="3")]
    pub active_view: ::core::option::Option<View>,
    /// TODO: Remove after version 0.145.x stabilizes.
    #[prost(message, optional, tag="1")]
    pub active_view_id: ::core::option::Option<ViewId>,
    #[prost(message, repeated, tag="2")]
    pub views: ::prost::alloc::vec::Vec<View>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFollowers {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(uint64, optional, tag="2")]
    pub project_id: ::core::option::Option<u64>,
    #[prost(oneof="update_followers::Variant", tags="5, 4, 6")]
    pub variant: ::core::option::Option<update_followers::Variant>,
}
/// Nested message and enum types in `UpdateFollowers`.
pub mod update_followers {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="5")]
        CreateView(super::View),
        /// TODO: Remove after version 0.145.x stabilizes.
        #[prost(message, tag="4")]
        UpdateActiveView(super::UpdateActiveView),
        #[prost(message, tag="6")]
        UpdateView(super::UpdateView),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unfollow {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(uint64, optional, tag="2")]
    pub project_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub leader_id: ::core::option::Option<PeerId>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewId {
    #[prost(message, optional, tag="1")]
    pub creator: ::core::option::Option<PeerId>,
    #[prost(uint64, tag="2")]
    pub id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActiveView {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<ViewId>,
    #[prost(message, optional, tag="2")]
    pub leader_id: ::core::option::Option<PeerId>,
    #[prost(message, optional, tag="3")]
    pub view: ::core::option::Option<View>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateView {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<ViewId>,
    #[prost(message, optional, tag="2")]
    pub leader_id: ::core::option::Option<PeerId>,
    #[prost(oneof="update_view::Variant", tags="3")]
    pub variant: ::core::option::Option<update_view::Variant>,
}
/// Nested message and enum types in `UpdateView`.
pub mod update_view {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Editor {
        #[prost(message, repeated, tag="1")]
        pub inserted_excerpts: ::prost::alloc::vec::Vec<super::ExcerptInsertion>,
        #[prost(uint64, repeated, tag="2")]
        pub deleted_excerpts: ::prost::alloc::vec::Vec<u64>,
        #[prost(message, repeated, tag="3")]
        pub selections: ::prost::alloc::vec::Vec<super::Selection>,
        #[prost(message, optional, tag="4")]
        pub pending_selection: ::core::option::Option<super::Selection>,
        #[prost(message, optional, tag="5")]
        pub scroll_top_anchor: ::core::option::Option<super::EditorAnchor>,
        #[prost(double, tag="8")]
        pub scroll_x: f64,
        #[prost(double, tag="9")]
        pub scroll_y: f64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="3")]
        Editor(Editor),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct View {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<ViewId>,
    #[prost(message, optional, tag="2")]
    pub leader_id: ::core::option::Option<PeerId>,
    #[prost(enumeration="PanelId", optional, tag="6")]
    pub panel_id: ::core::option::Option<i32>,
    #[prost(oneof="view::Variant", tags="3, 4, 5")]
    pub variant: ::core::option::Option<view::Variant>,
}
/// Nested message and enum types in `View`.
pub mod view {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Editor {
        #[prost(bool, tag="1")]
        pub singleton: bool,
        #[prost(string, optional, tag="2")]
        pub title: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag="3")]
        pub excerpts: ::prost::alloc::vec::Vec<super::Excerpt>,
        #[prost(message, repeated, tag="4")]
        pub selections: ::prost::alloc::vec::Vec<super::Selection>,
        #[prost(message, optional, tag="5")]
        pub pending_selection: ::core::option::Option<super::Selection>,
        #[prost(message, optional, tag="6")]
        pub scroll_top_anchor: ::core::option::Option<super::EditorAnchor>,
        #[prost(double, tag="9")]
        pub scroll_x: f64,
        #[prost(double, tag="10")]
        pub scroll_y: f64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChannelView {
        #[prost(uint64, tag="1")]
        pub channel_id: u64,
        #[prost(message, optional, tag="2")]
        pub editor: ::core::option::Option<Editor>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContextEditor {
        #[prost(string, tag="1")]
        pub context_id: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub editor: ::core::option::Option<Editor>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="3")]
        Editor(Editor),
        #[prost(message, tag="4")]
        ChannelView(ChannelView),
        #[prost(message, tag="5")]
        ContextEditor(ContextEditor),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExcerptInsertion {
    #[prost(message, optional, tag="1")]
    pub excerpt: ::core::option::Option<Excerpt>,
    #[prost(uint64, optional, tag="2")]
    pub previous_excerpt_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Excerpt {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(uint64, tag="2")]
    pub buffer_id: u64,
    #[prost(message, optional, tag="3")]
    pub context_start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="4")]
    pub context_end: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="5")]
    pub primary_start: ::core::option::Option<Anchor>,
    #[prost(message, optional, tag="6")]
    pub primary_end: ::core::option::Option<Anchor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(uint64, tag="1")]
    pub user_id: u64,
    #[prost(bool, tag="2")]
    pub online: bool,
    #[prost(bool, tag="3")]
    pub busy: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomParticipantRole {
    #[prost(uint64, tag="1")]
    pub room_id: u64,
    #[prost(uint64, tag="2")]
    pub user_id: u64,
    #[prost(enumeration="ChannelRole", tag="3")]
    pub role: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContactRequestResponse {
    Accept = 0,
    Decline = 1,
    Block = 2,
    Dismiss = 3,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PanelId {
    AssistantPanel = 0,
    DebugPanel = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Breakpoint {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Anchor>,
    #[prost(enumeration="BreakpointState", tag="2")]
    pub state: i32,
    #[prost(string, optional, tag="4")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub condition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub hit_condition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(map="uint64, message", tag="7")]
    pub session_state: ::std::collections::HashMap<u64, BreakpointSessionState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakpointSessionState {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(bool, tag="2")]
    pub verified: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakpointsForFile {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub breakpoints: ::prost::alloc::vec::Vec<Breakpoint>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleBreakpoint {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub breakpoint: ::core::option::Option<Breakpoint>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueFormat {
    #[prost(bool, optional, tag="1")]
    pub hex: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariablesRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(uint64, tag="3")]
    pub variables_reference: u64,
    #[prost(enumeration="VariablesArgumentsFilter", optional, tag="4")]
    pub filter: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag="5")]
    pub start: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub count: ::core::option::Option<u64>,
    #[prost(message, optional, tag="7")]
    pub format: ::core::option::Option<ValueFormat>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapLocationsRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub session_id: u64,
    #[prost(uint64, tag="3")]
    pub location_reference: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapLocationsResponse {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<DapSource>,
    #[prost(uint64, tag="2")]
    pub line: u64,
    #[prost(uint64, optional, tag="3")]
    pub column: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub end_line: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub end_column: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapEvaluateRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(string, tag="3")]
    pub expression: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="4")]
    pub frame_id: ::core::option::Option<u64>,
    #[prost(enumeration="DapEvaluateContext", optional, tag="5")]
    pub context: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapEvaluateResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub evaluate_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="3")]
    pub variable_reference: u64,
    #[prost(uint64, optional, tag="4")]
    pub named_variables: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub indexed_variables: ::core::option::Option<u64>,
    #[prost(string, optional, tag="6")]
    pub memory_reference: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapCompletionRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(string, tag="3")]
    pub query: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="4")]
    pub frame_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub line: ::core::option::Option<u64>,
    #[prost(uint64, tag="6")]
    pub column: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapCompletionItem {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub sort_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub detail: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="DapCompletionItemType", optional, tag="5")]
    pub typ: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag="6")]
    pub start: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="7")]
    pub length: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="8")]
    pub selection_start: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="9")]
    pub selection_length: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapCompletionResponse {
    #[prost(uint64, tag="1")]
    pub client_id: u64,
    #[prost(message, repeated, tag="2")]
    pub completions: ::prost::alloc::vec::Vec<DapCompletionItem>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapScopesRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(uint64, tag="3")]
    pub stack_frame_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapScopesResponse {
    #[prost(message, repeated, tag="1")]
    pub scopes: ::prost::alloc::vec::Vec<DapScope>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapSetVariableValueRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub variables_reference: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapSetVariableValueResponse {
    #[prost(uint64, tag="1")]
    pub client_id: u64,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub variable_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="4")]
    pub variables_reference: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub named_variables: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub indexed_variables: ::core::option::Option<u64>,
    #[prost(string, optional, tag="7")]
    pub memory_reference: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapPauseRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(int64, tag="3")]
    pub thread_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapDisconnectRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(bool, optional, tag="3")]
    pub restart: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub terminate_debuggee: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="5")]
    pub suspend_debuggee: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapTerminateThreadsRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(int64, repeated, tag="3")]
    pub thread_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapThreadsRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapThreadsResponse {
    #[prost(message, repeated, tag="1")]
    pub threads: ::prost::alloc::vec::Vec<DapThread>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapTerminateRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(bool, optional, tag="3")]
    pub restart: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapRestartRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(bytes="vec", tag="3")]
    pub raw_args: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapRestartStackFrameRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(uint64, tag="3")]
    pub stack_frame_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleIgnoreBreakpoints {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint32, tag="2")]
    pub session_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IgnoreBreakpointState {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub session_id: u64,
    #[prost(bool, tag="3")]
    pub ignore: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapNextRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(int64, tag="3")]
    pub thread_id: i64,
    #[prost(bool, optional, tag="4")]
    pub single_thread: ::core::option::Option<bool>,
    #[prost(enumeration="SteppingGranularity", optional, tag="5")]
    pub granularity: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapStepInRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(int64, tag="3")]
    pub thread_id: i64,
    #[prost(uint64, optional, tag="4")]
    pub target_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="5")]
    pub single_thread: ::core::option::Option<bool>,
    #[prost(enumeration="SteppingGranularity", optional, tag="6")]
    pub granularity: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapStepOutRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(int64, tag="3")]
    pub thread_id: i64,
    #[prost(bool, optional, tag="4")]
    pub single_thread: ::core::option::Option<bool>,
    #[prost(enumeration="SteppingGranularity", optional, tag="5")]
    pub granularity: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapStepBackRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(int64, tag="3")]
    pub thread_id: i64,
    #[prost(bool, optional, tag="4")]
    pub single_thread: ::core::option::Option<bool>,
    #[prost(enumeration="SteppingGranularity", optional, tag="5")]
    pub granularity: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapContinueRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(int64, tag="3")]
    pub thread_id: i64,
    #[prost(bool, optional, tag="4")]
    pub single_thread: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapContinueResponse {
    #[prost(uint64, tag="1")]
    pub client_id: u64,
    #[prost(bool, optional, tag="2")]
    pub all_threads_continued: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapModulesRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapModulesResponse {
    #[prost(uint64, tag="1")]
    pub client_id: u64,
    #[prost(message, repeated, tag="2")]
    pub modules: ::prost::alloc::vec::Vec<DapModule>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapLoadedSourcesRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapLoadedSourcesResponse {
    #[prost(uint64, tag="1")]
    pub client_id: u64,
    #[prost(message, repeated, tag="2")]
    pub sources: ::prost::alloc::vec::Vec<DapSource>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapStackTraceRequest {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub client_id: u64,
    #[prost(int64, tag="3")]
    pub thread_id: i64,
    #[prost(uint64, optional, tag="4")]
    pub start_frame: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub stack_trace_levels: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapStackTraceResponse {
    #[prost(message, repeated, tag="1")]
    pub frames: ::prost::alloc::vec::Vec<DapStackFrame>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapStackFrame {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub source: ::core::option::Option<DapSource>,
    #[prost(uint64, tag="4")]
    pub line: u64,
    #[prost(uint64, tag="5")]
    pub column: u64,
    #[prost(uint64, optional, tag="6")]
    pub end_line: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="7")]
    pub end_column: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="8")]
    pub can_restart: ::core::option::Option<bool>,
    #[prost(string, optional, tag="9")]
    pub instruction_pointer_reference: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="10")]
    pub module_id: ::core::option::Option<DapModuleId>,
    #[prost(enumeration="DapStackPresentationHint", optional, tag="11")]
    pub presentation_hint: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebuggerLoadedSourceList {
    #[prost(uint64, tag="1")]
    pub client_id: u64,
    #[prost(message, repeated, tag="2")]
    pub sources: ::prost::alloc::vec::Vec<DapSource>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapVariables {
    #[prost(uint64, tag="1")]
    pub client_id: u64,
    #[prost(message, repeated, tag="2")]
    pub variables: ::prost::alloc::vec::Vec<DapVariable>,
}
/// Remote Debugging: Dap Types
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapVariable {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// optional DapVariablePresentationHint presentation_hint = 4;
    #[prost(string, optional, tag="5")]
    pub evaluate_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="6")]
    pub variables_reference: u64,
    #[prost(uint64, optional, tag="7")]
    pub named_variables: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="8")]
    pub indexed_variables: ::core::option::Option<u64>,
    #[prost(string, optional, tag="9")]
    pub memory_reference: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapThread {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapScope {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="DapScopePresentationHint", optional, tag="2")]
    pub presentation_hint: ::core::option::Option<i32>,
    #[prost(uint64, tag="3")]
    pub variables_reference: u64,
    #[prost(uint64, optional, tag="4")]
    pub named_variables: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub indexed_variables: ::core::option::Option<u64>,
    #[prost(bool, tag="6")]
    pub expensive: bool,
    #[prost(message, optional, tag="7")]
    pub source: ::core::option::Option<DapSource>,
    #[prost(uint64, optional, tag="8")]
    pub line: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="9")]
    pub column: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="10")]
    pub end_line: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="11")]
    pub end_column: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapSource {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub source_reference: ::core::option::Option<u64>,
    #[prost(enumeration="DapSourcePresentationHint", optional, tag="4")]
    pub presentation_hint: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub origin: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="6")]
    pub sources: ::prost::alloc::vec::Vec<DapSource>,
    #[prost(bytes="vec", optional, tag="7")]
    pub adapter_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="8")]
    pub checksums: ::prost::alloc::vec::Vec<DapChecksum>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapOutputEvent {
    #[prost(string, tag="1")]
    pub output: ::prost::alloc::string::String,
    #[prost(enumeration="DapOutputCategory", optional, tag="2")]
    pub category: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag="3")]
    pub variables_reference: ::core::option::Option<u64>,
    #[prost(enumeration="DapOutputEventGroup", optional, tag="4")]
    pub group: ::core::option::Option<i32>,
    #[prost(message, optional, tag="5")]
    pub source: ::core::option::Option<DapSource>,
    #[prost(uint32, optional, tag="6")]
    pub line: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub column: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapChecksum {
    #[prost(enumeration="DapChecksumAlgorithm", tag="1")]
    pub algorithm: i32,
    #[prost(string, tag="2")]
    pub checksum: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapModule {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<DapModuleId>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="4")]
    pub is_optimized: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="5")]
    pub is_user_code: ::core::option::Option<bool>,
    #[prost(string, optional, tag="6")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub symbol_status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub symbol_file_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub date_time_stamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub address_range: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugTaskDefinition {
    #[prost(string, tag="1")]
    pub adapter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub config: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub tcp_connection: ::core::option::Option<TcpHost>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpHost {
    #[prost(uint32, optional, tag="1")]
    pub port: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub timeout: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugLaunchRequest {
    #[prost(string, tag="1")]
    pub program: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub cwd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, string", tag="4")]
    pub env: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugAttachRequest {
    #[prost(uint32, tag="1")]
    pub process_id: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DapModuleId {
    #[prost(oneof="dap_module_id::Id", tags="1, 2")]
    pub id: ::core::option::Option<dap_module_id::Id>,
}
/// Nested message and enum types in `DapModuleId`.
pub mod dap_module_id {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(uint32, tag="1")]
        Number(u32),
        #[prost(string, tag="2")]
        String(::prost::alloc::string::String),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDebugAdapterBinary {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="3")]
    pub session_id: u64,
    #[prost(message, optional, tag="2")]
    pub definition: ::core::option::Option<DebugTaskDefinition>,
    #[prost(uint64, tag="4")]
    pub worktree_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugAdapterBinary {
    #[prost(string, optional, tag="1")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, string", tag="3")]
    pub envs: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub cwd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub connection: ::core::option::Option<TcpHost>,
    #[prost(string, tag="7")]
    pub configuration: ::prost::alloc::string::String,
    #[prost(enumeration="debug_adapter_binary::LaunchType", tag="8")]
    pub launch_type: i32,
}
/// Nested message and enum types in `DebugAdapterBinary`.
pub mod debug_adapter_binary {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LaunchType {
        Attach = 0,
        Launch = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunDebugLocators {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub build_command: ::core::option::Option<SpawnInTerminal>,
    #[prost(string, tag="3")]
    pub locator: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugRequest {
    #[prost(oneof="debug_request::Request", tags="1, 2")]
    pub request: ::core::option::Option<debug_request::Request>,
}
/// Nested message and enum types in `DebugRequest`.
pub mod debug_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        DebugLaunchRequest(super::DebugLaunchRequest),
        #[prost(message, tag="2")]
        DebugAttachRequest(super::DebugAttachRequest),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugScenario {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub adapter: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub request: ::core::option::Option<DebugRequest>,
    #[prost(message, optional, tag="5")]
    pub connection: ::core::option::Option<TcpHost>,
    #[prost(bool, optional, tag="6")]
    pub stop_on_entry: ::core::option::Option<bool>,
    #[prost(string, optional, tag="7")]
    pub configuration: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogToDebugConsole {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub session_id: u64,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProcesses {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProcessesResponse {
    #[prost(message, repeated, tag="1")]
    pub processes: ::prost::alloc::vec::Vec<ProcessInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessInfo {
    #[prost(uint32, tag="1")]
    pub pid: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BreakpointState {
    Enabled = 0,
    Disabled = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapThreadStatus {
    Running = 0,
    Stopped = 1,
    Exited = 2,
    Ended = 3,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VariablesArgumentsFilter {
    Indexed = 0,
    Named = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SteppingGranularity {
    Statement = 0,
    Line = 1,
    Instruction = 2,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapEvaluateContext {
    Repl = 0,
    Watch = 1,
    Hover = 2,
    Clipboard = 3,
    EvaluateVariables = 4,
    EvaluateUnknown = 5,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapCompletionItemType {
    Method = 0,
    Function = 1,
    Constructor = 2,
    Field = 3,
    Variable = 4,
    Class = 5,
    Interface = 6,
    Module = 7,
    Property = 8,
    Unit = 9,
    Value = 10,
    Enum = 11,
    Keyword = 12,
    Snippet = 13,
    Text = 14,
    Color = 15,
    CompletionItemFile = 16,
    Reference = 17,
    Customcolor = 19,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapOutputCategory {
    ConsoleOutput = 0,
    Important = 1,
    Stdout = 2,
    Stderr = 3,
    Unknown = 4,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapOutputEventGroup {
    Start = 0,
    StartCollapsed = 1,
    End = 2,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapChecksumAlgorithm {
    ChecksumAlgorithmUnspecified = 0,
    Md5 = 1,
    Sha1 = 2,
    Sha256 = 3,
    Timestamp = 4,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapScopePresentationHint {
    Arguments = 0,
    Locals = 1,
    Registers = 2,
    ReturnValue = 3,
    ScopeUnknown = 4,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapSourcePresentationHint {
    SourceNormal = 0,
    Emphasize = 1,
    Deemphasize = 2,
    SourceUnknown = 3,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DapStackPresentationHint {
    StackNormal = 0,
    Label = 1,
    Subtle = 2,
    StackUnknown = 3,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenImageByPath {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenImageResponse {
    #[prost(uint64, tag="1")]
    pub image_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateImageForPeer {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(message, optional, tag="2")]
    pub peer_id: ::core::option::Option<PeerId>,
    #[prost(oneof="create_image_for_peer::Variant", tags="3, 4")]
    pub variant: ::core::option::Option<create_image_for_peer::Variant>,
}
/// Nested message and enum types in `CreateImageForPeer`.
pub mod create_image_for_peer {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        #[prost(message, tag="3")]
        State(super::ImageState),
        #[prost(message, tag="4")]
        Chunk(super::ImageChunk),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageState {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, optional, tag="2")]
    pub file: ::core::option::Option<File>,
    #[prost(uint64, tag="3")]
    pub content_size: u64,
    /// e.g., "png", "jpeg", "webp", etc.
    #[prost(string, tag="4")]
    pub format: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageChunk {
    #[prost(uint64, tag="1")]
    pub image_id: u64,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotifications {
    #[prost(uint64, optional, tag="1")]
    pub before_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNotification {
    #[prost(message, optional, tag="1")]
    pub notification: ::core::option::Option<Notification>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationsResponse {
    #[prost(message, repeated, tag="1")]
    pub notifications: ::prost::alloc::vec::Vec<Notification>,
    #[prost(bool, tag="2")]
    pub done: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotification {
    #[prost(uint64, tag="1")]
    pub notification_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotification {
    #[prost(message, optional, tag="1")]
    pub notification: ::core::option::Option<Notification>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkNotificationRead {
    #[prost(uint64, tag="1")]
    pub notification_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(uint64, tag="2")]
    pub timestamp: u64,
    #[prost(string, tag="3")]
    pub kind: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="4")]
    pub entity_id: ::core::option::Option<u64>,
    #[prost(string, tag="5")]
    pub content: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub is_read: bool,
    #[prost(bool, optional, tag="7")]
    pub response: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListToolchains {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(string, tag="3")]
    pub language_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Toolchain {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub raw_json: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolchainGroup {
    #[prost(uint64, tag="1")]
    pub start_index: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListToolchainsResponse {
    #[prost(message, repeated, tag="1")]
    pub toolchains: ::prost::alloc::vec::Vec<Toolchain>,
    #[prost(bool, tag="2")]
    pub has_values: bool,
    #[prost(message, repeated, tag="3")]
    pub groups: ::prost::alloc::vec::Vec<ToolchainGroup>,
    #[prost(string, optional, tag="4")]
    pub relative_worktree_path: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateToolchain {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(message, optional, tag="3")]
    pub toolchain: ::core::option::Option<Toolchain>,
    #[prost(string, tag="4")]
    pub language_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="5")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveToolchain {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(uint64, tag="2")]
    pub worktree_id: u64,
    #[prost(string, tag="3")]
    pub language_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveToolchainResponse {
    #[prost(message, optional, tag="1")]
    pub toolchain: ::core::option::Option<Toolchain>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveToolchain {
    #[prost(uint64, tag="1")]
    pub project_id: u64,
    #[prost(string, tag="2")]
    pub abs_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub language_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveToolchainResponse {
    #[prost(oneof="resolve_toolchain_response::Response", tags="1, 2")]
    pub response: ::core::option::Option<resolve_toolchain_response::Response>,
}
/// Nested message and enum types in `ResolveToolchainResponse`.
pub mod resolve_toolchain_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        Toolchain(super::Toolchain),
        #[prost(string, tag="2")]
        Error(::prost::alloc::string::String),
    }
}
// Looking for a number? Search "// current max"

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Envelope {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(uint32, optional, tag="2")]
    pub responding_to: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub original_sender_id: ::core::option::Option<PeerId>,
    #[prost(uint32, optional, tag="266")]
    pub ack_id: ::core::option::Option<u32>,
    #[prost(oneof="envelope::Payload", tags="4, 5, 6, 7, 8, 165, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 237, 238, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 291, 292, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 207, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 147, 349, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 162, 163, 170, 171, 172, 173, 174, 208, 186, 187, 196, 198, 199, 203, 204, 209, 210, 211, 212, 213, 232, 233, 214, 215, 216, 217, 218, 219, 220, 222, 223, 241, 242, 243, 244, 245, 257, 258, 260, 261, 262, 263, 264, 265, 267, 268, 269, 271, 272, 273, 274, 275, 276, 277, 278, 279, 282, 283, 284, 285, 286, 287, 288, 289, 290, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346, 347, 348, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 397, 398, 399, 400, 401, 402, 403, 404, 405")]
    pub payload: ::core::option::Option<envelope::Payload>,
}
/// Nested message and enum types in `Envelope`.
pub mod envelope {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag="4")]
        Hello(super::Hello),
        #[prost(message, tag="5")]
        Ack(super::Ack),
        #[prost(message, tag="6")]
        Error(super::Error),
        #[prost(message, tag="7")]
        Ping(super::Ping),
        #[prost(message, tag="8")]
        Test(super::Test),
        #[prost(message, tag="165")]
        EndStream(super::EndStream),
        #[prost(message, tag="9")]
        CreateRoom(super::CreateRoom),
        #[prost(message, tag="10")]
        CreateRoomResponse(super::CreateRoomResponse),
        #[prost(message, tag="11")]
        JoinRoom(super::JoinRoom),
        #[prost(message, tag="12")]
        JoinRoomResponse(super::JoinRoomResponse),
        #[prost(message, tag="13")]
        RejoinRoom(super::RejoinRoom),
        #[prost(message, tag="14")]
        RejoinRoomResponse(super::RejoinRoomResponse),
        #[prost(message, tag="15")]
        LeaveRoom(super::LeaveRoom),
        #[prost(message, tag="16")]
        Call(super::Call),
        #[prost(message, tag="17")]
        IncomingCall(super::IncomingCall),
        #[prost(message, tag="18")]
        CallCanceled(super::CallCanceled),
        #[prost(message, tag="19")]
        CancelCall(super::CancelCall),
        #[prost(message, tag="20")]
        DeclineCall(super::DeclineCall),
        #[prost(message, tag="21")]
        UpdateParticipantLocation(super::UpdateParticipantLocation),
        #[prost(message, tag="22")]
        RoomUpdated(super::RoomUpdated),
        #[prost(message, tag="23")]
        ShareProject(super::ShareProject),
        #[prost(message, tag="24")]
        ShareProjectResponse(super::ShareProjectResponse),
        #[prost(message, tag="25")]
        UnshareProject(super::UnshareProject),
        #[prost(message, tag="26")]
        JoinProject(super::JoinProject),
        #[prost(message, tag="27")]
        JoinProjectResponse(super::JoinProjectResponse),
        #[prost(message, tag="28")]
        LeaveProject(super::LeaveProject),
        #[prost(message, tag="29")]
        AddProjectCollaborator(super::AddProjectCollaborator),
        #[prost(message, tag="30")]
        UpdateProjectCollaborator(super::UpdateProjectCollaborator),
        #[prost(message, tag="31")]
        RemoveProjectCollaborator(super::RemoveProjectCollaborator),
        #[prost(message, tag="32")]
        GetDefinition(super::GetDefinition),
        #[prost(message, tag="33")]
        GetDefinitionResponse(super::GetDefinitionResponse),
        #[prost(message, tag="237")]
        GetDeclaration(super::GetDeclaration),
        #[prost(message, tag="238")]
        GetDeclarationResponse(super::GetDeclarationResponse),
        #[prost(message, tag="34")]
        GetTypeDefinition(super::GetTypeDefinition),
        #[prost(message, tag="35")]
        GetTypeDefinitionResponse(super::GetTypeDefinitionResponse),
        #[prost(message, tag="36")]
        GetReferences(super::GetReferences),
        #[prost(message, tag="37")]
        GetReferencesResponse(super::GetReferencesResponse),
        #[prost(message, tag="38")]
        GetDocumentHighlights(super::GetDocumentHighlights),
        #[prost(message, tag="39")]
        GetDocumentHighlightsResponse(super::GetDocumentHighlightsResponse),
        #[prost(message, tag="40")]
        GetProjectSymbols(super::GetProjectSymbols),
        #[prost(message, tag="41")]
        GetProjectSymbolsResponse(super::GetProjectSymbolsResponse),
        #[prost(message, tag="42")]
        OpenBufferForSymbol(super::OpenBufferForSymbol),
        #[prost(message, tag="43")]
        OpenBufferForSymbolResponse(super::OpenBufferForSymbolResponse),
        #[prost(message, tag="44")]
        UpdateProject(super::UpdateProject),
        #[prost(message, tag="45")]
        UpdateWorktree(super::UpdateWorktree),
        #[prost(message, tag="46")]
        CreateProjectEntry(super::CreateProjectEntry),
        #[prost(message, tag="47")]
        RenameProjectEntry(super::RenameProjectEntry),
        #[prost(message, tag="48")]
        CopyProjectEntry(super::CopyProjectEntry),
        #[prost(message, tag="49")]
        DeleteProjectEntry(super::DeleteProjectEntry),
        #[prost(message, tag="50")]
        ProjectEntryResponse(super::ProjectEntryResponse),
        #[prost(message, tag="51")]
        ExpandProjectEntry(super::ExpandProjectEntry),
        #[prost(message, tag="52")]
        ExpandProjectEntryResponse(super::ExpandProjectEntryResponse),
        #[prost(message, tag="291")]
        ExpandAllForProjectEntry(super::ExpandAllForProjectEntry),
        #[prost(message, tag="292")]
        ExpandAllForProjectEntryResponse(super::ExpandAllForProjectEntryResponse),
        #[prost(message, tag="53")]
        UpdateDiagnosticSummary(super::UpdateDiagnosticSummary),
        #[prost(message, tag="54")]
        StartLanguageServer(super::StartLanguageServer),
        #[prost(message, tag="55")]
        UpdateLanguageServer(super::UpdateLanguageServer),
        #[prost(message, tag="56")]
        OpenBufferById(super::OpenBufferById),
        #[prost(message, tag="57")]
        OpenBufferByPath(super::OpenBufferByPath),
        #[prost(message, tag="58")]
        OpenBufferResponse(super::OpenBufferResponse),
        #[prost(message, tag="59")]
        CreateBufferForPeer(super::CreateBufferForPeer),
        #[prost(message, tag="60")]
        UpdateBuffer(super::UpdateBuffer),
        #[prost(message, tag="61")]
        UpdateBufferFile(super::UpdateBufferFile),
        #[prost(message, tag="62")]
        SaveBuffer(super::SaveBuffer),
        #[prost(message, tag="63")]
        BufferSaved(super::BufferSaved),
        #[prost(message, tag="64")]
        BufferReloaded(super::BufferReloaded),
        #[prost(message, tag="65")]
        ReloadBuffers(super::ReloadBuffers),
        #[prost(message, tag="66")]
        ReloadBuffersResponse(super::ReloadBuffersResponse),
        #[prost(message, tag="67")]
        SynchronizeBuffers(super::SynchronizeBuffers),
        #[prost(message, tag="68")]
        SynchronizeBuffersResponse(super::SynchronizeBuffersResponse),
        #[prost(message, tag="69")]
        FormatBuffers(super::FormatBuffers),
        #[prost(message, tag="70")]
        FormatBuffersResponse(super::FormatBuffersResponse),
        #[prost(message, tag="71")]
        GetCompletions(super::GetCompletions),
        #[prost(message, tag="72")]
        GetCompletionsResponse(super::GetCompletionsResponse),
        #[prost(message, tag="73")]
        ResolveCompletionDocumentation(super::ResolveCompletionDocumentation),
        #[prost(message, tag="74")]
        ResolveCompletionDocumentationResponse(super::ResolveCompletionDocumentationResponse),
        #[prost(message, tag="75")]
        ApplyCompletionAdditionalEdits(super::ApplyCompletionAdditionalEdits),
        #[prost(message, tag="76")]
        ApplyCompletionAdditionalEditsResponse(super::ApplyCompletionAdditionalEditsResponse),
        #[prost(message, tag="77")]
        GetCodeActions(super::GetCodeActions),
        #[prost(message, tag="78")]
        GetCodeActionsResponse(super::GetCodeActionsResponse),
        #[prost(message, tag="79")]
        GetHover(super::GetHover),
        #[prost(message, tag="80")]
        GetHoverResponse(super::GetHoverResponse),
        #[prost(message, tag="81")]
        ApplyCodeAction(super::ApplyCodeAction),
        #[prost(message, tag="82")]
        ApplyCodeActionResponse(super::ApplyCodeActionResponse),
        #[prost(message, tag="83")]
        PrepareRename(super::PrepareRename),
        #[prost(message, tag="84")]
        PrepareRenameResponse(super::PrepareRenameResponse),
        #[prost(message, tag="85")]
        PerformRename(super::PerformRename),
        #[prost(message, tag="86")]
        PerformRenameResponse(super::PerformRenameResponse),
        #[prost(message, tag="89")]
        UpdateContacts(super::UpdateContacts),
        #[prost(message, tag="90")]
        UpdateInviteInfo(super::UpdateInviteInfo),
        #[prost(message, tag="91")]
        ShowContacts(super::ShowContacts),
        #[prost(message, tag="92")]
        GetUsers(super::GetUsers),
        #[prost(message, tag="93")]
        FuzzySearchUsers(super::FuzzySearchUsers),
        #[prost(message, tag="94")]
        UsersResponse(super::UsersResponse),
        #[prost(message, tag="95")]
        RequestContact(super::RequestContact),
        #[prost(message, tag="96")]
        RespondToContactRequest(super::RespondToContactRequest),
        #[prost(message, tag="97")]
        RemoveContact(super::RemoveContact),
        #[prost(message, tag="98")]
        Follow(super::Follow),
        #[prost(message, tag="99")]
        FollowResponse(super::FollowResponse),
        #[prost(message, tag="100")]
        UpdateFollowers(super::UpdateFollowers),
        #[prost(message, tag="101")]
        Unfollow(super::Unfollow),
        #[prost(message, tag="104")]
        UpdateDiffBases(super::UpdateDiffBases),
        #[prost(message, tag="105")]
        OnTypeFormatting(super::OnTypeFormatting),
        #[prost(message, tag="106")]
        OnTypeFormattingResponse(super::OnTypeFormattingResponse),
        #[prost(message, tag="107")]
        UpdateWorktreeSettings(super::UpdateWorktreeSettings),
        #[prost(message, tag="108")]
        InlayHints(super::InlayHints),
        #[prost(message, tag="109")]
        InlayHintsResponse(super::InlayHintsResponse),
        #[prost(message, tag="110")]
        ResolveInlayHint(super::ResolveInlayHint),
        #[prost(message, tag="111")]
        ResolveInlayHintResponse(super::ResolveInlayHintResponse),
        #[prost(message, tag="112")]
        RefreshInlayHints(super::RefreshInlayHints),
        #[prost(message, tag="113")]
        CreateChannel(super::CreateChannel),
        #[prost(message, tag="114")]
        CreateChannelResponse(super::CreateChannelResponse),
        #[prost(message, tag="115")]
        InviteChannelMember(super::InviteChannelMember),
        #[prost(message, tag="116")]
        RemoveChannelMember(super::RemoveChannelMember),
        #[prost(message, tag="117")]
        RespondToChannelInvite(super::RespondToChannelInvite),
        #[prost(message, tag="118")]
        UpdateChannels(super::UpdateChannels),
        #[prost(message, tag="119")]
        JoinChannel(super::JoinChannel),
        #[prost(message, tag="120")]
        DeleteChannel(super::DeleteChannel),
        #[prost(message, tag="121")]
        GetChannelMembers(super::GetChannelMembers),
        #[prost(message, tag="122")]
        GetChannelMembersResponse(super::GetChannelMembersResponse),
        #[prost(message, tag="123")]
        SetChannelMemberRole(super::SetChannelMemberRole),
        #[prost(message, tag="124")]
        RenameChannel(super::RenameChannel),
        #[prost(message, tag="125")]
        RenameChannelResponse(super::RenameChannelResponse),
        #[prost(message, tag="207")]
        SubscribeToChannels(super::SubscribeToChannels),
        #[prost(message, tag="126")]
        JoinChannelBuffer(super::JoinChannelBuffer),
        #[prost(message, tag="127")]
        JoinChannelBufferResponse(super::JoinChannelBufferResponse),
        #[prost(message, tag="128")]
        UpdateChannelBuffer(super::UpdateChannelBuffer),
        #[prost(message, tag="129")]
        LeaveChannelBuffer(super::LeaveChannelBuffer),
        #[prost(message, tag="130")]
        UpdateChannelBufferCollaborators(super::UpdateChannelBufferCollaborators),
        #[prost(message, tag="131")]
        RejoinChannelBuffers(super::RejoinChannelBuffers),
        #[prost(message, tag="132")]
        RejoinChannelBuffersResponse(super::RejoinChannelBuffersResponse),
        #[prost(message, tag="133")]
        AckBufferOperation(super::AckBufferOperation),
        #[prost(message, tag="134")]
        JoinChannelChat(super::JoinChannelChat),
        #[prost(message, tag="135")]
        JoinChannelChatResponse(super::JoinChannelChatResponse),
        #[prost(message, tag="136")]
        LeaveChannelChat(super::LeaveChannelChat),
        #[prost(message, tag="137")]
        SendChannelMessage(super::SendChannelMessage),
        #[prost(message, tag="138")]
        SendChannelMessageResponse(super::SendChannelMessageResponse),
        #[prost(message, tag="139")]
        ChannelMessageSent(super::ChannelMessageSent),
        #[prost(message, tag="140")]
        GetChannelMessages(super::GetChannelMessages),
        #[prost(message, tag="141")]
        GetChannelMessagesResponse(super::GetChannelMessagesResponse),
        #[prost(message, tag="142")]
        RemoveChannelMessage(super::RemoveChannelMessage),
        #[prost(message, tag="143")]
        AckChannelMessage(super::AckChannelMessage),
        #[prost(message, tag="144")]
        GetChannelMessagesById(super::GetChannelMessagesById),
        #[prost(message, tag="147")]
        MoveChannel(super::MoveChannel),
        #[prost(message, tag="349")]
        ReorderChannel(super::ReorderChannel),
        #[prost(message, tag="148")]
        SetChannelVisibility(super::SetChannelVisibility),
        #[prost(message, tag="149")]
        AddNotification(super::AddNotification),
        #[prost(message, tag="150")]
        GetNotifications(super::GetNotifications),
        #[prost(message, tag="151")]
        GetNotificationsResponse(super::GetNotificationsResponse),
        #[prost(message, tag="152")]
        DeleteNotification(super::DeleteNotification),
        #[prost(message, tag="153")]
        MarkNotificationRead(super::MarkNotificationRead),
        #[prost(message, tag="154")]
        LspExtExpandMacro(super::LspExtExpandMacro),
        #[prost(message, tag="155")]
        LspExtExpandMacroResponse(super::LspExtExpandMacroResponse),
        #[prost(message, tag="156")]
        SetRoomParticipantRole(super::SetRoomParticipantRole),
        #[prost(message, tag="157")]
        UpdateUserChannels(super::UpdateUserChannels),
        #[prost(message, tag="162")]
        GetImplementation(super::GetImplementation),
        #[prost(message, tag="163")]
        GetImplementationResponse(super::GetImplementationResponse),
        #[prost(message, tag="170")]
        UpdateChannelMessage(super::UpdateChannelMessage),
        #[prost(message, tag="171")]
        ChannelMessageUpdate(super::ChannelMessageUpdate),
        #[prost(message, tag="172")]
        BlameBuffer(super::BlameBuffer),
        #[prost(message, tag="173")]
        BlameBufferResponse(super::BlameBufferResponse),
        #[prost(message, tag="174")]
        UpdateNotification(super::UpdateNotification),
        #[prost(message, tag="208")]
        RestartLanguageServers(super::RestartLanguageServers),
        #[prost(message, tag="186")]
        RejoinRemoteProjects(super::RejoinRemoteProjects),
        #[prost(message, tag="187")]
        RejoinRemoteProjectsResponse(super::RejoinRemoteProjectsResponse),
        #[prost(message, tag="196")]
        OpenNewBuffer(super::OpenNewBuffer),
        #[prost(message, tag="198")]
        GetSupermavenApiKey(super::GetSupermavenApiKey),
        #[prost(message, tag="199")]
        GetSupermavenApiKeyResponse(super::GetSupermavenApiKeyResponse),
        #[prost(message, tag="203")]
        TaskContextForLocation(super::TaskContextForLocation),
        #[prost(message, tag="204")]
        TaskContext(super::TaskContext),
        #[prost(message, tag="209")]
        LinkedEditingRange(super::LinkedEditingRange),
        #[prost(message, tag="210")]
        LinkedEditingRangeResponse(super::LinkedEditingRangeResponse),
        #[prost(message, tag="211")]
        AdvertiseContexts(super::AdvertiseContexts),
        #[prost(message, tag="212")]
        OpenContext(super::OpenContext),
        #[prost(message, tag="213")]
        OpenContextResponse(super::OpenContextResponse),
        #[prost(message, tag="232")]
        CreateContext(super::CreateContext),
        #[prost(message, tag="233")]
        CreateContextResponse(super::CreateContextResponse),
        #[prost(message, tag="214")]
        UpdateContext(super::UpdateContext),
        #[prost(message, tag="215")]
        SynchronizeContexts(super::SynchronizeContexts),
        #[prost(message, tag="216")]
        SynchronizeContextsResponse(super::SynchronizeContextsResponse),
        #[prost(message, tag="217")]
        GetSignatureHelp(super::GetSignatureHelp),
        #[prost(message, tag="218")]
        GetSignatureHelpResponse(super::GetSignatureHelpResponse),
        #[prost(message, tag="219")]
        ListRemoteDirectory(super::ListRemoteDirectory),
        #[prost(message, tag="220")]
        ListRemoteDirectoryResponse(super::ListRemoteDirectoryResponse),
        #[prost(message, tag="222")]
        AddWorktree(super::AddWorktree),
        #[prost(message, tag="223")]
        AddWorktreeResponse(super::AddWorktreeResponse),
        #[prost(message, tag="241")]
        LspExtSwitchSourceHeader(super::LspExtSwitchSourceHeader),
        #[prost(message, tag="242")]
        LspExtSwitchSourceHeaderResponse(super::LspExtSwitchSourceHeaderResponse),
        #[prost(message, tag="243")]
        FindSearchCandidates(super::FindSearchCandidates),
        #[prost(message, tag="244")]
        FindSearchCandidatesResponse(super::FindSearchCandidatesResponse),
        #[prost(message, tag="245")]
        CloseBuffer(super::CloseBuffer),
        #[prost(message, tag="257")]
        ShutdownRemoteServer(super::ShutdownRemoteServer),
        #[prost(message, tag="258")]
        RemoveWorktree(super::RemoveWorktree),
        #[prost(message, tag="260")]
        LanguageServerLog(super::LanguageServerLog),
        #[prost(message, tag="261")]
        Toast(super::Toast),
        #[prost(message, tag="262")]
        HideToast(super::HideToast),
        #[prost(message, tag="263")]
        OpenServerSettings(super::OpenServerSettings),
        #[prost(message, tag="264")]
        GetPermalinkToLine(super::GetPermalinkToLine),
        #[prost(message, tag="265")]
        GetPermalinkToLineResponse(super::GetPermalinkToLineResponse),
        #[prost(message, tag="267")]
        FlushBufferedMessages(super::FlushBufferedMessages),
        #[prost(message, tag="268")]
        LanguageServerPromptRequest(super::LanguageServerPromptRequest),
        #[prost(message, tag="269")]
        LanguageServerPromptResponse(super::LanguageServerPromptResponse),
        #[prost(message, tag="271")]
        GitBranchesResponse(super::GitBranchesResponse),
        #[prost(message, tag="272")]
        UpdateGitBranch(super::UpdateGitBranch),
        #[prost(message, tag="273")]
        ListToolchains(super::ListToolchains),
        #[prost(message, tag="274")]
        ListToolchainsResponse(super::ListToolchainsResponse),
        #[prost(message, tag="275")]
        ActivateToolchain(super::ActivateToolchain),
        #[prost(message, tag="276")]
        ActiveToolchain(super::ActiveToolchain),
        #[prost(message, tag="277")]
        ActiveToolchainResponse(super::ActiveToolchainResponse),
        #[prost(message, tag="278")]
        GetPathMetadata(super::GetPathMetadata),
        #[prost(message, tag="279")]
        GetPathMetadataResponse(super::GetPathMetadataResponse),
        #[prost(message, tag="282")]
        CancelLanguageServerWork(super::CancelLanguageServerWork),
        #[prost(message, tag="283")]
        LspExtOpenDocs(super::LspExtOpenDocs),
        #[prost(message, tag="284")]
        LspExtOpenDocsResponse(super::LspExtOpenDocsResponse),
        #[prost(message, tag="285")]
        SyncExtensions(super::SyncExtensions),
        #[prost(message, tag="286")]
        SyncExtensionsResponse(super::SyncExtensionsResponse),
        #[prost(message, tag="287")]
        InstallExtension(super::InstallExtension),
        #[prost(message, tag="288")]
        OpenUnstagedDiff(super::OpenUnstagedDiff),
        #[prost(message, tag="289")]
        OpenUnstagedDiffResponse(super::OpenUnstagedDiffResponse),
        #[prost(message, tag="290")]
        RegisterBufferWithLanguageServers(super::RegisterBufferWithLanguageServers),
        #[prost(message, tag="293")]
        Stage(super::Stage),
        #[prost(message, tag="294")]
        Unstage(super::Unstage),
        #[prost(message, tag="295")]
        Commit(super::Commit),
        #[prost(message, tag="296")]
        OpenCommitMessageBuffer(super::OpenCommitMessageBuffer),
        #[prost(message, tag="297")]
        OpenUncommittedDiff(super::OpenUncommittedDiff),
        #[prost(message, tag="298")]
        OpenUncommittedDiffResponse(super::OpenUncommittedDiffResponse),
        #[prost(message, tag="299")]
        SetIndexText(super::SetIndexText),
        #[prost(message, tag="300")]
        GitShow(super::GitShow),
        #[prost(message, tag="301")]
        GitReset(super::GitReset),
        #[prost(message, tag="302")]
        GitCommitDetails(super::GitCommitDetails),
        #[prost(message, tag="303")]
        GitCheckoutFiles(super::GitCheckoutFiles),
        #[prost(message, tag="304")]
        Push(super::Push),
        #[prost(message, tag="305")]
        Fetch(super::Fetch),
        #[prost(message, tag="306")]
        GetRemotes(super::GetRemotes),
        #[prost(message, tag="307")]
        GetRemotesResponse(super::GetRemotesResponse),
        #[prost(message, tag="308")]
        Pull(super::Pull),
        #[prost(message, tag="309")]
        ApplyCodeActionKind(super::ApplyCodeActionKind),
        #[prost(message, tag="310")]
        ApplyCodeActionKindResponse(super::ApplyCodeActionKindResponse),
        #[prost(message, tag="311")]
        RemoteMessageResponse(super::RemoteMessageResponse),
        #[prost(message, tag="312")]
        GitGetBranches(super::GitGetBranches),
        #[prost(message, tag="313")]
        GitCreateBranch(super::GitCreateBranch),
        #[prost(message, tag="314")]
        GitChangeBranch(super::GitChangeBranch),
        #[prost(message, tag="315")]
        CheckForPushedCommits(super::CheckForPushedCommits),
        #[prost(message, tag="316")]
        CheckForPushedCommitsResponse(super::CheckForPushedCommitsResponse),
        #[prost(message, tag="317")]
        AskPassRequest(super::AskPassRequest),
        #[prost(message, tag="318")]
        AskPassResponse(super::AskPassResponse),
        #[prost(message, tag="319")]
        GitDiff(super::GitDiff),
        #[prost(message, tag="320")]
        GitDiffResponse(super::GitDiffResponse),
        #[prost(message, tag="321")]
        GitInit(super::GitInit),
        #[prost(message, tag="322")]
        CodeLens(super::CodeLens),
        #[prost(message, tag="323")]
        GetCodeLens(super::GetCodeLens),
        #[prost(message, tag="324")]
        GetCodeLensResponse(super::GetCodeLensResponse),
        #[prost(message, tag="325")]
        RefreshCodeLens(super::RefreshCodeLens),
        #[prost(message, tag="326")]
        ToggleBreakpoint(super::ToggleBreakpoint),
        #[prost(message, tag="327")]
        BreakpointsForFile(super::BreakpointsForFile),
        #[prost(message, tag="328")]
        UpdateRepository(super::UpdateRepository),
        #[prost(message, tag="329")]
        RemoveRepository(super::RemoveRepository),
        #[prost(message, tag="330")]
        GetDocumentSymbols(super::GetDocumentSymbols),
        #[prost(message, tag="331")]
        GetDocumentSymbolsResponse(super::GetDocumentSymbolsResponse),
        #[prost(message, tag="334")]
        LoadCommitDiff(super::LoadCommitDiff),
        #[prost(message, tag="335")]
        LoadCommitDiffResponse(super::LoadCommitDiffResponse),
        #[prost(message, tag="336")]
        StopLanguageServers(super::StopLanguageServers),
        #[prost(message, tag="337")]
        LspExtRunnables(super::LspExtRunnables),
        #[prost(message, tag="338")]
        LspExtRunnablesResponse(super::LspExtRunnablesResponse),
        #[prost(message, tag="339")]
        GetDebugAdapterBinary(super::GetDebugAdapterBinary),
        #[prost(message, tag="340")]
        DebugAdapterBinary(super::DebugAdapterBinary),
        #[prost(message, tag="341")]
        RunDebugLocators(super::RunDebugLocators),
        #[prost(message, tag="342")]
        DebugRequest(super::DebugRequest),
        #[prost(message, tag="343")]
        LspExtGoToParentModule(super::LspExtGoToParentModule),
        #[prost(message, tag="344")]
        LspExtGoToParentModuleResponse(super::LspExtGoToParentModuleResponse),
        #[prost(message, tag="345")]
        LspExtCancelFlycheck(super::LspExtCancelFlycheck),
        #[prost(message, tag="346")]
        LspExtRunFlycheck(super::LspExtRunFlycheck),
        #[prost(message, tag="347")]
        LspExtClearFlycheck(super::LspExtClearFlycheck),
        #[prost(message, tag="348")]
        LogToDebugConsole(super::LogToDebugConsole),
        #[prost(message, tag="350")]
        GetDocumentDiagnostics(super::GetDocumentDiagnostics),
        #[prost(message, tag="351")]
        GetDocumentDiagnosticsResponse(super::GetDocumentDiagnosticsResponse),
        #[prost(message, tag="352")]
        PullWorkspaceDiagnostics(super::PullWorkspaceDiagnostics),
        #[prost(message, tag="353")]
        GetDocumentColor(super::GetDocumentColor),
        #[prost(message, tag="354")]
        GetDocumentColorResponse(super::GetDocumentColorResponse),
        #[prost(message, tag="355")]
        GetColorPresentation(super::GetColorPresentation),
        #[prost(message, tag="356")]
        GetColorPresentationResponse(super::GetColorPresentationResponse),
        #[prost(message, tag="357")]
        Stash(super::Stash),
        #[prost(message, tag="358")]
        StashPop(super::StashPop),
        #[prost(message, tag="359")]
        GetDefaultBranch(super::GetDefaultBranch),
        #[prost(message, tag="360")]
        GetDefaultBranchResponse(super::GetDefaultBranchResponse),
        #[prost(message, tag="361")]
        GetCrashFiles(super::GetCrashFiles),
        #[prost(message, tag="362")]
        GetCrashFilesResponse(super::GetCrashFilesResponse),
        #[prost(message, tag="363")]
        GitClone(super::GitClone),
        #[prost(message, tag="364")]
        GitCloneResponse(super::GitCloneResponse),
        #[prost(message, tag="365")]
        LspQuery(super::LspQuery),
        #[prost(message, tag="366")]
        LspQueryResponse(super::LspQueryResponse),
        #[prost(message, tag="367")]
        ToggleLspLogs(super::ToggleLspLogs),
        #[prost(message, tag="368")]
        UpdateUserSettings(super::UpdateUserSettings),
        #[prost(message, tag="369")]
        GetProcesses(super::GetProcesses),
        #[prost(message, tag="370")]
        GetProcessesResponse(super::GetProcessesResponse),
        #[prost(message, tag="371")]
        ResolveToolchain(super::ResolveToolchain),
        #[prost(message, tag="372")]
        ResolveToolchainResponse(super::ResolveToolchainResponse),
        #[prost(message, tag="373")]
        GetAgentServerCommand(super::GetAgentServerCommand),
        #[prost(message, tag="374")]
        AgentServerCommand(super::AgentServerCommand),
        #[prost(message, tag="375")]
        ExternalAgentsUpdated(super::ExternalAgentsUpdated),
        #[prost(message, tag="376")]
        ExternalAgentLoadingStatusUpdated(super::ExternalAgentLoadingStatusUpdated),
        #[prost(message, tag="377")]
        NewExternalAgentVersionAvailable(super::NewExternalAgentVersionAvailable),
        #[prost(message, tag="378")]
        StashDrop(super::StashDrop),
        #[prost(message, tag="379")]
        StashApply(super::StashApply),
        #[prost(message, tag="380")]
        GitRenameBranch(super::GitRenameBranch),
        #[prost(message, tag="381")]
        RemoteStarted(super::RemoteStarted),
        #[prost(message, tag="382")]
        GetDirectoryEnvironment(super::GetDirectoryEnvironment),
        #[prost(message, tag="383")]
        DirectoryEnvironment(super::DirectoryEnvironment),
        #[prost(message, tag="384")]
        GetTreeDiff(super::GetTreeDiff),
        #[prost(message, tag="385")]
        GetTreeDiffResponse(super::GetTreeDiffResponse),
        #[prost(message, tag="386")]
        GetBlobContent(super::GetBlobContent),
        #[prost(message, tag="387")]
        GetBlobContentResponse(super::GetBlobContentResponse),
        #[prost(message, tag="388")]
        GitWorktreesResponse(super::GitWorktreesResponse),
        #[prost(message, tag="389")]
        GitGetWorktrees(super::GitGetWorktrees),
        #[prost(message, tag="390")]
        GitCreateWorktree(super::GitCreateWorktree),
        #[prost(message, tag="391")]
        OpenImageByPath(super::OpenImageByPath),
        #[prost(message, tag="392")]
        OpenImageResponse(super::OpenImageResponse),
        #[prost(message, tag="393")]
        CreateImageForPeer(super::CreateImageForPeer),
        #[prost(message, tag="397")]
        GitFileHistory(super::GitFileHistory),
        #[prost(message, tag="398")]
        GitFileHistoryResponse(super::GitFileHistoryResponse),
        #[prost(message, tag="399")]
        RunGitHook(super::RunGitHook),
        #[prost(message, tag="400")]
        GitDeleteBranch(super::GitDeleteBranch),
        #[prost(message, tag="401")]
        ExternalExtensionAgentsUpdated(super::ExternalExtensionAgentsUpdated),
        #[prost(message, tag="402")]
        GitCreateRemote(super::GitCreateRemote),
        #[prost(message, tag="403")]
        GitRemoveRemote(super::GitRemoveRemote),
        #[prost(message, tag="404")]
        TrustWorktrees(super::TrustWorktrees),
        /// current max
        #[prost(message, tag="405")]
        RestrictWorktrees(super::RestrictWorktrees),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hello {
    #[prost(message, optional, tag="1")]
    pub peer_id: ::core::option::Option<PeerId>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ack {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
    #[prost(enumeration="ErrorCode", tag="2")]
    pub code: i32,
    #[prost(string, repeated, tag="3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndStream {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Test {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushBufferedMessages {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushBufferedMessagesResponse {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteStarted {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorCode {
    Internal = 0,
    NoSuchChannel = 1,
    Disconnected = 2,
    SignedOut = 3,
    UpgradeRequired = 4,
    Forbidden = 5,
    NeedsCla = 7,
    NotARootChannel = 8,
    BadPublicNesting = 9,
    CircularNesting = 10,
    WrongMoveTarget = 11,
    UnsharedItem = 12,
    NoSuchProject = 13,
    DevServerProjectPathDoesNotExist = 16,
    RemoteUpgradeRequired = 17,
    RateLimitExceeded = 18,
    CommitFailed = 19,
}
