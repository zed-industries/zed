pub fn count_tree_sitter_errors<'a>(nodes: impl Iterator<Item = tree_sitter::Node<'a>>) -> usize {
    let mut total_count: usize = 0;
    for node in nodes {
        let mut cursor = node.walk();
        'node: loop {
            let current = cursor.node();
            if current.is_error() || current.is_missing() {
                total_count += 1;
            }
            if current.has_error() && cursor.goto_first_child() {
                continue;
            }
            if cursor.goto_next_sibling() {
                continue;
            }
            loop {
                if !cursor.goto_parent() {
                    break 'node;
                }
                if cursor.goto_next_sibling() {
                    continue;
                }
            }
        }
    }
    total_count
}
