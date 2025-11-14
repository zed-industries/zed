#!/usr/bin/env node
/*
  Minimal LSP server wrapper around vscode-markdown-languageservice.
  Runs over stdio and implements a subset of features commonly used in editors.
*/
const {
  createConnection,
  ProposedFeatures,
  TextDocuments,
  TextDocumentSyncKind,
} = require('vscode-languageserver/node');
const { Emitter } = require('vscode-languageserver-protocol');
const { TextDocument } = require('vscode-languageserver-textdocument');
const { URI } = require('vscode-uri');
const mdls = require('vscode-markdown-languageservice');
const MarkdownIt = require('markdown-it');
const fs = require('fs');
const path = require('path');

const connection = createConnection(ProposedFeatures.all);
const documents = new TextDocuments(TextDocument);

// Workspace implementation for the markdown language service
class Workspace {
  constructor() {
    this._docs = new Map();
    this.workspaceFolders = [];
    this.#onDidChangeMarkdownDocument = new Emitter();
    this.onDidChangeMarkdownDocument = this.#onDidChangeMarkdownDocument.event;
    this.#onDidCreateMarkdownDocument = new Emitter();
    this.onDidCreateMarkdownDocument = this.#onDidCreateMarkdownDocument.event;
    this.#onDidDeleteMarkdownDocument = new Emitter();
    this.onDidDeleteMarkdownDocument = this.#onDidDeleteMarkdownDocument.event;
  }
  #onDidChangeMarkdownDocument;
  #onDidCreateMarkdownDocument;
  #onDidDeleteMarkdownDocument;

  setFolders(folders) {
    this.workspaceFolders = folders;
  }
  onOpen(doc) { this._docs.set(doc.uri, doc); }
  onClose(uri) { this._docs.delete(uri); }
  async getAllMarkdownDocuments() { return Array.from(this._docs.values()); }
  hasMarkdownDocument(resource) { return this._docs.has(resource.toString()); }
  async openMarkdownDocument(resource) {
    const key = resource.toString();
    const existing = this._docs.get(key);
    if (existing) return existing;
    try {
      const text = await fs.promises.readFile(resource.fsPath, 'utf8');
      const doc = TextDocument.create(resource.toString(), 'markdown', 1, text);
      this._docs.set(key, doc);
      this.#onDidCreateMarkdownDocument.fire(doc);
      return doc;
    } catch {
      return undefined;
    }
  }
  async stat(resource) {
    try {
      const st = await fs.promises.stat(resource.fsPath);
      return { isDirectory: st.isDirectory() };
    } catch {
      return undefined;
    }
  }
  async readDirectory(resource) {
    try {
      const entries = await fs.promises.readdir(resource.fsPath, { withFileTypes: true });
      return entries.map(e => [e.name, { isDirectory: e.isDirectory() }]);
    } catch {
      return [];
    }
  }
}

const mdIt = MarkdownIt({ html: true });
const parser = new (class {
  constructor() { this.slugifier = mdls.githubSlugifier; }
  async tokenize(document) { return mdIt.parse(document.getText(), {}); }
})();

const logger = { log: (_level, _title, _message, _data) => {} };
const workspace = new Workspace();
let serverOptions = {};
let languageService = null;

// Diagnostics options: default; can be overridden from initializationOptions
let diagOptions = { validateFileLinks: true };

documents.onDidOpen((change) => {
  workspace.onOpen(change.document);
  void validateTextDocument(change.document);
});

documents.onDidChangeContent((change) => {
  workspace.onOpen(change.document);
  void validateTextDocument(change.document);
});

documents.onDidClose((change) => {
  workspace.onClose(change.document.uri);
  connection.sendDiagnostics({ uri: change.document.uri, diagnostics: [] });
});

async function validateTextDocument(document) {
  try {
    if (!languageService) return;
    const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
    const diagnostics = await languageService.computeDiagnostics(document, diagOptions, token);
    connection.sendDiagnostics({ uri: document.uri, diagnostics });
  } catch (e) {
    // Swallow diagnostics errors
  }
}

connection.onInitialize((params) => {
  serverOptions = params.initializationOptions || {};
  languageService = mdls.createLanguageService({ workspace, parser, logger, ...serverOptions });
  if (serverOptions.diagnostics) {
    diagOptions = { ...diagOptions, ...serverOptions.diagnostics };
  }
  // Capture workspace folders from client
  const folders = [];
  if (Array.isArray(params.workspaceFolders)) {
    for (const f of params.workspaceFolders) {
      try { folders.push(URI.parse(f.uri)); } catch {}
    }
  } else if (params.rootUri) {
    try { folders.push(URI.parse(params.rootUri)); } catch {}
  }
  workspace.setFolders(folders);
  console.error(`[markdown-lsp] initialized with ${folders.length} workspace folder(s)`);
  return {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      completionProvider: { resolveProvider: false },
      definitionProvider: true,
      referencesProvider: true,
      documentSymbolProvider: true,
      workspaceSymbolProvider: true,
      foldingRangeProvider: true,
      documentLinkProvider: { resolveProvider: true },
      renameProvider: { prepareProvider: true },
      codeActionProvider: true,
      hoverProvider: true,
      selectionRangeProvider: true,
    }
  };
});

connection.onShutdown(() => {
  try { if (languageService) languageService.dispose(); } catch {}
});

connection.onDidChangeConfiguration((params) => {
  try {
    const wsOptions = params.settings || {};
    serverOptions = { ...serverOptions, ...wsOptions };
    if (wsOptions.diagnostics) {
      diagOptions = { ...diagOptions, ...wsOptions.diagnostics };
    }
    if (languageService && typeof languageService.dispose === 'function') {
      languageService.dispose();
    }
    languageService = mdls.createLanguageService({ workspace, parser, logger, ...serverOptions });
  } catch (e) {
    // ignore
  }
});

connection.onCompletion(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return [];
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getCompletionItems(doc, params.position, { includeWorkspaceHeaderCompletions: true }, token);
});

connection.onDefinition(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return null;
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getDefinition(doc, params.position, token);
});

connection.onReferences(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return [];
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getReferences(doc, params.position, params.context, token);
});

connection.onDocumentSymbol(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return [];
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getDocumentSymbols(doc, { includeLinkDefinitions: true }, token);
});

connection.onDocumentLinks(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return [];
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  const links = await languageService.getDocumentLinks(doc, token);
  const resolved = await Promise.all(links.map(l => languageService.resolveDocumentLink(l, token).then(r => r ?? l)));
  return resolved;
});

connection.onFoldingRanges(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return [];
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getFoldingRanges(doc, token);
});

connection.onPrepareRename(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return null;
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.prepareRename(doc, params.position, token);
});

connection.onRenameRequest(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return null;
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getRenameEdit(doc, params.position, params.newName, token);
});

connection.onHover(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return null;
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getHover(doc, params.position, token);
});

connection.onSelectionRanges(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return [];
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getSelectionRanges(doc, params.positions, token);
});

connection.onCodeAction(async (params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc) return [];
  const token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose(){} }) };
  return languageService.getCodeActions(doc, params.range, params.context, token);
});

// Track open docs
documents.listen(connection);
connection.listen();
