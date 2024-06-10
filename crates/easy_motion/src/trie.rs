use std::{fmt::Debug, mem, sync::Arc};

// Represents the matches for easy motion in a trie.
// Nodes store their leaves in an array with the index of that array corresponding
// to the character for that Node

// ex: keys: "abc", root: node { [ leaf_1, leaf_2 ] }
// would have leaves with strings of "a" and "b" respectively

// ex: keys: "abc", root: node { [ leaf_1, leaf_2, node { [leaf_3, leaf_4] } ] }
// would have leaves with strings of "a", "b", "ca", and "cb" respectively

// When new layers are necessary, the deepest layers are assigned to the latest indices first
// so the most preferred keys are kept as short as possible, which will correspond to the closest matches

// notes: There will only ever be two layers separated by one.
// Upper layer always refers to layer with a smaller depth.
// Ex: in the above leaf_count=4 example the "a" and "b" leaves are in the upper layer
// while the other two are in the lower

#[derive(Debug)]
enum TrieNode<T> {
    Leaf(T),
    Node(Vec<TrieNode<T>>),
}

impl<T: Default> Default for TrieNode<T> {
    fn default() -> Self {
        TrieNode::Leaf(Default::default())
    }
}

pub(crate) struct Trie<T> {
    keys: Arc<str>,
    root: TrieNode<T>,
    len: usize,
}

impl<T> Debug for Trie<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trie")
            .field("keys", &self.keys)
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}

impl<T> Trie<T> {
    pub fn new_from_vec<TItem, F: Fn(usize, TItem) -> T>(
        keys: Arc<str>,
        list: Vec<TItem>,
        func: F,
    ) -> Self {
        TrieBuilder::new_from_vec(keys, list, func).populate()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    // returns a list of all permutations
    #[allow(dead_code)]
    pub fn trie_to_perms(&self) -> Vec<(String, &T)> {
        let mut perms = Vec::new();
        let mut path = String::new();
        self.trie_to_perms_rec(&self.root, &mut path, &mut perms, false);
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

#[derive(Debug)]
pub(crate) enum TrimResult<T> {
    Found(T),
    Changed,
    NoChange,
    Err,
}

impl<T: Clone> TrimResult<&T> {
    pub fn cloned(&self) -> TrimResult<T> {
        match *self {
            TrimResult::Found(t) => TrimResult::Found(t.clone()),
            TrimResult::NoChange => TrimResult::NoChange,
            TrimResult::Changed => TrimResult::Changed,
            TrimResult::Err => TrimResult::Err,
        }
    }
}

fn trie_max_depth(keys_len: usize, leaf_count: usize) -> usize {
    if leaf_count > 1 {
        let max_len = f32::from(leaf_count as u16 - 1);
        let keys_len = f32::from(keys_len as u16);
        max_len.log(keys_len) as usize + 1
    } else {
        1
    }
}

/// Gives the count of leaves which will be in the upper layer
/// ex: keys: "abc", leaf_count: 4
/// a b  c
///     b a
/// => 2
fn upper_layer_count(keys_len: usize, leaf_count: usize, max_trie_depth: usize) -> usize {
    if leaf_count <= keys_len {
        return leaf_count;
    }

    // count of nodes in the previous layer
    let lower_layer_count = keys_len.pow((max_trie_depth - 1) as u32);

    // count of elements we are placing in new lowest layer
    let diff = leaf_count - lower_layer_count;

    // when we need to create the next permutation with a leaf we create a node
    // with two leaves
    // ex ... b   c     next perm    ...  b      c
    //    ...   a b c     --->       ... a b   a b c
    // extra_leaves = 2
    let extra_leaves = (diff - 1) / (keys_len - 1) + 1;

    // higher_count = diff + extra_leaves;
    // lower_count = leaf_count - higher_count;
    // simplified
    lower_layer_count - extra_leaves
}

pub(crate) struct TrieBuilder<TItem, TOut, F: Fn(usize, TItem) -> TOut> {
    keys: Arc<str>,
    list: Vec<TItem>,
    total_leaf_count: usize,
    current_leaf_count: usize,
    upper_node_count: usize,
    max_depth: usize,
    func: F,
}

impl<TItem, TOut, F: Fn(usize, TItem) -> TOut> TrieBuilder<TItem, TOut, F> {
    fn new_from_vec(keys: Arc<str>, list: Vec<TItem>, func: F) -> Self {
        let keys_len = keys.len();
        let total_leaf_count = list.len();
        let max_depth = trie_max_depth(keys_len, total_leaf_count);
        let upper_node_count = upper_layer_count(keys_len, total_leaf_count, max_depth);
        TrieBuilder {
            total_leaf_count,
            current_leaf_count: 0,
            upper_node_count,
            max_depth,
            keys,
            list,
            func,
        }
    }

    fn populate(mut self) -> Trie<TOut> {
        let iter = mem::take(&mut self.list).into_iter();
        let root = if self.total_leaf_count <= self.keys.len() {
            let (root, mut iter) = self.node_from_iter(1, iter, self.total_leaf_count);
            debug_assert!(iter.next().is_none());
            root
        } else {
            let (root, mut iter) = self.populate_rec(1, iter);
            debug_assert!(iter.next().is_none());
            root
        };
        Trie {
            root,
            keys: self.keys,
            len: self.total_leaf_count,
        }
    }

    fn populate_rec<I>(&mut self, curr_depth: usize, mut values: I) -> (TrieNode<TOut>, I)
    where
        I: Iterator<Item = TItem>,
    {
        debug_assert!(curr_depth <= self.max_depth);

        let mut new_vec = Vec::new();
        if curr_depth < self.max_depth - 1 {
            for _ in 0..self.keys.len() {
                let (new_node, new_values) = self.populate_rec(curr_depth + 1, values);
                values = new_values;
                new_vec.push(new_node);
            }
            return (TrieNode::Node(new_vec), values);
        }

        for _ in 0..self.keys.len() {
            if self.current_leaf_count < self.upper_node_count {
                new_vec.push(self.oper(curr_depth, values.next().unwrap()));
                self.current_leaf_count += 1;
                continue;
            } else if self.current_leaf_count == self.upper_node_count {
                // when the all the upper leaves have been assigned the first node will not necessarily be full
                let lower_leaf_count = self.total_leaf_count - self.upper_node_count;
                let modulo = lower_leaf_count % self.keys.len();
                let len = if modulo == 0 { self.keys.len() } else { modulo };
                let (new_node, new_values) = self.node_from_iter(curr_depth + 1, values, len);
                values = new_values;
                self.current_leaf_count += len;
                new_vec.push(new_node);
            } else {
                let (node, new_values) =
                    self.node_from_iter(curr_depth + 1, values, self.keys.len());
                new_vec.push(node);
                self.current_leaf_count += self.keys.len();
                values = new_values;
            }
        }
        (TrieNode::Node(new_vec), values)
    }

    fn node_from_iter<I>(&self, depth: usize, mut values: I, len: usize) -> (TrieNode<TOut>, I)
    where
        I: Iterator<Item = TItem>,
    {
        let mut new_vec = Vec::new();
        new_vec.reserve_exact(len);
        for _ in 0..len {
            new_vec.push(self.oper(depth, values.next().unwrap()));
        }
        (TrieNode::Node(new_vec), values)
    }

    fn oper(&self, depth: usize, val: TItem) -> TrieNode<TOut> {
        TrieNode::Leaf((self.func)(depth, val))
    }
}

pub struct TrieIterator<'a, T> {
    keys: &'a str,
    stack: Vec<(&'a TrieNode<T>, String)>,
}

impl<'a, T> TrieIterator<'a, T> {
    fn new(trie: &'a Trie<T>) -> Self {
        TrieIterator {
            stack: vec![(&trie.root, String::new())],
            keys: trie.keys.as_ref(),
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
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_lower() {
        let keys_len = 3;
        // trie: a b c
        //          a b
        let leaf_count = 4;
        let max_trie_depth = trie_max_depth(keys_len, leaf_count);
        assert_eq!(upper_layer_count(keys_len, leaf_count, max_trie_depth), 2);

        // trie: a   b      c
        //          a b   a b c
        let leaf_count = 6;
        let max_trie_depth = trie_max_depth(keys_len, leaf_count);
        assert_eq!(upper_layer_count(keys_len, leaf_count, max_trie_depth), 1);

        // trie: a    b       c
        //          a b c   a b c
        let leaf_count = 7;
        let max_trie_depth = trie_max_depth(keys_len, leaf_count);
        assert_eq!(upper_layer_count(keys_len, leaf_count, max_trie_depth), 1);

        // trie:  a      b       c
        //       a b   a b c   a b c
        let leaf_count = 8;
        let max_trie_depth = trie_max_depth(keys_len, leaf_count);
        assert_eq!(upper_layer_count(keys_len, leaf_count, max_trie_depth), 0);

        // trie:   a       b       c
        //       a b c   a b c   a b c
        let leaf_count = 9;
        let max_trie_depth = trie_max_depth(keys_len, leaf_count);
        assert_eq!(upper_layer_count(keys_len, leaf_count, max_trie_depth), 0);

        // trie:   a       b        c
        //       a b c   a b c   a  b  c
        //                            a b
        let leaf_count = 10;
        let max_trie_depth = trie_max_depth(keys_len, leaf_count);
        assert_eq!(upper_layer_count(keys_len, leaf_count, max_trie_depth), 8);

        let keys_len = 5;
        // trie:   a       b     c           d           e
        //                    a b c d    a b c d e   a b c d e
        let leaf_count = 16;
        let max_trie_depth = trie_max_depth(keys_len, leaf_count);
        assert_eq!(upper_layer_count(keys_len, leaf_count, max_trie_depth), 2);
    }

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

    pub fn new_from_vec_helper(keys: &str, list: Vec<i32>) -> Trie<i32> {
        Trie::new_from_vec(keys.into(), list, |_, val| val)
    }

    #[test]
    fn test_new_from_vec() {
        let trie = new_from_vec_helper("abc", (0..=2).collect_vec());
        let expected = vec![("a", 0), ("b", 1), ("c", 2)];
        perms_helper(&trie, expected);

        let trie = new_from_vec_helper("abc", (0..=3).collect_vec());
        let expected = vec![("a", 0), ("b", 1), ("ca", 2), ("cb", 3)];
        perms_helper(&trie, expected);

        let trie = new_from_vec_helper("abc", (0..=5).collect_vec());
        let expected = vec![
            ("a", 0),
            ("ba", 1),
            ("bb", 2),
            ("ca", 3),
            ("cb", 4),
            ("cc", 5),
        ];
        perms_helper(&trie, expected);

        let trie = new_from_vec_helper("abc", (0..=7).collect_vec());
        let expected = vec![
            ("aa", 0),
            ("ab", 1),
            ("ba", 2),
            ("bb", 3),
            ("bc", 4),
            ("ca", 5),
            ("cb", 6),
            ("cc", 7),
        ];
        perms_helper(&trie, expected);

        let trie = new_from_vec_helper("abc", (0..=8).collect_vec());
        let expected = vec![
            ("aa", 0),
            ("ab", 1),
            ("ac", 2),
            ("ba", 3),
            ("bb", 4),
            ("bc", 5),
            ("ca", 6),
            ("cb", 7),
            ("cc", 8),
        ];
        perms_helper(&trie, expected);

        let trie = new_from_vec_helper("abc", (0..=9).collect_vec());
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

        let trie = new_from_vec_helper("abc", (0..=12).collect_vec());
        let expected = vec![
            ("aa", 0),
            ("ab", 1),
            ("ac", 2),
            ("ba", 3),
            ("bb", 4),
            ("bc", 5),
            ("ca", 6),
            ("cba", 7),
            ("cbb", 8),
            ("cbc", 9),
            ("cca", 10),
            ("ccb", 11),
            ("ccc", 12),
        ];
        perms_helper(&trie, expected);

        // trie:   a       b     c           d           e
        //                    a b c d    a b c d e   a b c d e
        let trie = new_from_vec_helper("abcde", (0..=15).collect_vec());
        let expected = vec![
            ("a", 0),
            ("b", 1),
            ("ca", 2),
            ("cb", 3),
            ("cc", 4),
            ("cd", 5),
            ("da", 6),
            ("db", 7),
            ("dc", 8),
            ("dd", 9),
            ("de", 10),
            ("ea", 11),
            ("eb", 12),
            ("ec", 13),
            ("ed", 14),
            ("ee", 15),
        ];
        perms_helper(&trie, expected);
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
        let trie = new_from_vec_helper("abc".into(), vec![0, 1, 2]);
        let expected = vec![("a", 0), ("b", 1), ("c", 2)];
        iter_helper(trie, expected);

        let trie = new_from_vec_helper("abc".into(), vec![0, 1, 2, 3]);
        let expected = vec![("a", 0), ("b", 1), ("ca", 2), ("cb", 3)];
        iter_helper(trie, expected);

        let trie = new_from_vec_helper("abc".into(), vec![0, 1, 2, 3, 4, 5]);
        let expected = vec![
            ("a", 0),
            ("ba", 1),
            ("bb", 2),
            ("ca", 3),
            ("cb", 4),
            ("cc", 5),
        ];
        iter_helper(trie, expected);

        let trie = new_from_vec_helper("abc".into(), vec![0, 1, 2, 3, 4, 5, 6]);
        let expected = vec![
            ("a", 0),
            ("ba", 1),
            ("bb", 2),
            ("bc", 3),
            ("ca", 4),
            ("cb", 5),
            ("cc", 6),
        ];
        iter_helper(trie, expected);

        let trie = new_from_vec_helper("abc".into(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let expected = vec![
            ("aa", 0),
            ("ab", 1),
            ("ac", 2),
            ("ba", 3),
            ("bb", 4),
            ("bc", 5),
            ("ca", 6),
            ("cb", 7),
            ("cc", 8),
        ];
        iter_helper(trie, expected);

        let trie = new_from_vec_helper("abc".into(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
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

        // trie:   a       b     c           d           e
        //                    a b c d    a b c d e   a b c d e
        let trie = new_from_vec_helper(
            "abcde".into(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        );
        let expected = vec![
            ("a", 0),
            ("b", 1),
            ("ca", 2),
            ("cb", 3),
            ("cc", 4),
            ("cd", 5),
            ("da", 6),
            ("db", 7),
            ("dc", 8),
            ("dd", 9),
            ("de", 10),
            ("ea", 11),
            ("eb", 12),
            ("ec", 13),
            ("ed", 14),
            ("ee", 15),
        ];
        iter_helper(trie, expected);
    }

    #[test]
    fn test_populate_with() {
        let keys: Arc<str> = "abc".into();
        let values = vec![0, 1, 2];
        let trie = Trie::new_from_vec(keys.clone(), values, |len, val| (len, val));
        let perms = trie.trie_to_perms();
        assert_eq!(
            perms,
            vec![
                ("a".to_string(), &(1, 0)),
                ("b".to_string(), &(1, 1)),
                ("c".to_string(), &(1, 2)),
            ]
        );

        let values = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let trie = Trie::new_from_vec(keys.clone(), values, |len, val| (len, val));
        let perms = trie.trie_to_perms();
        assert_eq!(
            perms,
            vec![
                ("aa".to_string(), &(2, 0)),
                ("ab".to_string(), &(2, 1)),
                ("ac".to_string(), &(2, 2)),
                ("ba".to_string(), &(2, 3)),
                ("bb".to_string(), &(2, 4)),
                ("bc".to_string(), &(2, 5)),
                ("ca".to_string(), &(2, 6)),
                ("cb".to_string(), &(2, 7)),
                ("cca".to_string(), &(3, 8)),
                ("ccb".to_string(), &(3, 9)),
            ]
        );
    }
}
