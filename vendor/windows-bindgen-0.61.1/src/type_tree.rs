use super::*;

// This uses BTree rather than Hash as we're getting close to writing the tokens in sorted order.
#[derive(Debug)]
pub struct TypeTree {
    pub namespace: &'static str,
    pub nested: BTreeMap<&'static str, Self>,
    pub types: BTreeSet<Type>,
}

impl TypeTree {
    pub fn new(dependencies: &TypeMap) -> Self {
        let mut tree = Self::with_namespace("");

        for (tn, types) in dependencies.iter() {
            let tree = tree.insert_namespace(tn.namespace());
            types.iter().for_each(|ty| {
                tree.types.insert(ty.clone());
            });
        }

        tree
    }

    // This is used to provide a flat iterator of trees so that they can be processed in parallel.
    pub fn flatten_trees(&self) -> Vec<&Self> {
        let mut flatten = if self.namespace.is_empty() {
            vec![]
        } else {
            vec![self]
        };
        flatten.extend(self.nested.values().flat_map(|tree| tree.flatten_trees()));
        flatten
    }

    // This is used to actually remove the tree structure and just have a flat iterator or types
    // that just happen to be sorted, thanks to BTreeSet, so that a flat list of types can be
    // generated in sort order.
    pub fn flatten_types(mut self) -> BTreeSet<Type> {
        fn flatten(set: &mut BTreeSet<Type>, tree: &mut TypeTree) {
            set.append(&mut tree.types);

            for tree in tree.nested.values_mut() {
                flatten(set, tree);
            }
        }

        let mut types = BTreeSet::new();
        flatten(&mut types, &mut self);
        types
    }

    pub fn feature(&self) -> String {
        self.namespace.split_once('.').unwrap().1.replace('.', "_")
    }

    fn with_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            nested: BTreeMap::new(),
            types: BTreeSet::new(),
        }
    }

    fn insert_namespace(&mut self, namespace: &'static str) -> &mut Self {
        fn insert_namespace<'a>(
            parent: &'a mut TypeTree,
            namespace: &'static str,
            pos: usize,
        ) -> &'a mut TypeTree {
            if let Some(next) = namespace[pos..].find('.') {
                let next = pos + next;

                let parent = parent
                    .nested
                    .entry(&namespace[pos..next])
                    .or_insert_with(|| TypeTree::with_namespace(&namespace[..next]));

                insert_namespace(parent, namespace, next + 1)
            } else {
                parent
                    .nested
                    .entry(&namespace[pos..])
                    .or_insert_with(|| TypeTree::with_namespace(namespace))
            }
        }

        if namespace.is_empty() {
            self
        } else {
            insert_namespace(self, namespace, 0)
        }
    }
}
