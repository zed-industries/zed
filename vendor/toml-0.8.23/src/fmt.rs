#[derive(Copy, Clone, Default)]
pub(crate) struct DocumentFormatter {
    pub(crate) multiline_array: bool,
    is_value: bool,
}

impl toml_edit::visit_mut::VisitMut for DocumentFormatter {
    fn visit_document_mut(&mut self, node: &mut toml_edit::DocumentMut) {
        toml_edit::visit_mut::visit_document_mut(self, node);
    }

    fn visit_item_mut(&mut self, node: &mut toml_edit::Item) {
        let is_parent_value = self.is_value;
        if !is_parent_value {
            let other = std::mem::take(node);
            let other = match other.into_table().map(toml_edit::Item::Table) {
                Ok(i) => i,
                Err(i) => i,
            };
            let other = match other
                .into_array_of_tables()
                .map(toml_edit::Item::ArrayOfTables)
            {
                Ok(i) => i,
                Err(i) => i,
            };
            self.is_value = other.is_value();
            *node = other;
        }

        toml_edit::visit_mut::visit_item_mut(self, node);
        self.is_value = is_parent_value;
    }

    fn visit_table_mut(&mut self, node: &mut toml_edit::Table) {
        node.decor_mut().clear();

        // Empty tables could be semantically meaningful, so make sure they are not implicit
        if !node.is_empty() {
            node.set_implicit(true);
        }

        toml_edit::visit_mut::visit_table_mut(self, node);
    }

    fn visit_value_mut(&mut self, node: &mut toml_edit::Value) {
        node.decor_mut().clear();

        toml_edit::visit_mut::visit_value_mut(self, node);
    }

    fn visit_array_mut(&mut self, node: &mut toml_edit::Array) {
        toml_edit::visit_mut::visit_array_mut(self, node);

        if !self.multiline_array || (0..=1).contains(&node.len()) {
            node.set_trailing("");
            node.set_trailing_comma(false);
        } else {
            for item in node.iter_mut() {
                item.decor_mut().set_prefix("\n    ");
            }
            node.set_trailing("\n");
            node.set_trailing_comma(true);
        }
    }
}
