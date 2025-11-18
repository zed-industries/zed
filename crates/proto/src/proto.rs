#![allow(non_snake_case)]

pub mod error;
mod macros;
mod typed_envelope;

pub use error::*;
pub use prost::{DecodeError, Message};
use std::{
    cmp,
    fmt::Debug,
    iter, mem,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
pub use typed_envelope::*;

include!(concat!(env!("OUT_DIR"), "/zed.messages.rs"));

pub const REMOTE_SERVER_PEER_ID: PeerId = PeerId { owner_id: 0, id: 0 };
pub const REMOTE_SERVER_PROJECT_ID: u64 = 0;

messages!(
    (Ack, Foreground),
    (AckBufferOperation, Background),
    (AckChannelMessage, Background),
    (ActivateToolchain, Foreground),
    (ActiveToolchain, Foreground),
    (ActiveToolchainResponse, Foreground),
    (ResolveToolchain, Background),
    (ResolveToolchainResponse, Background),
    (AddNotification, Foreground),
    (AddProjectCollaborator, Foreground),
    (AddWorktree, Foreground),
    (AddWorktreeResponse, Foreground),
    (AdvertiseContexts, Foreground),
    (ApplyCodeAction, Background),
    (ApplyCodeActionResponse, Background),
    (ApplyCompletionAdditionalEdits, Background),
    (ApplyCompletionAdditionalEditsResponse, Background),
    (BlameBuffer, Foreground),
    (BlameBufferResponse, Foreground),
    (BufferReloaded, Foreground),
    (BufferSaved, Foreground),
    (Call, Foreground),
    (CallCanceled, Foreground),
    (CancelCall, Foreground),
    (CancelLanguageServerWork, Foreground),
    (ChannelMessageSent, Foreground),
    (ChannelMessageUpdate, Foreground),
    (CloseBuffer, Foreground),
    (Commit, Background),
    (CopyProjectEntry, Foreground),
    (CreateBufferForPeer, Foreground),
    (CreateImageForPeer, Foreground),
    (CreateChannel, Foreground),
    (CreateChannelResponse, Foreground),
    (CreateContext, Foreground),
    (CreateContextResponse, Foreground),
    (CreateProjectEntry, Foreground),
    (CreateRoom, Foreground),
    (CreateRoomResponse, Foreground),
    (DeclineCall, Foreground),
    (DeleteChannel, Foreground),
    (DeleteNotification, Foreground),
    (DeleteProjectEntry, Foreground),
    (EndStream, Foreground),
    (Error, Foreground),
    (ExpandProjectEntry, Foreground),
    (ExpandProjectEntryResponse, Foreground),
    (FindSearchCandidatesResponse, Background),
    (FindSearchCandidates, Background),
    (FlushBufferedMessages, Foreground),
    (ExpandAllForProjectEntry, Foreground),
    (ExpandAllForProjectEntryResponse, Foreground),
    (Follow, Foreground),
    (FollowResponse, Foreground),
    (ApplyCodeActionKind, Foreground),
    (ApplyCodeActionKindResponse, Foreground),
    (FormatBuffers, Foreground),
    (FormatBuffersResponse, Foreground),
    (FuzzySearchUsers, Foreground),
    (GetChannelMembers, Foreground),
    (GetChannelMembersResponse, Foreground),
    (GetChannelMessages, Background),
    (GetChannelMessagesById, Background),
    (GetChannelMessagesResponse, Background),
    (GetCodeActions, Background),
    (GetCodeActionsResponse, Background),
    (GetCompletions, Background),
    (GetCompletionsResponse, Background),
    (GetDeclaration, Background),
    (GetDeclarationResponse, Background),
    (GetDefinition, Background),
    (GetDefinitionResponse, Background),
    (GetDocumentHighlights, Background),
    (GetDocumentHighlightsResponse, Background),
    (GetDocumentSymbols, Background),
    (GetDocumentSymbolsResponse, Background),
    (GetHover, Background),
    (GetHoverResponse, Background),
    (GetNotifications, Foreground),
    (GetNotificationsResponse, Foreground),
    (GetCrashFiles, Background),
    (GetCrashFilesResponse, Background),
    (GetPathMetadata, Background),
    (GetPathMetadataResponse, Background),
    (GetPermalinkToLine, Foreground),
    (GetProcesses, Background),
    (GetProcessesResponse, Background),
    (GetPermalinkToLineResponse, Foreground),
    (GetProjectSymbols, Background),
    (GetProjectSymbolsResponse, Background),
    (GetReferences, Background),
    (GetReferencesResponse, Background),
    (GetSignatureHelp, Background),
    (GetSignatureHelpResponse, Background),
    (GetSupermavenApiKey, Background),
    (GetSupermavenApiKeyResponse, Background),
    (GetTypeDefinition, Background),
    (GetTypeDefinitionResponse, Background),
    (GetImplementation, Background),
    (GetImplementationResponse, Background),
    (OpenUnstagedDiff, Foreground),
    (OpenUnstagedDiffResponse, Foreground),
    (OpenUncommittedDiff, Foreground),
    (OpenUncommittedDiffResponse, Foreground),
    (GetUsers, Foreground),
    (GitGetBranches, Background),
    (GitBranchesResponse, Background),
    (Hello, Foreground),
    (HideToast, Background),
    (IncomingCall, Foreground),
    (InlayHints, Background),
    (InlayHintsResponse, Background),
    (InstallExtension, Background),
    (InviteChannelMember, Foreground),
    (JoinChannel, Foreground),
    (JoinChannelBuffer, Foreground),
    (JoinChannelBufferResponse, Foreground),
    (JoinChannelChat, Foreground),
    (JoinChannelChatResponse, Foreground),
    (JoinProject, Foreground),
    (JoinProjectResponse, Foreground),
    (JoinRoom, Foreground),
    (JoinRoomResponse, Foreground),
    (LanguageServerLog, Foreground),
    (LanguageServerPromptRequest, Foreground),
    (LanguageServerPromptResponse, Foreground),
    (LeaveChannelBuffer, Background),
    (LeaveChannelChat, Foreground),
    (LeaveProject, Foreground),
    (LeaveRoom, Foreground),
    (LinkedEditingRange, Background),
    (LinkedEditingRangeResponse, Background),
    (ListRemoteDirectory, Background),
    (ListRemoteDirectoryResponse, Background),
    (ListToolchains, Foreground),
    (ListToolchainsResponse, Foreground),
    (LoadCommitDiff, Foreground),
    (LoadCommitDiffResponse, Foreground),
    (LspExtExpandMacro, Background),
    (LspExtExpandMacroResponse, Background),
    (LspExtOpenDocs, Background),
    (LspExtOpenDocsResponse, Background),
    (LspExtRunnables, Background),
    (LspExtRunnablesResponse, Background),
    (LspExtSwitchSourceHeader, Background),
    (LspExtSwitchSourceHeaderResponse, Background),
    (LspExtGoToParentModule, Background),
    (LspExtGoToParentModuleResponse, Background),
    (LspExtCancelFlycheck, Background),
    (LspExtRunFlycheck, Background),
    (LspExtClearFlycheck, Background),
    (MarkNotificationRead, Foreground),
    (MoveChannel, Foreground),
    (ReorderChannel, Foreground),
    (LspQuery, Background),
    (LspQueryResponse, Background),
    (OnTypeFormatting, Background),
    (OnTypeFormattingResponse, Background),
    (OpenBufferById, Background),
    (OpenBufferByPath, Background),
    (OpenImageByPath, Background),
    (OpenBufferForSymbol, Background),
    (OpenBufferForSymbolResponse, Background),
    (OpenBufferResponse, Background),
    (OpenImageResponse, Background),
    (OpenCommitMessageBuffer, Background),
    (OpenContext, Foreground),
    (OpenContextResponse, Foreground),
    (OpenNewBuffer, Foreground),
    (OpenServerSettings, Foreground),
    (PerformRename, Background),
    (PerformRenameResponse, Background),
    (Ping, Foreground),
    (PrepareRename, Background),
    (PrepareRenameResponse, Background),
    (ProjectEntryResponse, Foreground),
    (RefreshInlayHints, Foreground),
    (RegisterBufferWithLanguageServers, Background),
    (RejoinChannelBuffers, Foreground),
    (RejoinChannelBuffersResponse, Foreground),
    (RejoinRemoteProjects, Foreground),
    (RejoinRemoteProjectsResponse, Foreground),
    (RejoinRoom, Foreground),
    (RejoinRoomResponse, Foreground),
    (ReloadBuffers, Foreground),
    (ReloadBuffersResponse, Foreground),
    (RemoveChannelMember, Foreground),
    (RemoveChannelMessage, Foreground),
    (RemoveContact, Foreground),
    (RemoveProjectCollaborator, Foreground),
    (RemoveWorktree, Foreground),
    (RenameChannel, Foreground),
    (RenameChannelResponse, Foreground),
    (RenameProjectEntry, Foreground),
    (RequestContact, Foreground),
    (ResolveCompletionDocumentation, Background),
    (ResolveCompletionDocumentationResponse, Background),
    (ResolveInlayHint, Background),
    (ResolveInlayHintResponse, Background),
    (GetDocumentColor, Background),
    (GetDocumentColorResponse, Background),
    (GetColorPresentation, Background),
    (GetColorPresentationResponse, Background),
    (RefreshCodeLens, Background),
    (GetCodeLens, Background),
    (GetCodeLensResponse, Background),
    (RespondToChannelInvite, Foreground),
    (RespondToContactRequest, Foreground),
    (RestartLanguageServers, Foreground),
    (StopLanguageServers, Background),
    (RoomUpdated, Foreground),
    (SaveBuffer, Foreground),
    (SendChannelMessage, Background),
    (SendChannelMessageResponse, Background),
    (SetChannelMemberRole, Foreground),
    (SetChannelVisibility, Foreground),
    (SetRoomParticipantRole, Foreground),
    (ShareProject, Foreground),
    (ShareProjectResponse, Foreground),
    (ShowContacts, Foreground),
    (ShutdownRemoteServer, Foreground),
    (Stage, Background),
    (StartLanguageServer, Foreground),
    (SubscribeToChannels, Foreground),
    (SyncExtensions, Background),
    (SyncExtensionsResponse, Background),
    (BreakpointsForFile, Background),
    (ToggleBreakpoint, Foreground),
    (SynchronizeBuffers, Foreground),
    (SynchronizeBuffersResponse, Foreground),
    (SynchronizeContexts, Foreground),
    (SynchronizeContextsResponse, Foreground),
    (TaskContext, Background),
    (TaskContextForLocation, Background),
    (Test, Foreground),
    (Toast, Background),
    (Unfollow, Foreground),
    (UnshareProject, Foreground),
    (Unstage, Background),
    (Stash, Background),
    (StashPop, Background),
    (StashApply, Background),
    (StashDrop, Background),
    (UpdateBuffer, Foreground),
    (UpdateBufferFile, Foreground),
    (UpdateChannelBuffer, Foreground),
    (UpdateChannelBufferCollaborators, Foreground),
    (UpdateChannelMessage, Foreground),
    (UpdateChannels, Foreground),
    (UpdateContacts, Foreground),
    (UpdateContext, Foreground),
    (UpdateDiagnosticSummary, Foreground),
    (UpdateDiffBases, Foreground),
    (UpdateFollowers, Foreground),
    (UpdateGitBranch, Background),
    (UpdateInviteInfo, Foreground),
    (UpdateLanguageServer, Foreground),
    (UpdateNotification, Foreground),
    (UpdateParticipantLocation, Foreground),
    (UpdateProject, Foreground),
    (UpdateProjectCollaborator, Foreground),
    (UpdateUserChannels, Foreground),
    (UpdateWorktree, Foreground),
    (UpdateWorktreeSettings, Foreground),
    (UpdateUserSettings, Background),
    (UpdateRepository, Foreground),
    (RemoveRepository, Foreground),
    (UsersResponse, Foreground),
    (GitReset, Background),
    (GitCheckoutFiles, Background),
    (GitShow, Background),
    (GitCommitDetails, Background),
    (SetIndexText, Background),
    (Push, Background),
    (Fetch, Background),
    (GetRemotes, Background),
    (GetRemotesResponse, Background),
    (Pull, Background),
    (RemoteMessageResponse, Background),
    (AskPassRequest, Background),
    (AskPassResponse, Background),
    (GitCreateBranch, Background),
    (GitChangeBranch, Background),
    (GitRenameBranch, Background),
    (CheckForPushedCommits, Background),
    (CheckForPushedCommitsResponse, Background),
    (GitDiff, Background),
    (GitDiffResponse, Background),
    (GitInit, Background),
    (GetDebugAdapterBinary, Background),
    (DebugAdapterBinary, Background),
    (RunDebugLocators, Background),
    (DebugRequest, Background),
    (LogToDebugConsole, Background),
    (GetDocumentDiagnostics, Background),
    (GetDocumentDiagnosticsResponse, Background),
    (PullWorkspaceDiagnostics, Background),
    (GetDefaultBranch, Background),
    (GetDefaultBranchResponse, Background),
    (GetTreeDiff, Background),
    (GetTreeDiffResponse, Background),
    (GetBlobContent, Background),
    (GetBlobContentResponse, Background),
    (GitClone, Background),
    (GitCloneResponse, Background),
    (ToggleLspLogs, Background),
    (GetDirectoryEnvironment, Background),
    (DirectoryEnvironment, Background),
    (GetAgentServerCommand, Background),
    (AgentServerCommand, Background),
    (ExternalAgentsUpdated, Background),
    (ExternalExtensionAgentsUpdated, Background),
    (ExternalAgentLoadingStatusUpdated, Background),
    (NewExternalAgentVersionAvailable, Background),
    (RemoteStarted, Background),
    (GitGetWorktrees, Background),
    (GitWorktreesResponse, Background),
    (GitCreateWorktree, Background)
);

request_messages!(
    (ApplyCodeAction, ApplyCodeActionResponse),
    (
        ApplyCompletionAdditionalEdits,
        ApplyCompletionAdditionalEditsResponse
    ),
    (Call, Ack),
    (CancelCall, Ack),
    (Commit, Ack),
    (CopyProjectEntry, ProjectEntryResponse),
    (CreateChannel, CreateChannelResponse),
    (CreateProjectEntry, ProjectEntryResponse),
    (CreateRoom, CreateRoomResponse),
    (DeclineCall, Ack),
    (DeleteChannel, Ack),
    (DeleteProjectEntry, ProjectEntryResponse),
    (ExpandProjectEntry, ExpandProjectEntryResponse),
    (ExpandAllForProjectEntry, ExpandAllForProjectEntryResponse),
    (Follow, FollowResponse),
    (ApplyCodeActionKind, ApplyCodeActionKindResponse),
    (FormatBuffers, FormatBuffersResponse),
    (FuzzySearchUsers, UsersResponse),
    (GetChannelMembers, GetChannelMembersResponse),
    (GetChannelMessages, GetChannelMessagesResponse),
    (GetChannelMessagesById, GetChannelMessagesResponse),
    (GetCodeActions, GetCodeActionsResponse),
    (GetCompletions, GetCompletionsResponse),
    (GetDefinition, GetDefinitionResponse),
    (GetDeclaration, GetDeclarationResponse),
    (GetImplementation, GetImplementationResponse),
    (GetDocumentHighlights, GetDocumentHighlightsResponse),
    (GetDocumentSymbols, GetDocumentSymbolsResponse),
    (GetHover, GetHoverResponse),
    (GetNotifications, GetNotificationsResponse),
    (GetProjectSymbols, GetProjectSymbolsResponse),
    (GetReferences, GetReferencesResponse),
    (GetSignatureHelp, GetSignatureHelpResponse),
    (OpenUnstagedDiff, OpenUnstagedDiffResponse),
    (OpenUncommittedDiff, OpenUncommittedDiffResponse),
    (GetSupermavenApiKey, GetSupermavenApiKeyResponse),
    (GetTypeDefinition, GetTypeDefinitionResponse),
    (LinkedEditingRange, LinkedEditingRangeResponse),
    (ListRemoteDirectory, ListRemoteDirectoryResponse),
    (GetUsers, UsersResponse),
    (IncomingCall, Ack),
    (InlayHints, InlayHintsResponse),
    (GetCodeLens, GetCodeLensResponse),
    (InviteChannelMember, Ack),
    (JoinChannel, JoinRoomResponse),
    (JoinChannelBuffer, JoinChannelBufferResponse),
    (JoinChannelChat, JoinChannelChatResponse),
    (JoinProject, JoinProjectResponse),
    (JoinRoom, JoinRoomResponse),
    (LeaveChannelBuffer, Ack),
    (LeaveRoom, Ack),
    (LoadCommitDiff, LoadCommitDiffResponse),
    (MarkNotificationRead, Ack),
    (MoveChannel, Ack),
    (OnTypeFormatting, OnTypeFormattingResponse),
    (OpenBufferById, OpenBufferResponse),
    (OpenBufferByPath, OpenBufferResponse),
    (OpenImageByPath, OpenImageResponse),
    (OpenBufferForSymbol, OpenBufferForSymbolResponse),
    (OpenCommitMessageBuffer, OpenBufferResponse),
    (OpenNewBuffer, OpenBufferResponse),
    (PerformRename, PerformRenameResponse),
    (Ping, Ack),
    (PrepareRename, PrepareRenameResponse),
    (RefreshInlayHints, Ack),
    (RefreshCodeLens, Ack),
    (RejoinChannelBuffers, RejoinChannelBuffersResponse),
    (RejoinRoom, RejoinRoomResponse),
    (ReloadBuffers, ReloadBuffersResponse),
    (RemoveChannelMember, Ack),
    (RemoveChannelMessage, Ack),
    (UpdateChannelMessage, Ack),
    (RemoveContact, Ack),
    (RenameChannel, RenameChannelResponse),
    (RenameProjectEntry, ProjectEntryResponse),
    (ReorderChannel, Ack),
    (RequestContact, Ack),
    (
        ResolveCompletionDocumentation,
        ResolveCompletionDocumentationResponse
    ),
    (ResolveInlayHint, ResolveInlayHintResponse),
    (GetDocumentColor, GetDocumentColorResponse),
    (GetColorPresentation, GetColorPresentationResponse),
    (RespondToChannelInvite, Ack),
    (RespondToContactRequest, Ack),
    (SaveBuffer, BufferSaved),
    (Stage, Ack),
    (FindSearchCandidates, FindSearchCandidatesResponse),
    (SendChannelMessage, SendChannelMessageResponse),
    (SetChannelMemberRole, Ack),
    (SetChannelVisibility, Ack),
    (ShareProject, ShareProjectResponse),
    (SynchronizeBuffers, SynchronizeBuffersResponse),
    (TaskContextForLocation, TaskContext),
    (Test, Test),
    (Unstage, Ack),
    (Stash, Ack),
    (StashPop, Ack),
    (StashApply, Ack),
    (StashDrop, Ack),
    (UpdateBuffer, Ack),
    (UpdateParticipantLocation, Ack),
    (UpdateProject, Ack),
    (UpdateWorktree, Ack),
    (UpdateRepository, Ack),
    (RemoveRepository, Ack),
    (LspExtExpandMacro, LspExtExpandMacroResponse),
    (LspExtOpenDocs, LspExtOpenDocsResponse),
    (LspExtRunnables, LspExtRunnablesResponse),
    (SetRoomParticipantRole, Ack),
    (BlameBuffer, BlameBufferResponse),
    (RejoinRemoteProjects, RejoinRemoteProjectsResponse),
    (LspQuery, Ack),
    (LspQueryResponse, Ack),
    (RestartLanguageServers, Ack),
    (StopLanguageServers, Ack),
    (OpenContext, OpenContextResponse),
    (CreateContext, CreateContextResponse),
    (SynchronizeContexts, SynchronizeContextsResponse),
    (LspExtSwitchSourceHeader, LspExtSwitchSourceHeaderResponse),
    (LspExtGoToParentModule, LspExtGoToParentModuleResponse),
    (LspExtCancelFlycheck, Ack),
    (LspExtRunFlycheck, Ack),
    (LspExtClearFlycheck, Ack),
    (AddWorktree, AddWorktreeResponse),
    (ShutdownRemoteServer, Ack),
    (RemoveWorktree, Ack),
    (OpenServerSettings, OpenBufferResponse),
    (GetPermalinkToLine, GetPermalinkToLineResponse),
    (FlushBufferedMessages, Ack),
    (LanguageServerPromptRequest, LanguageServerPromptResponse),
    (GitGetBranches, GitBranchesResponse),
    (UpdateGitBranch, Ack),
    (ListToolchains, ListToolchainsResponse),
    (ActivateToolchain, Ack),
    (ActiveToolchain, ActiveToolchainResponse),
    (ResolveToolchain, ResolveToolchainResponse),
    (GetPathMetadata, GetPathMetadataResponse),
    (GetCrashFiles, GetCrashFilesResponse),
    (CancelLanguageServerWork, Ack),
    (SyncExtensions, SyncExtensionsResponse),
    (InstallExtension, Ack),
    (RegisterBufferWithLanguageServers, Ack),
    (GitShow, GitCommitDetails),
    (GitReset, Ack),
    (GitCheckoutFiles, Ack),
    (SetIndexText, Ack),
    (Push, RemoteMessageResponse),
    (Fetch, RemoteMessageResponse),
    (GetRemotes, GetRemotesResponse),
    (Pull, RemoteMessageResponse),
    (AskPassRequest, AskPassResponse),
    (GitCreateBranch, Ack),
    (GitChangeBranch, Ack),
    (GitRenameBranch, Ack),
    (CheckForPushedCommits, CheckForPushedCommitsResponse),
    (GitDiff, GitDiffResponse),
    (GitInit, Ack),
    (ToggleBreakpoint, Ack),
    (GetDebugAdapterBinary, DebugAdapterBinary),
    (RunDebugLocators, DebugRequest),
    (GetDocumentDiagnostics, GetDocumentDiagnosticsResponse),
    (PullWorkspaceDiagnostics, Ack),
    (GetDefaultBranch, GetDefaultBranchResponse),
    (GetBlobContent, GetBlobContentResponse),
    (GetTreeDiff, GetTreeDiffResponse),
    (GitClone, GitCloneResponse),
    (ToggleLspLogs, Ack),
    (GetDirectoryEnvironment, DirectoryEnvironment),
    (GetProcesses, GetProcessesResponse),
    (GetAgentServerCommand, AgentServerCommand),
    (RemoteStarted, Ack),
    (GitGetWorktrees, GitWorktreesResponse),
    (GitCreateWorktree, Ack)
);

lsp_messages!(
    (GetReferences, GetReferencesResponse, true),
    (GetDocumentColor, GetDocumentColorResponse, true),
    (GetHover, GetHoverResponse, true),
    (GetCodeActions, GetCodeActionsResponse, true),
    (GetSignatureHelp, GetSignatureHelpResponse, true),
    (GetCodeLens, GetCodeLensResponse, true),
    (GetDocumentDiagnostics, GetDocumentDiagnosticsResponse, true),
    (GetDefinition, GetDefinitionResponse, true),
    (GetDeclaration, GetDeclarationResponse, true),
    (GetTypeDefinition, GetTypeDefinitionResponse, true),
    (GetImplementation, GetImplementationResponse, true),
    (InlayHints, InlayHintsResponse, false),
);

entity_messages!(
    {project_id, ShareProject},
    AddProjectCollaborator,
    AddWorktree,
    ApplyCodeAction,
    ApplyCompletionAdditionalEdits,
    BlameBuffer,
    BufferReloaded,
    BufferSaved,
    CloseBuffer,
    Commit,
    GetColorPresentation,
    CopyProjectEntry,
    CreateBufferForPeer,
    CreateImageForPeer,
    CreateProjectEntry,
    GetDocumentColor,
    DeleteProjectEntry,
    ExpandProjectEntry,
    ExpandAllForProjectEntry,
    FindSearchCandidates,
    ApplyCodeActionKind,
    FormatBuffers,
    GetCodeActions,
    GetCodeLens,
    GetCompletions,
    GetDefinition,
    GetDeclaration,
    GetImplementation,
    GetDocumentHighlights,
    GetDocumentSymbols,
    GetHover,
    GetProjectSymbols,
    GetReferences,
    GetSignatureHelp,
    OpenUnstagedDiff,
    OpenUncommittedDiff,
    GetTypeDefinition,
    InlayHints,
    JoinProject,
    LeaveProject,
    LinkedEditingRange,
    LoadCommitDiff,
    LspQuery,
    LspQueryResponse,
    RestartLanguageServers,
    StopLanguageServers,
    OnTypeFormatting,
    OpenNewBuffer,
    OpenBufferById,
    OpenBufferByPath,
    OpenImageByPath,
    OpenBufferForSymbol,
    OpenCommitMessageBuffer,
    PerformRename,
    PrepareRename,
    RefreshInlayHints,
    RefreshCodeLens,
    ReloadBuffers,
    RemoveProjectCollaborator,
    RenameProjectEntry,
    ResolveCompletionDocumentation,
    ResolveInlayHint,
    SaveBuffer,
    Stage,
    StartLanguageServer,
    SynchronizeBuffers,
    TaskContextForLocation,
    UnshareProject,
    Unstage,
    Stash,
    StashPop,
    StashApply,
    StashDrop,
    UpdateBuffer,
    UpdateBufferFile,
    UpdateDiagnosticSummary,
    UpdateDiffBases,
    UpdateLanguageServer,
    UpdateProject,
    UpdateProjectCollaborator,
    UpdateWorktree,
    UpdateRepository,
    RemoveRepository,
    UpdateWorktreeSettings,
    UpdateUserSettings,
    LspExtExpandMacro,
    LspExtOpenDocs,
    LspExtRunnables,
    AdvertiseContexts,
    OpenContext,
    CreateContext,
    UpdateContext,
    SynchronizeContexts,
    LspExtSwitchSourceHeader,
    LspExtGoToParentModule,
    LspExtCancelFlycheck,
    LspExtRunFlycheck,
    LspExtClearFlycheck,
    LanguageServerLog,
    Toast,
    HideToast,
    OpenServerSettings,
    GetPermalinkToLine,
    LanguageServerPromptRequest,
    GitGetBranches,
    UpdateGitBranch,
    ListToolchains,
    ActivateToolchain,
    ActiveToolchain,
    ResolveToolchain,
    GetPathMetadata,
    GetProcesses,
    CancelLanguageServerWork,
    RegisterBufferWithLanguageServers,
    GitShow,
    GitReset,
    GitCheckoutFiles,
    SetIndexText,
    ToggleLspLogs,
    GetDirectoryEnvironment,

    Push,
    Fetch,
    GetRemotes,
    Pull,
    AskPassRequest,
    GitChangeBranch,
    GitRenameBranch,
    GitCreateBranch,
    CheckForPushedCommits,
    GitDiff,
    GitInit,
    BreakpointsForFile,
    ToggleBreakpoint,
    RunDebugLocators,
    GetDebugAdapterBinary,
    LogToDebugConsole,
    GetDocumentDiagnostics,
    PullWorkspaceDiagnostics,
    GetDefaultBranch,
    GetTreeDiff,
    GetBlobContent,
    GitClone,
    GetAgentServerCommand,
    ExternalAgentsUpdated,
    ExternalExtensionAgentsUpdated,
    ExternalAgentLoadingStatusUpdated,
    NewExternalAgentVersionAvailable,
    GitGetWorktrees,
    GitCreateWorktree
);

entity_messages!(
    {channel_id, Channel},
    ChannelMessageSent,
    ChannelMessageUpdate,
    RemoveChannelMessage,
    UpdateChannelMessage,
    UpdateChannelBuffer,
    UpdateChannelBufferCollaborators,
);

impl From<Timestamp> for SystemTime {
    fn from(val: Timestamp) -> Self {
        UNIX_EPOCH
            .checked_add(Duration::new(val.seconds, val.nanos))
            .unwrap()
    }
}

impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
        Self {
            seconds: duration.as_secs(),
            nanos: duration.subsec_nanos(),
        }
    }
}

impl From<u128> for Nonce {
    fn from(nonce: u128) -> Self {
        let upper_half = (nonce >> 64) as u64;
        let lower_half = nonce as u64;
        Self {
            upper_half,
            lower_half,
        }
    }
}

impl From<Nonce> for u128 {
    fn from(nonce: Nonce) -> Self {
        let upper_half = (nonce.upper_half as u128) << 64;
        let lower_half = nonce.lower_half as u128;
        upper_half | lower_half
    }
}

#[cfg(any(test, feature = "test-support"))]
pub const MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE: usize = 2;
#[cfg(not(any(test, feature = "test-support")))]
pub const MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE: usize = 256;

pub fn split_worktree_update(mut message: UpdateWorktree) -> impl Iterator<Item = UpdateWorktree> {
    let mut done = false;

    iter::from_fn(move || {
        if done {
            return None;
        }

        let updated_entries_chunk_size = cmp::min(
            message.updated_entries.len(),
            MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE,
        );
        let updated_entries: Vec<_> = message
            .updated_entries
            .drain(..updated_entries_chunk_size)
            .collect();

        let removed_entries_chunk_size = cmp::min(
            message.removed_entries.len(),
            MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE,
        );
        let removed_entries = message
            .removed_entries
            .drain(..removed_entries_chunk_size)
            .collect();

        let mut updated_repositories = Vec::new();
        let mut limit = MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE;
        while let Some(repo) = message.updated_repositories.first_mut() {
            let updated_statuses_limit = cmp::min(repo.updated_statuses.len(), limit);
            let removed_statuses_limit = cmp::min(repo.removed_statuses.len(), limit);

            updated_repositories.push(RepositoryEntry {
                repository_id: repo.repository_id,
                branch_summary: repo.branch_summary.clone(),
                updated_statuses: repo
                    .updated_statuses
                    .drain(..updated_statuses_limit)
                    .collect(),
                removed_statuses: repo
                    .removed_statuses
                    .drain(..removed_statuses_limit)
                    .collect(),
                current_merge_conflicts: repo.current_merge_conflicts.clone(),
            });
            if repo.removed_statuses.is_empty() && repo.updated_statuses.is_empty() {
                message.updated_repositories.remove(0);
            }
            limit = limit.saturating_sub(removed_statuses_limit + updated_statuses_limit);
            if limit == 0 {
                break;
            }
        }

        done = message.updated_entries.is_empty()
            && message.removed_entries.is_empty()
            && message.updated_repositories.is_empty();

        let removed_repositories = if done {
            mem::take(&mut message.removed_repositories)
        } else {
            Default::default()
        };

        Some(UpdateWorktree {
            project_id: message.project_id,
            worktree_id: message.worktree_id,
            root_name: message.root_name.clone(),
            abs_path: message.abs_path.clone(),
            updated_entries,
            removed_entries,
            scan_id: message.scan_id,
            is_last_update: done && message.is_last_update,
            updated_repositories,
            removed_repositories,
        })
    })
}

pub fn split_repository_update(
    mut update: UpdateRepository,
) -> impl Iterator<Item = UpdateRepository> {
    let mut updated_statuses_iter = mem::take(&mut update.updated_statuses).into_iter().fuse();
    let mut removed_statuses_iter = mem::take(&mut update.removed_statuses).into_iter().fuse();
    std::iter::from_fn({
        let update = update.clone();
        move || {
            let updated_statuses = updated_statuses_iter
                .by_ref()
                .take(MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE)
                .collect::<Vec<_>>();
            let removed_statuses = removed_statuses_iter
                .by_ref()
                .take(MAX_WORKTREE_UPDATE_MAX_CHUNK_SIZE)
                .collect::<Vec<_>>();
            if updated_statuses.is_empty() && removed_statuses.is_empty() {
                return None;
            }
            Some(UpdateRepository {
                updated_statuses,
                removed_statuses,
                is_last_update: false,
                ..update.clone()
            })
        }
    })
    .chain([UpdateRepository {
        updated_statuses: Vec::new(),
        removed_statuses: Vec::new(),
        is_last_update: true,
        ..update
    }])
}

impl LspQuery {
    pub fn query_name_and_write_permissions(&self) -> (&str, bool) {
        match self.request {
            Some(lsp_query::Request::GetHover(_)) => ("GetHover", false),
            Some(lsp_query::Request::GetCodeActions(_)) => ("GetCodeActions", true),
            Some(lsp_query::Request::GetSignatureHelp(_)) => ("GetSignatureHelp", false),
            Some(lsp_query::Request::GetCodeLens(_)) => ("GetCodeLens", true),
            Some(lsp_query::Request::GetDocumentDiagnostics(_)) => {
                ("GetDocumentDiagnostics", false)
            }
            Some(lsp_query::Request::GetDefinition(_)) => ("GetDefinition", false),
            Some(lsp_query::Request::GetDeclaration(_)) => ("GetDeclaration", false),
            Some(lsp_query::Request::GetTypeDefinition(_)) => ("GetTypeDefinition", false),
            Some(lsp_query::Request::GetImplementation(_)) => ("GetImplementation", false),
            Some(lsp_query::Request::GetReferences(_)) => ("GetReferences", false),
            Some(lsp_query::Request::GetDocumentColor(_)) => ("GetDocumentColor", false),
            Some(lsp_query::Request::InlayHints(_)) => ("InlayHints", false),
            None => ("<unknown>", true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converting_peer_id_from_and_to_u64() {
        let peer_id = PeerId {
            owner_id: 10,
            id: 3,
        };
        assert_eq!(PeerId::from_u64(peer_id.as_u64()), peer_id);
        let peer_id = PeerId {
            owner_id: u32::MAX,
            id: 3,
        };
        assert_eq!(PeerId::from_u64(peer_id.as_u64()), peer_id);
        let peer_id = PeerId {
            owner_id: 10,
            id: u32::MAX,
        };
        assert_eq!(PeerId::from_u64(peer_id.as_u64()), peer_id);
        let peer_id = PeerId {
            owner_id: u32::MAX,
            id: u32::MAX,
        };
        assert_eq!(PeerId::from_u64(peer_id.as_u64()), peer_id);
    }
}
