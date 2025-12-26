mod gum_tree;

pub use gum_tree::{
    DiffNode, DiffOperation, DiffResult, DiffTree, Matching, NodeId, generate_diff, match_trees,
};

/// Compute a syntax-aware diff between two syntax trees.
///
/// Returns a `SyntaxDiffResult` containing the operations needed to transform
/// the old tree into the new tree, with operations aligned to AST node boundaries.
pub fn diff_trees<'a>(
    old_tree: &'a tree_sitter::Tree,
    old_text: &'a str,
    new_tree: &'a tree_sitter::Tree,
    new_text: &'a str,
) -> DiffResult {
    let old_diff_tree = DiffTree::new(old_tree, old_text);
    let new_diff_tree = DiffTree::new(new_tree, new_text);
    let matching = gum_tree::match_trees(&old_diff_tree, &new_diff_tree);

    gum_tree::generate_diff(&old_diff_tree, &new_diff_tree, &matching)
}
