use std::mem;

#[derive(Debug)]
enum TrieNode<T> {
    Leaf(T),
    Node(Vec<TrieNode<T>>),
}

impl<T> TrieNode<T> {
    #[allow(dead_code)]
    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf(_))
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        match self {
            Self::Leaf(_) => 0,
            Self::Node(hash_map) => hash_map.len(),
        }
    }
}

impl<T: Default> TrieNode<T> {
    pub fn insert(&mut self, val: T) -> &mut Self {
        match self {
            Self::Leaf(old_val) => {
                *self = Self::Node(vec![
                    TrieNode::Leaf(mem::take(old_val)),
                    TrieNode::Leaf(val),
                ]);
            }
            Self::Node(hash_map) => {
                hash_map.push(TrieNode::Leaf(val));
            }
        }
        self
    }
}

impl<T: Default> Default for TrieNode<T> {
    fn default() -> Self {
        TrieNode::Leaf(Default::default())
    }
}

#[derive(Debug)]
pub(crate) struct TrieBuilder {
    root: TrieNode<()>,
    keys: String,
}

impl TrieBuilder {
    pub fn new(keys: String, len: usize) -> Self {
        let root = TrieNode::Node(vec![TrieNode::Leaf(())]);
        let mut builder = TrieBuilder { keys, root };
        let mut p = vec![0];
        // constructs the trie
        for _ in 1..len {
            p = builder.next_perm(p);
        }
        builder
        // replace the points with the actual points
    }

    pub fn populate<T>(self, reverse: bool, iter: impl IntoIterator<Item = T>) -> Trie<T> {
        let (node, _) = TrieBuilder::new_rec(self.root, reverse, iter.into_iter());
        Trie {
            keys: self.keys,
            root: node,
        }
    }

    pub fn populate_with<TItem, TFinal>(
        mut self,
        reverse: bool,
        iter: impl IntoIterator<Item = TItem>,
        func: impl Fn(&str, TItem) -> TFinal,
    ) -> Trie<TFinal> {
        let node = mem::take(&mut self.root);
        let (node, _, _) =
            self.new_rec_with(node, "".to_string(), reverse, iter.into_iter(), &func);
        Trie {
            keys: self.keys,
            root: node,
        }
    }

    fn next_perm(&mut self, pointer: Vec<usize>) -> Vec<usize> {
        self.next_perm_internal(pointer)
    }

    // reverse puts the deeper nodes last
    // i.e. aa ab b c -> a b ca cb
    fn new_rec<T>(
        node: TrieNode<()>,
        reverse: bool,
        mut iter: impl Iterator<Item = T>,
    ) -> (TrieNode<T>, impl Iterator<Item = T>) {
        let node = match node {
            TrieNode::Leaf(_) => {
                let val = iter.next().unwrap();
                TrieNode::Leaf(val)
            }
            TrieNode::Node(mut list) => {
                let mut ret = Vec::<TrieNode<T>>::new();
                for _ in 0..list.len() {
                    let child = if reverse {
                        list.pop().unwrap()
                    } else {
                        list.remove(0)
                    };
                    let (new_child, new_iter) = TrieBuilder::new_rec(child, reverse, iter);
                    iter = new_iter;
                    ret.push(new_child);
                }
                TrieNode::Node(ret)
            }
        };
        (node, iter)
    }

    fn new_rec_with<TItem, TFinal, TFunc: Fn(&str, TItem) -> TFinal>(
        &self,
        node: TrieNode<()>,
        mut path: String,
        reverse: bool,
        mut iter: impl Iterator<Item = TItem>,
        func: &TFunc,
    ) -> (TrieNode<TFinal>, impl Iterator<Item = TItem>, String) {
        let node = match node {
            TrieNode::Leaf(_) => {
                let val = iter.next().unwrap();
                TrieNode::Leaf(func(&path, val))
            }
            TrieNode::Node(mut list) => {
                let mut ret = Vec::<TrieNode<TFinal>>::new();
                for i in 0..list.len() {
                    let character = self.keys.chars().nth(i).unwrap();
                    path.push(character);
                    let child = if reverse {
                        list.pop().unwrap()
                    } else {
                        list.remove(0)
                    };
                    let (new_child, new_iter, new_path) =
                        self.new_rec_with(child, path, reverse, iter, func);
                    iter = new_iter;
                    ret.push(new_child);
                    path = new_path;
                    path.pop();
                }
                TrieNode::Node(ret)
            }
        };
        (node, iter, path)
    }

    fn next_perm_internal(&mut self, mut pointer: Vec<usize>) -> Vec<usize> {
        let max_len = self.keys.len();
        let len = pointer.len();
        let last_index = pointer.last().unwrap().clone();
        if last_index < max_len - 1 {
            pointer[len - 1] += 1;
            // get parent to current node
            let node = self.pointer_to_node_ref(&pointer[0..len - 1]);
            debug_assert!(matches!(node, TrieNode::Node(_)));
            node.insert(());
            return pointer;
        }

        let mut depth = len;
        // find the first layer that is not full
        while depth > 0 {
            if !(pointer[depth - 1] < max_len - 1) {
                depth -= 1;
                continue;
            }

            pointer[depth - 1] += 1;
            // return to original depth adding 0s
            pointer[depth..].fill(0);
            *pointer.last_mut().unwrap() = 1;
            let node = self.pointer_to_node_ref(&pointer[0..len - 1]);
            debug_assert!(matches!(node, TrieNode::Leaf(_)));
            node.insert(());
            return pointer;
        }

        // current layer is completely full so we need to add a new layer
        pointer.fill(0);
        pointer.push(1);
        let node = self.pointer_to_node_ref(&pointer[0..len]);
        debug_assert!(matches!(node, TrieNode::Leaf(_)));
        node.insert(());
        pointer
    }

    fn pointer_to_node_ref(&mut self, pointer: &[usize]) -> &mut TrieNode<()> {
        if pointer.is_empty() {
            return &mut self.root;
        }
        let mut node = &mut self.root;
        for i in pointer {
            node = match node {
                TrieNode::Leaf(_) => return node,
                TrieNode::Node(list) => list.get_mut(i.clone()).unwrap(),
            };
        }
        node
    }
}

#[derive(Debug)]
pub(crate) struct Trie<T> {
    keys: String,
    root: TrieNode<T>,
}

impl<T> Trie<T> {
    pub fn new_from_vec(keys: String, values: Vec<T>, reverse: bool) -> Self {
        TrieBuilder::new(keys, values.len()).populate(reverse, values.into_iter())
    }

    #[allow(dead_code)]
    pub fn for_each(&self, func: impl Fn(&str, &T)) {
        let mut path = String::new();
        self.for_each_rec(&self.root, &mut path, false, &func);
    }

    #[allow(dead_code)]
    pub fn for_each_reverse(&self, func: impl Fn(&str, &T)) {
        let mut path = String::new();
        self.for_each_rec(&self.root, &mut path, true, &func);
    }

    fn for_each_rec(
        &self,
        node: &TrieNode<T>,
        path: &mut String,
        reverse: bool,
        func: &impl Fn(&str, &T),
    ) {
        match node {
            TrieNode::Leaf(val) => func(path, val),
            TrieNode::Node(list) => {
                if reverse {
                    for (i, child) in list.into_iter().enumerate().rev() {
                        self.for_each_rec_loop(i, child, path, true, func);
                    }
                    return;
                }
                for (i, child) in list.into_iter().enumerate() {
                    self.for_each_rec_loop(i, child, path, false, func);
                }
            }
        }
    }

    fn for_each_rec_loop(
        &self,
        i: usize,
        node: &TrieNode<T>,
        path: &mut String,
        reverse: bool,
        func: &impl Fn(&str, &T),
    ) {
        let character = self.keys.chars().nth(i).unwrap();
        path.push(character);
        self.for_each_rec(node, path, reverse, func);
        path.pop();
    }

    // returns a list of all permutations
    #[allow(dead_code)]
    pub fn trie_to_perms(&self) -> Vec<(String, &T)> {
        let mut perms = Vec::new();
        let mut path = String::new();
        self.trie_to_perms_rec(&self.root, &mut path, &mut perms, false);
        return perms;
    }

    // returns a list of all permutations with the indices reversed
    // i.e. a b ca cb -> c b ac ab
    #[allow(dead_code)]
    pub fn trie_to_perms_rev(&self) -> Vec<(String, &T)> {
        let mut perms = Vec::new();
        let mut path = String::new();
        self.trie_to_perms_rec(&self.root, &mut path, &mut perms, true);
        return perms;
    }

    fn trie_to_perms_rec<'a>(
        &'a self,
        node: &'a TrieNode<T>,
        path: &mut String,
        perms: &mut Vec<(String, &'a T)>,
        reverse: bool,
    ) {
        match node {
            TrieNode::Leaf(val) => {
                perms.push((path.clone(), &val));
                return;
            }
            TrieNode::Node(list) => {
                if reverse {
                    for (i, child) in list.into_iter().enumerate().rev() {
                        self.trie_to_perms_rec_loop(i, child, path, perms, reverse);
                    }
                    return;
                }
                for (i, child) in list.into_iter().enumerate() {
                    self.trie_to_perms_rec_loop(i, child, path, perms, reverse);
                }
            }
        }
    }

    fn trie_to_perms_rec_loop<'a>(
        &'a self,
        i: usize,
        node: &'a TrieNode<T>,
        path: &mut String,
        perms: &mut Vec<(String, &'a T)>,
        reverse: bool,
    ) {
        let character = self.keys.chars().nth(i).unwrap();
        path.push(character);
        self.trie_to_perms_rec(node, path, perms, reverse);
        path.pop();
    }

    pub fn trim(&mut self, character: char) -> TrimResult<&T> {
        let node = match &mut self.root {
            TrieNode::Leaf(_) => {
                return TrimResult::Err;
            }
            TrieNode::Node(ref mut map) => {
                let index = self.keys.find(character);
                let Some(index) = index else {
                    return TrimResult::NoChange;
                };
                if index >= map.len() {
                    return TrimResult::NoChange;
                }
                map.swap_remove(index)
            }
        };
        self.root = node;
        match &self.root {
            TrieNode::Leaf(val) => TrimResult::Found(&val),
            TrieNode::Node(_) => TrimResult::Changed,
        }
    }

    pub fn iter(&self) -> TrieIterator<T> {
        TrieIterator::new(self)
    }
}

pub(crate) enum TrimResult<T> {
    Found(T),
    Changed,
    NoChange,
    Err,
}

pub struct TrieIterator<'a, T> {
    keys: &'a str,
    stack: Vec<(&'a TrieNode<T>, String)>,
}

impl<'a, T> TrieIterator<'a, T> {
    fn new(trie: &'a Trie<T>) -> Self {
        TrieIterator {
            stack: vec![(&trie.root, String::new())],
            keys: trie.keys.as_str(),
        }
    }
}

impl<'a, T> Iterator for TrieIterator<'a, T> {
    type Item = (String, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.stack.pop();
        while node.is_some() {
            match node.unwrap() {
                (TrieNode::Leaf(val), path) => {
                    return Some((path, val));
                }
                (TrieNode::Node(list), path) => {
                    let old_stack = mem::take(&mut self.stack);
                    let new_stack = list
                        .iter()
                        .enumerate()
                        .map(|(i, child)| {
                            let mut path = path.clone();
                            path.push(self.keys.chars().nth(i).unwrap());
                            (child, path)
                        })
                        .rev();
                    self.stack = old_stack.into_iter().chain(new_stack).collect();
                }
            }
            node = self.stack.pop();
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perms_helper(trie: &Trie<i32>, perms: Vec<(&str, i32)>) {
        let trie_perms = trie.trie_to_perms();
        assert_eq!(
            trie_perms,
            perms
                .iter()
                .map(|(a, b)| (a.to_string(), b))
                .collect::<Vec<_>>()
        );
    }

    fn perms_helper_rev(trie: &Trie<i32>, perms: Vec<(&str, i32)>) {
        let trie_perms = trie.trie_to_perms_rev();
        assert_eq!(
            trie_perms,
            perms
                .iter()
                .map(|(a, b)| (a.to_string(), b))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_trie_perms() {
        let trie = Trie::new_from_vec("abc".to_string(), vec![0, 1, 2], false);
        let expected = vec![("a", 0), ("b", 1), ("c", 2)];
        perms_helper(&trie, expected);

        let trie = Trie::new_from_vec("abc".to_string(), vec![0, 1, 2, 3], false);
        let expected = vec![("aa", 0), ("ab", 1), ("b", 2), ("c", 3)];
        perms_helper(&trie, expected);

        let trie = Trie::new_from_vec("abc".to_string(), vec![0, 1, 2, 3, 4], false);
        let expected = vec![("aa", 0), ("ab", 1), ("ac", 2), ("b", 3), ("c", 4)];
        perms_helper(&trie, expected);

        let trie = Trie::new_from_vec("abc".to_string(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], false);
        let expected = vec![
            ("aaa", 0),
            ("aab", 1),
            ("ab", 2),
            ("ac", 3),
            ("ba", 4),
            ("bb", 5),
            ("bc", 6),
            ("ca", 7),
            ("cb", 8),
            ("cc", 9),
        ];
        perms_helper(&trie, expected);

        let mut trie =
            Trie::new_from_vec("abc".to_string(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], true);
        let expected = vec![
            ("aa", 0),
            ("ab", 1),
            ("ac", 2),
            ("ba", 3),
            ("bb", 4),
            ("bc", 5),
            ("ca", 6),
            ("cb", 7),
            ("cca", 8),
            ("ccb", 9),
        ];
        perms_helper(&trie, expected);

        let res = trie.trim('c');
        assert!(matches!(res, TrimResult::Changed));
        let expected = vec![("a", 6), ("b", 7), ("ca", 8), ("cb", 9)];
        perms_helper(&trie, expected);

        let res = trie.trim('c');
        assert!(matches!(res, TrimResult::Changed));
        let expected = vec![("a", 8), ("b", 9)];
        perms_helper(&trie, expected);

        let res = trie.trim('c');
        assert!(matches!(res, TrimResult::NoChange));
        let expected = vec![("a", 8), ("b", 9)];
        perms_helper(&trie, expected);

        let res = trie.trim('b');
        match res {
            TrimResult::Found(p) => {
                assert_eq!(p, &9);
            }
            _ => panic!("Expected Found"),
        }
        let res = trie.trim('b');
        assert!(matches!(res, TrimResult::Err));
    }

    fn iter_helper(trie: Trie<i32>, expected: Vec<(&str, i32)>) {
        for ((path, point), (expected_path, expected_point)) in
            trie.iter().zip(expected.into_iter())
        {
            assert_eq!(path, expected_path);
            assert_eq!(point, &expected_point);
        }
    }

    #[test]
    fn test_trie_iter() {
        let trie = Trie::new_from_vec("abc".to_string(), vec![0, 1, 2], false);
        let expected = vec![("a", 0), ("b", 1), ("c", 2)];
        iter_helper(trie, expected);

        let trie = Trie::new_from_vec("abc".to_string(), vec![0, 1, 2, 3], false);
        let expected = vec![("aa", 0), ("ab", 1), ("b", 2), ("c", 3)];
        iter_helper(trie, expected);

        let trie = Trie::new_from_vec("abc".to_string(), vec![0, 1, 2, 3], true);
        let expected = vec![("a", 0), ("b", 1), ("ca", 2), ("cb", 3)];
        iter_helper(trie, expected);

        let trie = Trie::new_from_vec("abc".to_string(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], true);
        let expected = vec![
            ("aa", 0),
            ("ab", 1),
            ("ac", 2),
            ("ba", 3),
            ("bb", 4),
            ("bc", 5),
            ("ca", 6),
            ("cb", 7),
            ("cca", 8),
            ("ccb", 9),
        ];
        iter_helper(trie, expected);
    }

    #[test]
    fn test_populate_with() {
        let keys = "abc".to_string();
        let values = vec![0, 1, 2];
        let builder = TrieBuilder::new(keys.clone(), values.len());
        let trie = builder.populate_with(true, values.into_iter(), |path, val| {
            (path.to_string(), val)
        });
        let perms = trie.trie_to_perms();
        assert_eq!(
            perms,
            vec![
                ("a".to_string(), &("a".to_string(), 0)),
                ("b".to_string(), &("b".to_string(), 1)),
                ("c".to_string(), &("c".to_string(), 2)),
            ]
        );

        let values = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let builder = TrieBuilder::new(keys, values.len());
        let trie = builder.populate_with(true, values.into_iter(), |path, val| {
            (path.to_string(), val)
        });
        let perms = trie.trie_to_perms();
        assert_eq!(
            perms,
            vec![
                ("aa".to_string(), &("aa".to_string(), 0)),
                ("ab".to_string(), &("ab".to_string(), 1)),
                ("ac".to_string(), &("ac".to_string(), 2)),
                ("ba".to_string(), &("ba".to_string(), 3)),
                ("bb".to_string(), &("bb".to_string(), 4)),
                ("bc".to_string(), &("bc".to_string(), 5)),
                ("ca".to_string(), &("ca".to_string(), 6)),
                ("cb".to_string(), &("cb".to_string(), 7)),
                ("cca".to_string(), &("cca".to_string(), 8)),
                ("ccb".to_string(), &("ccb".to_string(), 9)),
            ]
        );
    }
}
