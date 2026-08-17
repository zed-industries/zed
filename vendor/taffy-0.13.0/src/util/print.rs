//! Contains functions for printing a debug representation of the tree
use crate::tree::{NodeId, PrintTree};
use std::io;

/// Prints a debug representation of the computed layout for a tree of nodes, starting with the passed root node.
pub fn print_tree(tree: &impl PrintTree, root: NodeId) {
    // Buffer into a string so that output can be captured in tests
    let mut buffer = Vec::<u8>::new();
    write_tree(&mut buffer, tree, root).expect("Wrote tree debug representation to Stdout");
    println!("{}", std::str::from_utf8(&buffer).unwrap());
}

/// Writes a debug representation of the computed layout to the writer, starting with the passed root node.
pub fn write_tree(mut writer: impl io::Write, tree: &impl PrintTree, root: NodeId) -> io::Result<()> {
    writeln!(writer, "TREE")?;
    write_node(&mut writer, tree, root, false, String::new())?;

    /// Recursive function that writes each node in the tree
    fn write_node(
        writer: &mut impl io::Write,
        tree: &impl PrintTree,
        node_id: NodeId,
        has_sibling: bool,
        lines_string: String,
    ) -> io::Result<()> {
        let layout = &tree.get_final_layout(node_id);
        let display = tree.get_debug_label(node_id);
        let num_children = tree.child_count(node_id);

        let fork_string = if has_sibling { "├── " } else { "└── " };
        #[cfg(feature = "content_size")]
        writeln!(
                writer,
                "{lines}{fork} {display} [x: {x:<4} y: {y:<4} w: {width:<4} h: {height:<4} content_w: {content_width:<4} content_h: {content_height:<4} border: l:{bl} r:{br} t:{bt} b:{bb}, padding: l:{pl} r:{pr} t:{pt} b:{pb}] ({key:?})",
                lines = lines_string,
                fork = fork_string,
                display = display,
                x = layout.location.x,
                y = layout.location.y,
                width = layout.size.width,
                height = layout.size.height,
                content_width = layout.content_size.width,
                content_height = layout.content_size.height,
                bl = layout.border.left,
                br = layout.border.right,
                bt = layout.border.top,
                bb = layout.border.bottom,
                pl = layout.padding.left,
                pr = layout.padding.right,
                pt = layout.padding.top,
                pb = layout.padding.bottom,
                key = node_id,
            )?;
        #[cfg(not(feature = "content_size"))]
        writeln!(
            writer,
            "{lines}{fork} {display} [x: {x:<4} y: {y:<4} width: {width:<4} height: {height:<4}] ({key:?})",
            lines = lines_string,
            fork = fork_string,
            display = display,
            x = layout.location.x,
            y = layout.location.y,
            width = layout.size.width,
            height = layout.size.height,
            key = node_id,
        );
        let bar = if has_sibling { "│   " } else { "    " };
        let new_string = lines_string + bar;

        // Recurse into children
        for (index, child) in tree.child_ids(node_id).enumerate() {
            let has_sibling = index < num_children - 1;
            write_node(writer, tree, child, has_sibling, new_string.clone())?;
        }
        Ok(())
    }
    Ok(())
}
