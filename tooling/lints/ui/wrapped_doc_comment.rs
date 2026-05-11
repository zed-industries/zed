// Tests for the `wrapped_doc_comment` lint.

#![allow(unused)]

// --- Should warn ---

/// Returns a list of references including file paths,
/// line numbers, and code snippets.
pub struct WrappedTwoLines;

/// This performs a semantic rename, updating all references to the symbol
/// across all files in the project. The language server determines which
/// occurrences to rename based on the symbol's type and scope.
pub struct WrappedThreeLines;

/// First paragraph fits on one line.
///
/// Second paragraph wraps across multiple
/// lines, which is what we want to flag.
pub struct WrappedSecondParagraph;

pub struct WrappedField {
    /// The relative path of the file containing the symbol
    /// (e.g. "crates/editor/src/editor.rs").
    pub file_path: String,
}

pub enum WrappedVariant {
    /// A long description that wraps over
    /// two physical lines for no good reason.
    First,
}

// --- Should NOT warn ---

/// A single-line summary.
pub struct OneLine;

/// First sentence on its own line.
///
/// Second sentence on its own line.
pub struct ParagraphsSeparated;

/// Fast file path pattern matching tool that works with any codebase size
///
/// - Supports glob patterns like "**/*.js" or "src/**/*.ts"
/// - Returns matching file paths sorted alphabetically
/// - Prefer the `grep` tool to this tool when searching for symbols
pub struct BulletList;

/// A heading-led block.
///
/// ### Why is this bad?
///
/// Some explanation that fits on one line.
pub struct WithHeading;

/// Numbered list items each on their own single line.
///
/// 1. First item is on its own line.
/// 2. Second item is on its own line.
/// 3. Third item is on its own line.
pub struct NumberedList;

/// Block-quoted single line.
///
/// > Quoted text on a single line.
pub struct BlockQuote;

/// Tag-bracketed example as a paragraph boundary.
///
/// <example>
/// Example body that fits on one line.
/// </example>
pub struct WithExampleTag;

pub struct PlainField {
    /// One-line field description.
    pub a: i32,
    /// Another one-line field description.
    pub b: i32,
}

fn main() {}
