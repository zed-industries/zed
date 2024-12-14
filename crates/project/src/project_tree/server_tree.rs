//! This module defines an LSP Tree.
//!
//! An LSP Tree is responsible for determining which language servers apply to a given project path.
//!
//! ## RPC
//! LSP Tree is transparent to RPC peers; when clients ask host to spawn a new language server, the host will perform LSP Tree lookup for provided path; it may decide
//! to reuse existing language server. The client maintains it's own LSP Tree that is a subset of host LSP Tree. Done this way, the client does not need to
//! ask about suitable language server for each path it interacts with; it can resolve most of the queries locally.
//! This module defines a Project Tree.

use std::{collections::BTreeMap, path::Path, sync::Arc};

use collections::HashMap;
use language::LanguageName;
use lsp::Url;

use crate::LanguageServerId;

pub trait LspRootFinder {
    fn find_root(&self) -> () {}
}

enum Action {
    PinpointTo,
    ExtendWorkspaceFolders(LanguageServerId, Url),
}

pub type AbsWorkspaceRootPath = Arc<Path>;

#[derive(Default)]
pub struct LspTree {
    /// Language servers for which we can just update workspaceFolders when we detect a new project root
    umbrella_language_servers: HashMap<LanguageName, Vec<LanguageServerId>>,
    pinpoint_language_servers:
        HashMap<LanguageName, BTreeMap<AbsWorkspaceRootPath, Vec<LanguageServerId>>>,
}

impl LspTree {
    fn new() -> Self {
        Self::default()
    }
    pub fn insert_new_server(
        &mut self,
        name: LanguageName,
        id: LanguageServerId,
        root_path: Option<AbsWorkspaceRootPath>,
    ) {
        if let Some(root_path) = root_path {
            self.pinpoint_language_servers
                .entry(name)
                .or_default()
                .entry(root_path)
                .or_default()
                .push(id);
        } else {
            self.umbrella_language_servers
                .entry(name)
                .or_default()
                .push(id);
        }
    }
    pub fn get<'a, 'b>(
        &'a mut self,
        language: LanguageName,
        file: &'b Path,
    ) -> impl Iterator<Item = LanguageServerId> + 'b
    where
        'a: 'b,
    {
        self.pinpoint_language_servers
            .get(&language)
            .into_iter()
            .flat_map(move |joint| {
                file.ancestors().flat_map(move |ancestor| {
                    joint.get(ancestor).map(|servers| servers.iter().cloned())
                })
            })
            .flatten()
            .chain(
                self.umbrella_language_servers
                    .get(&language)
                    .into_iter()
                    .flat_map(|servers| servers.iter().cloned()),
            )
    }
}

#[cfg(test)]
mod tests {}
