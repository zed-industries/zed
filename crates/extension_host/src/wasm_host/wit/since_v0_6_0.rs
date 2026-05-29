use crate::wasm_host::WasmState;
use anyhow::Result;
use extension::{KeyValueStoreDelegate, ProjectDelegate, WorktreeDelegate};
use gpui::BackgroundExecutor;
use semver::Version;
use std::sync::{Arc, OnceLock};
use wasmtime::component::{Linker, Resource};

use super::latest;

pub const MIN_VERSION: Version = Version::new(0, 6, 0);
pub const MAX_VERSION: Version = Version::new(0, 7, 0);

wasmtime::component::bindgen!({
    imports: {
        default: async | trappable,
    },
    exports: {
        default: async,
    },
    path: "../extension_api/wit/since_v0.6.0",
    with: {
        "worktree": ExtensionWorktree,
        "project": ExtensionProject,
        "key-value-store": ExtensionKeyValueStore,
        "zed:extension/common": latest::zed::extension::common,
        "zed:extension/http-client": latest::zed::extension::http_client,
        "zed:extension/nodejs": latest::zed::extension::nodejs,
        "zed:extension/platform": latest::zed::extension::platform,
        "zed:extension/process": latest::zed::extension::process,
        "zed:extension/slash-command": latest::zed::extension::slash_command,
        "zed:extension/context-server": latest::zed::extension::context_server,
    },
});

pub use self::zed::extension::*;

mod settings {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/since_v0.6.0/settings.rs"));
}

pub type ExtensionWorktree = Arc<dyn WorktreeDelegate>;
pub type ExtensionProject = Arc<dyn ProjectDelegate>;
pub type ExtensionKeyValueStore = Arc<dyn KeyValueStoreDelegate>;

pub fn linker(executor: &BackgroundExecutor) -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| {
        super::new_linker(executor, |linker| {
            Extension::add_to_linker::<_, WasmState>(linker, |s| s)
        })
    })
}

impl From<CodeLabel> for latest::CodeLabel {
    fn from(value: CodeLabel) -> Self {
        Self {
            code: value.code,
            spans: value.spans.into_iter().map(Into::into).collect(),
            filter_range: value.filter_range,
        }
    }
}

impl From<CodeLabelSpan> for latest::CodeLabelSpan {
    fn from(value: CodeLabelSpan) -> Self {
        match value {
            CodeLabelSpan::CodeRange(range) => Self::CodeRange(range),
            CodeLabelSpan::Literal(literal) => Self::Literal(literal.into()),
        }
    }
}

impl From<CodeLabelSpanLiteral> for latest::CodeLabelSpanLiteral {
    fn from(value: CodeLabelSpanLiteral) -> Self {
        Self {
            text: value.text,
            highlight_name: value.highlight_name,
        }
    }
}

impl From<SettingsLocation> for latest::SettingsLocation {
    fn from(value: SettingsLocation) -> Self {
        Self {
            worktree_id: value.worktree_id,
            path: value.path,
        }
    }
}

impl From<LanguageServerInstallationStatus> for latest::LanguageServerInstallationStatus {
    fn from(value: LanguageServerInstallationStatus) -> Self {
        match value {
            LanguageServerInstallationStatus::None => Self::None,
            LanguageServerInstallationStatus::Downloading => Self::Downloading,
            LanguageServerInstallationStatus::CheckingForUpdate => Self::CheckingForUpdate,
            LanguageServerInstallationStatus::Failed(message) => Self::Failed(message),
        }
    }
}

impl From<DownloadedFileType> for latest::DownloadedFileType {
    fn from(value: DownloadedFileType) -> Self {
        match value {
            DownloadedFileType::Gzip => Self::Gzip,
            DownloadedFileType::GzipTar => Self::GzipTar,
            DownloadedFileType::Zip => Self::Zip,
            DownloadedFileType::Uncompressed => Self::Uncompressed,
        }
    }
}

impl From<latest::github::GithubReleaseAsset> for github::GithubReleaseAsset {
    fn from(value: latest::github::GithubReleaseAsset) -> Self {
        Self {
            name: value.name,
            download_url: value.download_url,
        }
    }
}

impl From<latest::github::GithubRelease> for github::GithubRelease {
    fn from(value: latest::github::GithubRelease) -> Self {
        Self {
            version: value.version,
            assets: value.assets.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<github::GithubReleaseOptions> for latest::github::GithubReleaseOptions {
    fn from(value: github::GithubReleaseOptions) -> Self {
        Self {
            require_assets: value.require_assets,
            pre_release: value.pre_release,
        }
    }
}

impl zed::extension::github::Host for WasmState {
    async fn github_release_by_tag_name(
        &mut self,
        repo: String,
        tag: String,
    ) -> wasmtime::Result<Result<github::GithubRelease, String>> {
        latest::github::Host::github_release_by_tag_name(self, repo, tag)
            .await
            .map(|result| result.map(Into::into))
    }

    async fn latest_github_release(
        &mut self,
        repo: String,
        options: github::GithubReleaseOptions,
    ) -> wasmtime::Result<Result<github::GithubRelease, String>> {
        latest::github::Host::latest_github_release(self, repo, options.into())
            .await
            .map(|result| result.map(Into::into))
    }
}

impl From<latest::lsp::Completion> for lsp::Completion {
    fn from(value: latest::lsp::Completion) -> Self {
        Self {
            label: value.label,
            label_details: value.label_details.map(Into::into),
            detail: value.detail,
            kind: value.kind.map(Into::into),
            insert_text_format: value.insert_text_format.map(Into::into),
        }
    }
}

impl From<latest::lsp::Symbol> for lsp::Symbol {
    fn from(value: latest::lsp::Symbol) -> Self {
        Self {
            name: value.name,
            kind: value.kind.into(),
        }
    }
}

impl From<latest::lsp::CompletionLabelDetails> for lsp::CompletionLabelDetails {
    fn from(value: latest::lsp::CompletionLabelDetails) -> Self {
        Self {
            detail: value.detail,
            description: value.description,
        }
    }
}

impl From<latest::lsp::CompletionKind> for lsp::CompletionKind {
    fn from(value: latest::lsp::CompletionKind) -> Self {
        match value {
            latest::lsp::CompletionKind::Text => Self::Text,
            latest::lsp::CompletionKind::Method => Self::Method,
            latest::lsp::CompletionKind::Function => Self::Function,
            latest::lsp::CompletionKind::Constructor => Self::Constructor,
            latest::lsp::CompletionKind::Field => Self::Field,
            latest::lsp::CompletionKind::Variable => Self::Variable,
            latest::lsp::CompletionKind::Class => Self::Class,
            latest::lsp::CompletionKind::Interface => Self::Interface,
            latest::lsp::CompletionKind::Module => Self::Module,
            latest::lsp::CompletionKind::Property => Self::Property,
            latest::lsp::CompletionKind::Unit => Self::Unit,
            latest::lsp::CompletionKind::Value => Self::Value,
            latest::lsp::CompletionKind::Enum => Self::Enum,
            latest::lsp::CompletionKind::Keyword => Self::Keyword,
            latest::lsp::CompletionKind::Snippet => Self::Snippet,
            latest::lsp::CompletionKind::Color => Self::Color,
            latest::lsp::CompletionKind::File => Self::File,
            latest::lsp::CompletionKind::Reference => Self::Reference,
            latest::lsp::CompletionKind::Folder => Self::Folder,
            latest::lsp::CompletionKind::EnumMember => Self::EnumMember,
            latest::lsp::CompletionKind::Constant => Self::Constant,
            latest::lsp::CompletionKind::Struct => Self::Struct,
            latest::lsp::CompletionKind::Event => Self::Event,
            latest::lsp::CompletionKind::Operator => Self::Operator,
            latest::lsp::CompletionKind::TypeParameter => Self::TypeParameter,
            latest::lsp::CompletionKind::Other(kind) => Self::Other(kind),
        }
    }
}

impl From<latest::lsp::InsertTextFormat> for lsp::InsertTextFormat {
    fn from(value: latest::lsp::InsertTextFormat) -> Self {
        match value {
            latest::lsp::InsertTextFormat::PlainText => Self::PlainText,
            latest::lsp::InsertTextFormat::Snippet => Self::Snippet,
            latest::lsp::InsertTextFormat::Other(value) => Self::Other(value),
        }
    }
}

impl From<latest::lsp::SymbolKind> for lsp::SymbolKind {
    fn from(value: latest::lsp::SymbolKind) -> Self {
        match value {
            latest::lsp::SymbolKind::File => Self::File,
            latest::lsp::SymbolKind::Module => Self::Module,
            latest::lsp::SymbolKind::Namespace => Self::Namespace,
            latest::lsp::SymbolKind::Package => Self::Package,
            latest::lsp::SymbolKind::Class => Self::Class,
            latest::lsp::SymbolKind::Method => Self::Method,
            latest::lsp::SymbolKind::Property => Self::Property,
            latest::lsp::SymbolKind::Field => Self::Field,
            latest::lsp::SymbolKind::Constructor => Self::Constructor,
            latest::lsp::SymbolKind::Enum => Self::Enum,
            latest::lsp::SymbolKind::Interface => Self::Interface,
            latest::lsp::SymbolKind::Function => Self::Function,
            latest::lsp::SymbolKind::Variable => Self::Variable,
            latest::lsp::SymbolKind::Constant => Self::Constant,
            latest::lsp::SymbolKind::String => Self::String,
            latest::lsp::SymbolKind::Number => Self::Number,
            latest::lsp::SymbolKind::Boolean => Self::Boolean,
            latest::lsp::SymbolKind::Array => Self::Array,
            latest::lsp::SymbolKind::Object => Self::Object,
            latest::lsp::SymbolKind::Key => Self::Key,
            latest::lsp::SymbolKind::Null => Self::Null,
            latest::lsp::SymbolKind::EnumMember => Self::EnumMember,
            latest::lsp::SymbolKind::Struct => Self::Struct,
            latest::lsp::SymbolKind::Event => Self::Event,
            latest::lsp::SymbolKind::Operator => Self::Operator,
            latest::lsp::SymbolKind::TypeParameter => Self::TypeParameter,
            latest::lsp::SymbolKind::Other(kind) => Self::Other(kind),
        }
    }
}

impl lsp::Host for WasmState {}

impl HostKeyValueStore for WasmState {
    async fn insert(
        &mut self,
        kv_store: Resource<ExtensionKeyValueStore>,
        key: String,
        value: String,
    ) -> wasmtime::Result<Result<(), String>> {
        latest::HostKeyValueStore::insert(self, kv_store, key, value).await
    }

    async fn drop(&mut self, _worktree: Resource<ExtensionKeyValueStore>) -> Result<()> {
        // We only ever hand out borrows of key-value stores.
        Ok(())
    }
}

impl HostProject for WasmState {
    async fn worktree_ids(
        &mut self,
        project: Resource<ExtensionProject>,
    ) -> wasmtime::Result<Vec<u64>> {
        latest::HostProject::worktree_ids(self, project).await
    }

    async fn drop(&mut self, _project: Resource<Project>) -> Result<()> {
        // We only ever hand out borrows of projects.
        Ok(())
    }
}

impl HostWorktree for WasmState {
    async fn id(&mut self, delegate: Resource<Arc<dyn WorktreeDelegate>>) -> wasmtime::Result<u64> {
        latest::HostWorktree::id(self, delegate).await
    }

    async fn root_path(
        &mut self,
        delegate: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<String> {
        latest::HostWorktree::root_path(self, delegate).await
    }

    async fn read_text_file(
        &mut self,
        delegate: Resource<Arc<dyn WorktreeDelegate>>,
        path: String,
    ) -> wasmtime::Result<Result<String, String>> {
        latest::HostWorktree::read_text_file(self, delegate, path).await
    }

    async fn shell_env(
        &mut self,
        delegate: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<EnvVars> {
        latest::HostWorktree::shell_env(self, delegate).await
    }

    async fn which(
        &mut self,
        delegate: Resource<Arc<dyn WorktreeDelegate>>,
        binary_name: String,
    ) -> wasmtime::Result<Option<String>> {
        latest::HostWorktree::which(self, delegate, binary_name).await
    }

    async fn drop(&mut self, _worktree: Resource<Worktree>) -> Result<()> {
        // We only ever hand out borrows of worktrees.
        Ok(())
    }
}

impl ExtensionImports for WasmState {
    async fn get_settings(
        &mut self,
        location: Option<self::SettingsLocation>,
        category: String,
        key: Option<String>,
    ) -> wasmtime::Result<Result<String, String>> {
        latest::ExtensionImports::get_settings(
            self,
            location.map(|location| location.into()),
            category,
            key,
        )
        .await
    }

    async fn set_language_server_installation_status(
        &mut self,
        server_name: String,
        status: LanguageServerInstallationStatus,
    ) -> wasmtime::Result<()> {
        latest::ExtensionImports::set_language_server_installation_status(
            self,
            server_name,
            status.into(),
        )
        .await
    }

    async fn download_file(
        &mut self,
        url: String,
        path: String,
        file_type: DownloadedFileType,
    ) -> wasmtime::Result<Result<(), String>> {
        latest::ExtensionImports::download_file(self, url, path, file_type.into()).await
    }

    async fn make_file_executable(&mut self, path: String) -> wasmtime::Result<Result<(), String>> {
        latest::ExtensionImports::make_file_executable(self, path).await
    }
}

impl From<dap::TcpArguments> for latest::dap::TcpArguments {
    fn from(value: dap::TcpArguments) -> Self {
        let [a, b, c, d] = std::net::Ipv4Addr::from_bits(value.host).octets();
        Self {
            host: latest::dap::IpAddress::Ipv4((a, b, c, d)),
            port: value.port,
            timeout: value.timeout,
        }
    }
}

impl TryFrom<latest::dap::TcpArguments> for dap::TcpArguments {
    type Error = anyhow::Error;

    fn try_from(value: latest::dap::TcpArguments) -> Result<Self> {
        let host = match value.host {
            latest::dap::IpAddress::Ipv4((a, b, c, d)) => {
                std::net::Ipv4Addr::new(a, b, c, d).to_bits()
            }
            latest::dap::IpAddress::Ipv6((a, b, c, d, e, f, g, h)) => {
                let addr = std::net::Ipv6Addr::new(a, b, c, d, e, f, g, h);
                anyhow::bail!(
                    "DAP returned IPv6 host {addr}, which the v0.6.0 extension API cannot represent; the extension must be updated to v0.8.0 or later"
                );
            }
        };
        Ok(Self {
            host,
            port: value.port,
            timeout: value.timeout,
        })
    }
}

impl From<dap::TcpArgumentsTemplate> for latest::dap::TcpArgumentsTemplate {
    fn from(value: dap::TcpArgumentsTemplate) -> Self {
        Self {
            host: value.host.map(|host| {
                let [a, b, c, d] = std::net::Ipv4Addr::from_bits(host).octets();
                latest::dap::IpAddress::Ipv4((a, b, c, d))
            }),
            port: value.port,
            timeout: value.timeout,
        }
    }
}

impl From<latest::dap::TcpArgumentsTemplate> for dap::TcpArgumentsTemplate {
    fn from(value: latest::dap::TcpArgumentsTemplate) -> Self {
        Self {
            host: value.host.and_then(|host| match host {
                latest::dap::IpAddress::Ipv4((a, b, c, d)) => {
                    Some(std::net::Ipv4Addr::new(a, b, c, d).to_bits())
                }
                latest::dap::IpAddress::Ipv6((a, b, c, d, e, f, g, h)) => {
                    let addr = std::net::Ipv6Addr::new(a, b, c, d, e, f, g, h);
                    log::warn!(
                        "Dropping IPv6 host {addr} when handing TCP arguments back to a v0.6.0 extension; update the extension to v0.8.0 or later for IPv6 support"
                    );
                    None
                }
            }),
            port: value.port,
            timeout: value.timeout,
        }
    }
}

impl From<dap::LaunchRequest> for latest::dap::LaunchRequest {
    fn from(value: dap::LaunchRequest) -> Self {
        Self {
            program: value.program,
            cwd: value.cwd,
            args: value.args,
            envs: value.envs,
        }
    }
}

impl From<latest::dap::LaunchRequest> for dap::LaunchRequest {
    fn from(value: latest::dap::LaunchRequest) -> Self {
        Self {
            program: value.program,
            cwd: value.cwd,
            args: value.args,
            envs: value.envs,
        }
    }
}

impl From<dap::AttachRequest> for latest::dap::AttachRequest {
    fn from(value: dap::AttachRequest) -> Self {
        Self {
            process_id: value.process_id,
        }
    }
}

impl From<latest::dap::AttachRequest> for dap::AttachRequest {
    fn from(value: latest::dap::AttachRequest) -> Self {
        Self {
            process_id: value.process_id,
        }
    }
}

impl From<DebugRequest> for latest::DebugRequest {
    fn from(value: DebugRequest) -> Self {
        match value {
            DebugRequest::Launch(req) => Self::Launch(req.into()),
            DebugRequest::Attach(req) => Self::Attach(req.into()),
        }
    }
}

impl From<latest::DebugRequest> for DebugRequest {
    fn from(value: latest::DebugRequest) -> Self {
        match value {
            latest::DebugRequest::Launch(req) => Self::Launch(req.into()),
            latest::DebugRequest::Attach(req) => Self::Attach(req.into()),
        }
    }
}

impl From<DebugConfig> for latest::DebugConfig {
    fn from(value: DebugConfig) -> Self {
        Self {
            label: value.label,
            adapter: value.adapter,
            request: value.request.into(),
            stop_on_entry: value.stop_on_entry,
        }
    }
}

impl From<latest::DebugConfig> for DebugConfig {
    fn from(value: latest::DebugConfig) -> Self {
        Self {
            label: value.label,
            adapter: value.adapter,
            request: value.request.into(),
            stop_on_entry: value.stop_on_entry,
        }
    }
}

impl From<dap::TaskTemplate> for latest::dap::TaskTemplate {
    fn from(value: dap::TaskTemplate) -> Self {
        Self {
            label: value.label,
            command: value.command,
            args: value.args,
            env: value.env,
            cwd: value.cwd,
        }
    }
}

impl From<latest::dap::TaskTemplate> for dap::TaskTemplate {
    fn from(value: latest::dap::TaskTemplate) -> Self {
        Self {
            label: value.label,
            command: value.command,
            args: value.args,
            env: value.env,
            cwd: value.cwd,
        }
    }
}

impl From<dap::BuildTaskDefinition> for latest::dap::BuildTaskDefinition {
    fn from(value: dap::BuildTaskDefinition) -> Self {
        match value {
            dap::BuildTaskDefinition::ByName(name) => Self::ByName(name),
            dap::BuildTaskDefinition::Template(payload) => {
                Self::Template(latest::dap::BuildTaskDefinitionTemplatePayload {
                    locator_name: payload.locator_name,
                    template: payload.template.into(),
                })
            }
        }
    }
}

impl From<latest::dap::BuildTaskDefinition> for dap::BuildTaskDefinition {
    fn from(value: latest::dap::BuildTaskDefinition) -> Self {
        match value {
            latest::dap::BuildTaskDefinition::ByName(name) => Self::ByName(name),
            latest::dap::BuildTaskDefinition::Template(payload) => {
                Self::Template(dap::BuildTaskDefinitionTemplatePayload {
                    locator_name: payload.locator_name,
                    template: payload.template.into(),
                })
            }
        }
    }
}

impl From<DebugScenario> for latest::DebugScenario {
    fn from(value: DebugScenario) -> Self {
        Self {
            label: value.label,
            adapter: value.adapter,
            build: value.build.map(Into::into),
            config: value.config,
            tcp_connection: value.tcp_connection.map(Into::into),
        }
    }
}

impl From<latest::DebugScenario> for DebugScenario {
    fn from(value: latest::DebugScenario) -> Self {
        Self {
            label: value.label,
            adapter: value.adapter,
            build: value.build.map(Into::into),
            config: value.config,
            tcp_connection: value.tcp_connection.map(Into::into),
        }
    }
}

impl From<DebugTaskDefinition> for latest::DebugTaskDefinition {
    fn from(value: DebugTaskDefinition) -> Self {
        Self {
            label: value.label,
            adapter: value.adapter,
            config: value.config,
            tcp_connection: value.tcp_connection.map(Into::into),
        }
    }
}

impl From<latest::DebugTaskDefinition> for DebugTaskDefinition {
    fn from(value: latest::DebugTaskDefinition) -> Self {
        Self {
            label: value.label,
            adapter: value.adapter,
            config: value.config,
            tcp_connection: value.tcp_connection.map(Into::into),
        }
    }
}

impl From<dap::StartDebuggingRequestArgumentsRequest>
    for latest::dap::StartDebuggingRequestArgumentsRequest
{
    fn from(value: dap::StartDebuggingRequestArgumentsRequest) -> Self {
        match value {
            dap::StartDebuggingRequestArgumentsRequest::Launch => Self::Launch,
            dap::StartDebuggingRequestArgumentsRequest::Attach => Self::Attach,
        }
    }
}

impl From<latest::dap::StartDebuggingRequestArgumentsRequest>
    for dap::StartDebuggingRequestArgumentsRequest
{
    fn from(value: latest::dap::StartDebuggingRequestArgumentsRequest) -> Self {
        match value {
            latest::dap::StartDebuggingRequestArgumentsRequest::Launch => Self::Launch,
            latest::dap::StartDebuggingRequestArgumentsRequest::Attach => Self::Attach,
        }
    }
}

impl From<dap::StartDebuggingRequestArguments> for latest::dap::StartDebuggingRequestArguments {
    fn from(value: dap::StartDebuggingRequestArguments) -> Self {
        Self {
            configuration: value.configuration,
            request: value.request.into(),
        }
    }
}

impl From<latest::dap::StartDebuggingRequestArguments> for dap::StartDebuggingRequestArguments {
    fn from(value: latest::dap::StartDebuggingRequestArguments) -> Self {
        Self {
            configuration: value.configuration,
            request: value.request.into(),
        }
    }
}

impl From<DebugAdapterBinary> for latest::DebugAdapterBinary {
    fn from(value: DebugAdapterBinary) -> Self {
        Self {
            command: value.command,
            arguments: value.arguments,
            envs: value.envs,
            cwd: value.cwd,
            connection: value.connection.map(Into::into),
            request_args: value.request_args.into(),
        }
    }
}

impl TryFrom<latest::DebugAdapterBinary> for DebugAdapterBinary {
    type Error = anyhow::Error;

    fn try_from(value: latest::DebugAdapterBinary) -> Result<Self> {
        Ok(Self {
            command: value.command,
            arguments: value.arguments,
            envs: value.envs,
            cwd: value.cwd,
            connection: value.connection.map(TryInto::try_into).transpose()?,
            request_args: value.request_args.into(),
        })
    }
}

impl zed::extension::dap::Host for WasmState {
    async fn resolve_tcp_template(
        &mut self,
        template: dap::TcpArgumentsTemplate,
    ) -> wasmtime::Result<Result<dap::TcpArguments, String>> {
        let result = latest::dap::Host::resolve_tcp_template(self, template.into()).await?;
        Ok(
            result
                .and_then(|args| dap::TcpArguments::try_from(args).map_err(|err| err.to_string())),
        )
    }
}
