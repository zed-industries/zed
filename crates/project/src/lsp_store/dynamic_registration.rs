//! Handling of `client/registerCapability` and `client/unregisterCapability` requests,
//! which let language servers toggle their capabilities at runtime.
//!
//! Multiple registrations of the same capability may coexist: the most recent one is
//! "active" and mirrored into the server's [`lsp::ServerCapabilities`], while the rest
//! (including the statically declared capability, if any) are kept to be restored when
//! the active registration is unregistered.

use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet, IndexMap};
use gpui::{Context, SharedString};
use lsp::{
    DiagnosticServerCapabilities, LanguageServer, LanguageServerId, OneOf,
    TextDocumentSyncSaveOptions,
};

use crate::lsp_store::{
    LanguageServerState, LspStore, RenamePathsWatchedForServer,
    completion_trigger_characters_for_buffer, lsp_workspace_diagnostics_refresh,
};

#[derive(Debug)]
pub(super) enum RegistrationSource {
    Static,
    Dynamic(String),
}

impl RegistrationSource {
    pub(super) fn registration_id(&self) -> Option<&str> {
        match self {
            RegistrationSource::Static => None,
            RegistrationSource::Dynamic(id) => Some(id),
        }
    }
}

pub(super) type CapabilityRegistrations<T> = Vec<(RegistrationSource, T)>;

#[derive(Debug, Clone, Copy)]
struct CapabilityRegistrationChange {
    active_capability_changed: bool,
    text_document_registration_changed: bool,
    registration_changed: bool,
}

impl CapabilityRegistrationChange {
    fn applicability_may_have_changed(self) -> bool {
        self.active_capability_changed || self.text_document_registration_changed
    }

    fn ordered_capability_may_have_changed(self) -> bool {
        self.active_capability_changed || self.registration_changed
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub(super) struct DynamicTextDocumentRegistration {
    pub(super) document_selector: Option<lsp::DocumentSelector>,
    pub(super) server_capabilities: lsp::ServerCapabilities,
}

pub(super) type TextDocumentRegistrations =
    HashMap<String, IndexMap<String, DynamicTextDocumentRegistration>>;

#[derive(Default, Debug)]
pub(super) struct DynamicRegistrations {
    pub(super) did_change_watched_files: HashSet<String>,
    pub(super) text_documents: TextDocumentRegistrations,
    workspace_folders: CapabilityRegistrations<lsp::WorkspaceFoldersServerCapabilities>,
    workspace_symbol: CapabilityRegistrations<OneOf<bool, lsp::WorkspaceSymbolOptions>>,
    file_operations: CapabilityRegistrations<lsp::WorkspaceFileOperationsServerCapabilities>,
    execute_command: CapabilityRegistrations<lsp::ExecuteCommandOptions>,
}

impl LspStore {
    fn register_dynamic_text_document_capability<T: Clone + PartialEq>(
        &mut self,
        server: &LanguageServer,
        method: &str,
        registration_id: String,
        options: T,
        document_selector: Option<lsp::DocumentSelector>,
        cx: &mut Context<Self>,
        capability_of: impl Fn(&mut lsp::ServerCapabilities) -> &mut Option<T>,
    ) -> anyhow::Result<CapabilityRegistrationChange> {
        let server_id = server.server_id();
        let local = self
            .as_local_mut()
            .context("Expected LSP Store to be local")?;
        let mut server_capabilities = lsp::ServerCapabilities::default();
        *capability_of(&mut server_capabilities) = Some(options);
        let new_registration = DynamicTextDocumentRegistration {
            document_selector,
            server_capabilities,
        };
        let registrations = local
            .language_server_dynamic_registrations
            .entry(server_id)
            .or_default()
            .text_documents
            .entry(method.to_owned())
            .or_default();
        // Guests track registrations by id, so notify on any id-visible change; caches only
        // consume the (selector, options) value set, so refresh flags compare sets.
        let registration_changed = registrations.get(&registration_id) != Some(&new_registration);
        let applicable_set_changed;
        if registrations.contains_key(&registration_id) {
            log::warn!(
                "Received a duplicate {method} registration with ID {registration_id}, replacing the previous one"
            );
            let previous_values = registrations.values().cloned().collect::<Vec<_>>();
            registrations.insert(registration_id, new_registration);
            applicable_set_changed = !previous_values
                .iter()
                .all(|previous| registrations.values().any(|current| current == previous))
                || !registrations
                    .values()
                    .all(|current| previous_values.contains(current));
        } else {
            applicable_set_changed = !registrations
                .values()
                .any(|existing| existing == &new_registration);
            registrations.insert(registration_id, new_registration);
        }
        let new_active_envelope = registrations
            .last()
            .map(|(_, registration)| registration.server_capabilities.clone());
        let new_active = match new_active_envelope {
            Some(mut envelope) => capability_of(&mut envelope).take(),
            None => local
                .initial_server_capabilities
                .get(&server_id)
                .cloned()
                .and_then(|mut capabilities| capability_of(&mut capabilities).take()),
        };
        let mut current_capabilities = server.capabilities();
        let previous_active = capability_of(&mut current_capabilities).take();
        let active_changed = new_active != previous_active;
        if active_changed {
            server.update_capabilities(|capabilities| *capability_of(capabilities) = new_active);
        }
        if active_changed || registration_changed {
            self.notify_server_capabilities_updated(server, cx);
        }
        Ok(CapabilityRegistrationChange {
            active_capability_changed: active_changed,
            text_document_registration_changed: applicable_set_changed,
            registration_changed,
        })
    }

    fn unregister_dynamic_text_document_capability<T: Clone + PartialEq>(
        &mut self,
        server: &LanguageServer,
        unregistration: &lsp::Unregistration,
        cx: &mut Context<Self>,
        capability_of: impl Fn(&mut lsp::ServerCapabilities) -> &mut Option<T>,
    ) -> anyhow::Result<Option<CapabilityRegistrationChange>> {
        let server_id = server.server_id();
        let local = self
            .as_local_mut()
            .context("Expected LSP Store to be local")?;
        let Some(registrations) = local
            .language_server_dynamic_registrations
            .get_mut(&server_id)
            .and_then(|dynamic_registrations| {
                dynamic_registrations
                    .text_documents
                    .get_mut(&unregistration.method)
            })
        else {
            log::warn!(
                "Attempted to unregister non-existent {} registration with ID {}",
                unregistration.method,
                unregistration.id
            );
            return Ok(None);
        };
        let Some(removed) = registrations.shift_remove(&unregistration.id) else {
            log::warn!(
                "Attempted to unregister non-existent {} registration with ID {}",
                unregistration.method,
                unregistration.id
            );
            return Ok(None);
        };
        let applicable_set_changed = !registrations
            .values()
            .any(|remaining| remaining == &removed);
        let new_active_envelope = registrations
            .last()
            .map(|(_, registration)| registration.server_capabilities.clone());
        let new_active = match new_active_envelope {
            Some(mut envelope) => capability_of(&mut envelope).take(),
            None => local
                .initial_server_capabilities
                .get(&server_id)
                .cloned()
                .and_then(|mut capabilities| capability_of(&mut capabilities).take()),
        };
        let mut current_capabilities = server.capabilities();
        let previous_active = capability_of(&mut current_capabilities).take();
        let active_changed = new_active != previous_active;
        if active_changed {
            server.update_capabilities(|capabilities| *capability_of(capabilities) = new_active);
        }
        self.notify_server_capabilities_updated(server, cx);
        Ok(Some(CapabilityRegistrationChange {
            active_capability_changed: active_changed,
            text_document_registration_changed: applicable_set_changed,
            registration_changed: true,
        }))
    }

    /// Returns `true` when the registration changed the server's active capability value:
    /// duplicate-ID replacements of non-active registrations and (re-)registrations with
    /// options identical to the active ones do not.
    fn register_dynamic_capability<T: Clone + PartialEq>(
        &mut self,
        server: &LanguageServer,
        method: &str,
        registration_id: String,
        options: T,
        cx: &mut Context<Self>,
        registrations_of: impl FnOnce(&mut DynamicRegistrations) -> &mut CapabilityRegistrations<T>,
        capability_of: impl Fn(&mut lsp::ServerCapabilities) -> &mut Option<T>,
    ) -> anyhow::Result<bool> {
        let server_id = server.server_id();
        let local = self
            .as_local_mut()
            .context("Expected LSP Store to be local")?;
        let dynamic_registrations = local
            .language_server_dynamic_registrations
            .entry(server_id)
            .or_default();
        let registrations = registrations_of(dynamic_registrations);
        if registrations.is_empty() {
            let mut initial_capabilities = server.capabilities();
            if let Some(static_options) = capability_of(&mut initial_capabilities).take() {
                registrations.push((RegistrationSource::Static, static_options));
            }
        }
        let previously_active = registrations.last().map(|(_, options)| options.clone());
        if let Some(index) = registrations
            .iter()
            .position(|(source, _)| source.registration_id() == Some(registration_id.as_str()))
        {
            log::warn!(
                "Received a duplicate {method} registration with ID {registration_id}, replacing the previous one"
            );
            registrations[index].1 = options;
        } else {
            registrations.push((RegistrationSource::Dynamic(registration_id), options));
        }
        let active = registrations.last().map(|(_, options)| options.clone());
        let active_changed = active != previously_active;
        if active_changed {
            server.update_capabilities(|capabilities| *capability_of(capabilities) = active);
            self.notify_server_capabilities_updated(server, cx);
        }
        Ok(active_changed)
    }

    fn unregister_dynamic_capability<T: Clone + PartialEq>(
        &mut self,
        server: &LanguageServer,
        unregistration: &lsp::Unregistration,
        cx: &mut Context<Self>,
        registrations_of: impl FnOnce(&mut DynamicRegistrations) -> &mut CapabilityRegistrations<T>,
        capability_of: impl FnOnce(&mut lsp::ServerCapabilities) -> &mut Option<T>,
    ) -> anyhow::Result<Option<bool>> {
        let server_id = server.server_id();
        let local = self
            .as_local_mut()
            .context("Expected LSP Store to be local")?;
        let Some(dynamic_registrations) = local
            .language_server_dynamic_registrations
            .get_mut(&server_id)
        else {
            log::warn!(
                "Attempted to unregister non-existent {} registration with ID {}",
                unregistration.method,
                unregistration.id
            );
            return Ok(None);
        };
        let registrations = registrations_of(dynamic_registrations);
        let Some(index) = registrations
            .iter()
            .position(|(source, _)| source.registration_id() == Some(unregistration.id.as_str()))
        else {
            log::warn!(
                "Attempted to unregister non-existent {} registration with ID {}",
                unregistration.method,
                unregistration.id
            );
            return Ok(None);
        };
        let removed_active = index + 1 == registrations.len();
        let (_, removed_options) = registrations.remove(index);
        let restored = registrations.last().map(|(_, options)| options.clone());
        if !removed_active || restored.as_ref() == Some(&removed_options) {
            return Ok(Some(false));
        }
        server.update_capabilities(|capabilities| *capability_of(capabilities) = restored);
        self.notify_server_capabilities_updated(server, cx);
        Ok(Some(true))
    }

    fn update_paths_watched_for_rename(&mut self, server: &LanguageServer) {
        let Some(local) = self.as_local_mut() else {
            return;
        };
        let watcher = server
            .capabilities()
            .workspace
            .and_then(|workspace| workspace.file_operations)
            .and_then(|file_operations| {
                let did_rename_caps = file_operations.did_rename.as_ref();
                let will_rename_caps = file_operations.will_rename.as_ref();
                did_rename_caps.or(will_rename_caps)?;
                Some(
                    RenamePathsWatchedForServer::default()
                        .with_did_rename_patterns(did_rename_caps)
                        .with_will_rename_patterns(will_rename_caps),
                )
            });
        match watcher {
            Some(watcher) => {
                local
                    .language_server_paths_watched_for_rename
                    .insert(server.server_id(), watcher);
            }
            None => {
                local
                    .language_server_paths_watched_for_rename
                    .remove(&server.server_id());
            }
        }
    }

    fn apply_completion_triggers(&self, server_id: LanguageServerId, cx: &mut Context<Self>) {
        let Some(local) = self.as_local() else {
            return;
        };
        let Some(LanguageServerState::Running { adapter, .. }) =
            local.language_servers.get(&server_id)
        else {
            return;
        };
        let mut buffers_with_language_server = Vec::new();
        for handle in self.buffer_store.read(cx).buffers() {
            let buffer_id = handle.read(cx).remote_id();
            if local
                .buffers_opened_in_servers
                .get(&buffer_id)
                .filter(|servers| servers.contains(&server_id))
                .is_some()
            {
                buffers_with_language_server.push(handle);
            }
        }
        for handle in buffers_with_language_server {
            handle.update(cx, |buffer, cx| {
                let triggers =
                    completion_trigger_characters_for_buffer(local, server_id, adapter, buffer);
                buffer.set_completion_triggers(server_id, triggers, cx);
            });
        }
    }

    pub(super) fn register_server_capabilities(
        &mut self,
        server_id: LanguageServerId,
        params: lsp::RegistrationParams,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<()> {
        let server = self
            .language_server_for_id(server_id)
            .with_context(|| format!("no server {server_id} found"))?;
        for reg in params.registrations {
            match reg.method.as_str() {
                "workspace/didChangeWatchedFiles" => {
                    if let Some(options) = reg.register_options {
                        let notify = if let Some(local_lsp_store) = self.as_local_mut() {
                            let caps = serde_json::from_value(options)?;
                            local_lsp_store
                                .on_lsp_did_change_watched_files(server_id, &reg.id, caps, cx);
                            true
                        } else {
                            false
                        };
                        if notify {
                            self.notify_server_capabilities_updated(&server, cx);
                        }
                    }
                }
                "workspace/didChangeConfiguration" => {
                    // Ignore payload since we notify clients of setting changes unconditionally, relying on them pulling the latest settings.
                }
                "workspace/didChangeWorkspaceFolders" => {
                    // In this case register options is an empty object, we can ignore it
                    let caps = lsp::WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Right(reg.id.clone())),
                    };
                    self.register_dynamic_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        caps,
                        cx,
                        |registrations| &mut registrations.workspace_folders,
                        |capabilities| {
                            &mut capabilities
                                .workspace
                                .get_or_insert_default()
                                .workspace_folders
                        },
                    )?;
                }
                "workspace/symbol" => {
                    let options = parse_register_capabilities(reg.register_options)?;
                    self.register_dynamic_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        options,
                        cx,
                        |registrations| &mut registrations.workspace_symbol,
                        |capabilities| &mut capabilities.workspace_symbol_provider,
                    )?;
                }
                "workspace/fileOperations" => {
                    if let Some(options) = reg.register_options {
                        let caps = serde_json::from_value(options)?;
                        if self.register_dynamic_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            caps,
                            cx,
                            |registrations| &mut registrations.file_operations,
                            |capabilities| {
                                &mut capabilities
                                    .workspace
                                    .get_or_insert_default()
                                    .file_operations
                            },
                        )? {
                            self.update_paths_watched_for_rename(&server);
                        }
                    }
                }
                "workspace/executeCommand" => {
                    if let Some(options) = reg.register_options {
                        let options = serde_json::from_value(options)?;
                        self.register_dynamic_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            options,
                            cx,
                            |registrations| &mut registrations.execute_command,
                            |capabilities| &mut capabilities.execute_command_provider,
                        )?;
                    }
                }
                "textDocument/rangeFormatting" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    self.register_dynamic_text_document_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        options,
                        document_selector,
                        cx,
                        |capabilities| &mut capabilities.document_range_formatting_provider,
                    )?;
                }
                "textDocument/onTypeFormatting" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    if let Some(options) = reg
                        .register_options
                        .map(serde_json::from_value)
                        .transpose()?
                    {
                        self.register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            options,
                            document_selector,
                            cx,
                            |capabilities| &mut capabilities.document_on_type_formatting_provider,
                        )?;
                    }
                }
                "textDocument/formatting" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    self.register_dynamic_text_document_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        options,
                        document_selector,
                        cx,
                        |capabilities| &mut capabilities.document_formatting_provider,
                    )?;
                }
                "textDocument/rename" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    self.register_dynamic_text_document_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        options,
                        document_selector,
                        cx,
                        |capabilities| &mut capabilities.rename_provider,
                    )?;
                }
                "textDocument/inlayHint" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    if self
                        .register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            options,
                            document_selector,
                            cx,
                            |capabilities| &mut capabilities.inlay_hint_provider,
                        )?
                        .applicability_may_have_changed()
                    {
                        self.refresh_inlay_hints(server_id, cx);
                    }
                }
                "textDocument/documentSymbol" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    if self
                        .register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            options,
                            document_selector,
                            cx,
                            |capabilities| &mut capabilities.document_symbol_provider,
                        )?
                        .applicability_may_have_changed()
                    {
                        self.refresh_document_symbols(Some(server_id), cx);
                    }
                }
                "textDocument/codeAction" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    let provider = match options {
                        OneOf::Left(value) => lsp::CodeActionProviderCapability::Simple(value),
                        OneOf::Right(caps) => caps,
                    };
                    self.register_dynamic_text_document_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        provider,
                        document_selector,
                        cx,
                        |capabilities| &mut capabilities.code_action_provider,
                    )?;
                }
                "textDocument/prepareCallHierarchy" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    let provider = match options {
                        OneOf::Left(value) => lsp::CallHierarchyServerCapability::Simple(value),
                        OneOf::Right(options) => {
                            lsp::CallHierarchyServerCapability::Options(options)
                        }
                    };
                    self.register_dynamic_text_document_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        provider,
                        document_selector,
                        cx,
                        |capabilities| &mut capabilities.call_hierarchy_provider,
                    )?;
                }
                "textDocument/definition" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    self.register_dynamic_text_document_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        options,
                        document_selector,
                        cx,
                        |capabilities| &mut capabilities.definition_provider,
                    )?;
                }
                "textDocument/documentHighlight" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    self.register_dynamic_text_document_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        options,
                        document_selector,
                        cx,
                        |capabilities| &mut capabilities.document_highlight_provider,
                    )?;
                }
                "textDocument/completion" => {
                    if let Some(registration_options) = reg
                        .register_options
                        .map(serde_json::from_value::<lsp::CompletionRegistrationOptions>)
                        .transpose()?
                    {
                        self.register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            registration_options.completion_options,
                            registration_options
                                .text_document_registration_options
                                .document_selector,
                            cx,
                            |capabilities| &mut capabilities.completion_provider,
                        )?;
                        self.apply_completion_triggers(server_id, cx);
                    }
                }
                "textDocument/hover" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    let provider = match options {
                        OneOf::Left(value) => lsp::HoverProviderCapability::Simple(value),
                        OneOf::Right(caps) => caps,
                    };
                    self.register_dynamic_text_document_capability(
                        &server,
                        &reg.method,
                        reg.id,
                        provider,
                        document_selector,
                        cx,
                        |capabilities| &mut capabilities.hover_provider,
                    )?;
                }
                "textDocument/signatureHelp" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    if let Some(caps) = reg
                        .register_options
                        .map(serde_json::from_value)
                        .transpose()?
                    {
                        self.register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            caps,
                            document_selector,
                            cx,
                            |capabilities| &mut capabilities.signature_help_provider,
                        )?;
                    }
                }
                "textDocument/didChange" => {
                    if let Some(sync_kind) = reg
                        .register_options
                        .and_then(|opts| opts.get("syncKind").cloned())
                        .map(serde_json::from_value::<lsp::TextDocumentSyncKind>)
                        .transpose()?
                    {
                        server.update_capabilities(|capabilities| {
                            let mut sync_options = take_text_document_sync_options(capabilities);
                            sync_options.change = Some(sync_kind);
                            capabilities.text_document_sync =
                                Some(lsp::TextDocumentSyncCapability::Options(sync_options));
                        });
                        self.notify_server_capabilities_updated(&server, cx);
                    }
                }
                "textDocument/didSave" => {
                    if let Some(include_text) = reg
                        .register_options
                        .map(|opts| {
                            let transpose = opts
                                .get("includeText")
                                .cloned()
                                .map(serde_json::from_value::<Option<bool>>)
                                .transpose();
                            match transpose {
                                Ok(value) => Ok(value.flatten()),
                                Err(e) => Err(e),
                            }
                        })
                        .transpose()?
                    {
                        server.update_capabilities(|capabilities| {
                            let mut sync_options = take_text_document_sync_options(capabilities);
                            sync_options.save =
                                Some(TextDocumentSyncSaveOptions::SaveOptions(lsp::SaveOptions {
                                    include_text,
                                }));
                            capabilities.text_document_sync =
                                Some(lsp::TextDocumentSyncCapability::Options(sync_options));
                        });
                        self.notify_server_capabilities_updated(&server, cx);
                    }
                }
                "textDocument/codeLens" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    if let Some(options) = reg
                        .register_options
                        .map(serde_json::from_value)
                        .transpose()?
                    {
                        if self
                            .register_dynamic_text_document_capability(
                                &server,
                                &reg.method,
                                reg.id,
                                options,
                                document_selector,
                                cx,
                                |capabilities| &mut capabilities.code_lens_provider,
                            )?
                            .applicability_may_have_changed()
                        {
                            self.refresh_code_lens(Some(server_id), cx);
                        }
                    }
                }
                "textDocument/diagnostic" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    if let Some(caps) = reg
                        .register_options
                        .map(serde_json::from_value::<DiagnosticServerCapabilities>)
                        .transpose()?
                    {
                        let registration_id = reg.id.clone();
                        self.register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            caps.clone(),
                            document_selector,
                            cx,
                            |capabilities| &mut capabilities.diagnostic_provider,
                        )?;

                        let supports_workspace_diagnostics =
                            |capabilities: &DiagnosticServerCapabilities| match capabilities {
                                DiagnosticServerCapabilities::Options(diagnostic_options) => {
                                    diagnostic_options.workspace_diagnostics
                                }
                                DiagnosticServerCapabilities::RegistrationOptions(
                                    diagnostic_registration_options,
                                ) => {
                                    diagnostic_registration_options
                                        .diagnostic_options
                                        .workspace_diagnostics
                                }
                            };

                        let local = self
                            .as_local_mut()
                            .context("Expected LSP Store to be local")?;
                        let state = local
                            .language_servers
                            .get_mut(&server_id)
                            .context("Could not obtain Language Servers state")?;
                        if let LanguageServerState::Running {
                            workspace_diagnostics_refresh_tasks,
                            ..
                        } = state
                        {
                            workspace_diagnostics_refresh_tasks
                                .remove(&Some(registration_id.clone()));
                            if supports_workspace_diagnostics(&caps)
                                && let Some(task) = lsp_workspace_diagnostics_refresh(
                                    Some(registration_id.clone()),
                                    caps,
                                    server.clone(),
                                    cx,
                                )
                            {
                                workspace_diagnostics_refresh_tasks
                                    .insert(Some(registration_id), task);
                            }
                        }

                        let _ = self.pull_document_diagnostics_for_server(server_id, None, cx);
                    }
                }
                "textDocument/documentColor" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    let provider = match options {
                        OneOf::Left(value) => lsp::ColorProviderCapability::Simple(value),
                        OneOf::Right(caps) => caps,
                    };
                    if self
                        .register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            provider,
                            document_selector,
                            cx,
                            |capabilities| &mut capabilities.color_provider,
                        )?
                        .applicability_may_have_changed()
                    {
                        self.refresh_document_colors(Some(server_id), cx);
                    }
                }
                "textDocument/foldingRange" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    let options = parse_register_capabilities(reg.register_options)?;
                    let provider = match options {
                        OneOf::Left(value) => lsp::FoldingRangeProviderCapability::Simple(value),
                        OneOf::Right(caps) => caps,
                    };
                    if self
                        .register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            provider,
                            document_selector,
                            cx,
                            |capabilities| &mut capabilities.folding_range_provider,
                        )?
                        .applicability_may_have_changed()
                    {
                        self.refresh_folding_ranges(Some(server_id), cx);
                    }
                }
                "textDocument/documentLink" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    if let Some(caps) = reg
                        .register_options
                        .map(serde_json::from_value)
                        .transpose()?
                    {
                        if self
                            .register_dynamic_text_document_capability(
                                &server,
                                &reg.method,
                                reg.id,
                                caps,
                                document_selector,
                                cx,
                                |capabilities| &mut capabilities.document_link_provider,
                            )?
                            .applicability_may_have_changed()
                        {
                            self.refresh_document_links(Some(server_id), cx);
                        }
                    }
                }
                "textDocument/semanticTokens" => {
                    let document_selector =
                        parse_text_document_registration(reg.register_options.as_ref())?;
                    if let Some(caps) = reg
                        .register_options
                        .map(serde_json::from_value::<lsp::SemanticTokensRegistrationOptions>)
                        .transpose()?
                    {
                        if self.register_dynamic_text_document_capability(
                            &server,
                            &reg.method,
                            reg.id,
                            lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(caps),
                            document_selector,
                            cx,
                            |capabilities| &mut capabilities.semantic_tokens_provider,
                        )?
                        .ordered_capability_may_have_changed()
                        {
                            // Re-query already-open buffers, which would otherwise keep
                            // tree-sitter-only highlighting until edited.
                            self.refresh_semantic_tokens(server_id, cx);
                        }
                    }
                }
                _ => log::warn!("unhandled capability registration: {reg:?}"),
            }
        }

        Ok(())
    }

    pub(super) fn unregister_server_capabilities(
        &mut self,
        server_id: LanguageServerId,
        params: lsp::UnregistrationParams,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<()> {
        let server = self
            .language_server_for_id(server_id)
            .with_context(|| format!("no server {server_id} found"))?;
        for unreg in params.unregisterations.iter() {
            match unreg.method.as_str() {
                "workspace/didChangeWatchedFiles" => {
                    let notify = if let Some(local_lsp_store) = self.as_local_mut() {
                        local_lsp_store
                            .on_lsp_unregister_did_change_watched_files(server_id, &unreg.id, cx);
                        true
                    } else {
                        false
                    };
                    if notify {
                        self.notify_server_capabilities_updated(&server, cx);
                    }
                }
                "workspace/didChangeConfiguration" => {
                    // Ignore payload since we notify clients of setting changes unconditionally, relying on them pulling the latest settings.
                }
                "workspace/didChangeWorkspaceFolders" => {
                    self.unregister_dynamic_capability(
                        &server,
                        unreg,
                        cx,
                        |registrations| &mut registrations.workspace_folders,
                        |capabilities| {
                            &mut capabilities
                                .workspace
                                .get_or_insert_default()
                                .workspace_folders
                        },
                    )?;
                }
                "workspace/symbol" => {
                    self.unregister_dynamic_capability(
                        &server,
                        unreg,
                        cx,
                        |registrations| &mut registrations.workspace_symbol,
                        |capabilities| &mut capabilities.workspace_symbol_provider,
                    )?;
                }
                "workspace/fileOperations" => {
                    if self.unregister_dynamic_capability(
                        &server,
                        unreg,
                        cx,
                        |registrations| &mut registrations.file_operations,
                        |capabilities| {
                            &mut capabilities
                                .workspace
                                .get_or_insert_default()
                                .file_operations
                        },
                    )? == Some(true)
                    {
                        self.update_paths_watched_for_rename(&server);
                    }
                }
                "workspace/executeCommand" => {
                    self.unregister_dynamic_capability(
                        &server,
                        unreg,
                        cx,
                        |registrations| &mut registrations.execute_command,
                        |capabilities| &mut capabilities.execute_command_provider,
                    )?;
                }
                "textDocument/rangeFormatting" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.document_range_formatting_provider,
                    )?;
                }
                "textDocument/onTypeFormatting" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.document_on_type_formatting_provider,
                    )?;
                }
                "textDocument/formatting" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.document_formatting_provider,
                    )?;
                }
                "textDocument/rename" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.rename_provider,
                    )?;
                }
                "textDocument/codeAction" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.code_action_provider,
                    )?;
                }
                "textDocument/prepareCallHierarchy" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.call_hierarchy_provider,
                    )?;
                }
                "textDocument/definition" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.definition_provider,
                    )?;
                }
                "textDocument/documentHighlight" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.document_highlight_provider,
                    )?;
                }
                "textDocument/completion" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.completion_provider,
                        )?
                        .is_some()
                    {
                        self.apply_completion_triggers(server_id, cx);
                    }
                }
                "textDocument/hover" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.hover_provider,
                    )?;
                }
                "textDocument/signatureHelp" => {
                    self.unregister_dynamic_text_document_capability(
                        &server,
                        unreg,
                        cx,
                        |capabilities| &mut capabilities.signature_help_provider,
                    )?;
                }
                "textDocument/semanticTokens" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.semantic_tokens_provider,
                        )?
                        .is_some_and(
                            CapabilityRegistrationChange::ordered_capability_may_have_changed,
                        )
                    {
                        self.refresh_semantic_tokens(server_id, cx);
                    }
                }
                "textDocument/didChange" => {
                    server.update_capabilities(|capabilities| {
                        let mut sync_options = take_text_document_sync_options(capabilities);
                        sync_options.change = None;
                        capabilities.text_document_sync =
                            Some(lsp::TextDocumentSyncCapability::Options(sync_options));
                    });
                    self.notify_server_capabilities_updated(&server, cx);
                }
                "textDocument/didSave" => {
                    server.update_capabilities(|capabilities| {
                        let mut sync_options = take_text_document_sync_options(capabilities);
                        sync_options.save = None;
                        capabilities.text_document_sync =
                            Some(lsp::TextDocumentSyncCapability::Options(sync_options));
                    });
                    self.notify_server_capabilities_updated(&server, cx);
                }
                "textDocument/inlayHint" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.inlay_hint_provider,
                        )?
                        .is_some_and(CapabilityRegistrationChange::applicability_may_have_changed)
                    {
                        self.refresh_inlay_hints(server_id, cx);
                    }
                }
                "textDocument/documentSymbol" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.document_symbol_provider,
                        )?
                        .is_some_and(CapabilityRegistrationChange::applicability_may_have_changed)
                    {
                        self.refresh_document_symbols(Some(server_id), cx);
                    }
                }
                "textDocument/codeLens" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.code_lens_provider,
                        )?
                        .is_some_and(CapabilityRegistrationChange::applicability_may_have_changed)
                    {
                        self.refresh_code_lens(Some(server_id), cx);
                    }
                }
                "textDocument/diagnostic" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.diagnostic_provider,
                        )?
                        .is_some()
                    {
                        let local = self
                            .as_local_mut()
                            .context("Expected LSP Store to be local")?;
                        let state = local
                            .language_servers
                            .get_mut(&server_id)
                            .context("Could not obtain Language Servers state")?;
                        if let LanguageServerState::Running {
                            workspace_diagnostics_refresh_tasks,
                            ..
                        } = state
                        {
                            workspace_diagnostics_refresh_tasks.remove(&Some(unreg.id.clone()));
                        }

                        self.clear_unregistered_diagnostics(
                            server_id,
                            SharedString::from(unreg.id.clone()),
                            cx,
                        )?;
                    }
                }
                "textDocument/documentColor" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.color_provider,
                        )?
                        .is_some_and(CapabilityRegistrationChange::applicability_may_have_changed)
                    {
                        self.refresh_document_colors(Some(server_id), cx);
                    }
                }
                "textDocument/foldingRange" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.folding_range_provider,
                        )?
                        .is_some_and(CapabilityRegistrationChange::applicability_may_have_changed)
                    {
                        self.refresh_folding_ranges(Some(server_id), cx);
                    }
                }
                "textDocument/documentLink" => {
                    if self
                        .unregister_dynamic_text_document_capability(
                            &server,
                            unreg,
                            cx,
                            |capabilities| &mut capabilities.document_link_provider,
                        )?
                        .is_some_and(CapabilityRegistrationChange::applicability_may_have_changed)
                    {
                        self.refresh_document_links(Some(server_id), cx);
                    }
                }
                _ => log::warn!("unhandled capability unregistration: {unreg:?}"),
            }
        }

        Ok(())
    }
}

fn parse_text_document_registration(
    register_options: Option<&serde_json::Value>,
) -> Result<Option<lsp::DocumentSelector>> {
    Ok(register_options
        .cloned()
        .map(serde_json::from_value::<lsp::TextDocumentRegistrationOptions>)
        .transpose()?
        .and_then(|options| options.document_selector))
}

// Registration with registerOptions as null, should fallback to true.
// https://github.com/microsoft/vscode-languageserver-node/blob/d90a87f9557a0df9142cfb33e251cfa6fe27d970/client/src/common/client.ts#L2133
fn parse_register_capabilities<T: serde::de::DeserializeOwned>(
    register_options: Option<serde_json::Value>,
) -> Result<OneOf<bool, T>> {
    Ok(match register_options {
        Some(options) => OneOf::Right(serde_json::from_value::<T>(options)?),
        None => OneOf::Left(true),
    })
}

fn take_text_document_sync_options(
    capabilities: &mut lsp::ServerCapabilities,
) -> lsp::TextDocumentSyncOptions {
    match capabilities.text_document_sync.take() {
        Some(lsp::TextDocumentSyncCapability::Options(sync_options)) => sync_options,
        Some(lsp::TextDocumentSyncCapability::Kind(sync_kind)) => {
            let mut sync_options = lsp::TextDocumentSyncOptions::default();
            sync_options.change = Some(sync_kind);
            sync_options
        }
        None => lsp::TextDocumentSyncOptions::default(),
    }
}
