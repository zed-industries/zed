use editor::DisplayPoint;
use std::slice::Iter;

#[derive(Debug)]
enum TrieNode {
    Leaf(DisplayPoint),
    Node(Vec<TrieNode>),
}

impl TrieNode {
    pub fn insert(&mut self, point: DisplayPoint) -> &mut Self {
        match self {
            Self::Leaf(old_point) => {
                *self = Self::Node(vec![
                    TrieNode::Leaf(old_point.to_owned()),
                    TrieNode::Leaf(point),
                ]);
            }
            Self::Node(hash_map) => {
                hash_map.push(TrieNode::Leaf(point));
            }
        }
        self
    }

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

#[derive(Debug)]
pub(crate) struct Trie {
    keys: String,
    root: TrieNode,
}

impl Trie {
    pub fn new(keys: String, points: Vec<DisplayPoint>, reverse: bool) -> Self {
        let root = TrieNode::Node(vec![TrieNode::Leaf(DisplayPoint::default())]);
        let mut trie = Trie { keys, root };
        let mut p = vec![0];
        // constructs the trie
        for _ in 1..points.len() {
            p = trie.next_perm(p, DisplayPoint::default());
        }
        // replace the points with the actual points
        (trie.root, _) = Trie::new_rec(trie.root, points.iter(), reverse);
        trie
    }

    fn next_perm(&mut self, pointer: Vec<usize>, point: DisplayPoint) -> Vec<usize> {
        self.next_perm_internal(pointer, point)
    }

    // reverse puts the deeper nodes last
    // i.e. aa ab b c -> a b ca cb
    fn new_rec(
        node: TrieNode,
        mut iter: Iter<DisplayPoint>,
        reverse: bool,
    ) -> (TrieNode, Iter<DisplayPoint>) {
        let node = match node {
            TrieNode::Leaf(_) => {
                let point = iter.next().unwrap().to_owned();
                TrieNode::Leaf(point)
            }
            TrieNode::Node(mut list) => {
                let mut ret = Vec::<TrieNode>::new();
                for _ in 0..list.len() {
                    let mut child = if reverse {
                        list.pop().unwrap()
                    } else {
                        list.remove(0)
                    };
                    (child, iter) = Trie::new_rec(child, iter, reverse);
                    ret.push(child);
                }
                TrieNode::Node(ret)
            }
        };
        (node, iter)
    }

    fn next_perm_internal(&mut self, mut pointer: Vec<usize>, point: DisplayPoint) -> Vec<usize> {
        let max_len = self.keys.len();
        let len = pointer.len();
        let last_index = pointer.last().unwrap().clone();
        if last_index < max_len - 1 {
            pointer[len - 1] += 1;
            // get parent to current node
            let node = self.pointer_to_node(&pointer[0..len - 1]);
            debug_assert!(matches!(node, TrieNode::Node(_)));
            node.insert(point);
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
            let node = self.pointer_to_node(&pointer[0..len - 1]);
            debug_assert!(matches!(node, TrieNode::Leaf(_)));
            node.insert(point);
            return pointer;
        }

        // current layer is completely full so we need to add a new layer
        pointer.fill(0);
        pointer.push(1);
        let node = self.pointer_to_node(&pointer[0..len]);
        debug_assert!(matches!(node, TrieNode::Leaf(_)));
        node.insert(point);
        pointer
    }

    fn pointer_to_node(&mut self, pointer: &[usize]) -> &mut TrieNode {
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

    #[allow(dead_code)]
    pub fn for_each(&self, func: impl Fn(&str, &DisplayPoint)) {
        let mut path = String::new();
        self.for_each_rec(&self.root, &mut path, false, &func);
    }

    #[allow(dead_code)]
    pub fn for_each_reverse(&self, func: impl Fn(&str, &DisplayPoint)) {
        let mut path = String::new();
        self.for_each_rec(&self.root, &mut path, true, &func);
    }

    fn for_each_rec(
        &self,
        node: &TrieNode,
        path: &mut String,
        reverse: bool,
        func: &impl Fn(&str, &DisplayPoint),
    ) {
        match node {
            TrieNode::Leaf(point) => func(path, point),
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
        node: &TrieNode,
        path: &mut String,
        reverse: bool,
        func: &impl Fn(&str, &DisplayPoint),
    ) {
        let character = self.keys.chars().nth(i).unwrap();
        path.push(character);
        self.for_each_rec(node, path, reverse, func);
        path.pop();
    }

    // returns a list of all permutations
    pub fn trie_to_perms(&self) -> Vec<(String, DisplayPoint)> {
        let mut perms = Vec::new();
        let mut path = String::new();
        self.trie_to_perms_rec(&self.root, &mut path, &mut perms, false);
        return perms;
    }

    // returns a list of all permutations with the indices reversed
    // i.e. a b ca cb -> c b ac ab
    #[allow(dead_code)]
    pub fn trie_to_perms_rev(&self) -> Vec<(String, DisplayPoint)> {
        let mut perms = Vec::new();
        let mut path = String::new();
        self.trie_to_perms_rec(&self.root, &mut path, &mut perms, true);
        return perms;
    }

    fn trie_to_perms_rec(
        &self,
        node: &TrieNode,
        path: &mut String,
        perms: &mut Vec<(String, DisplayPoint)>,
        reverse: bool,
    ) {
        match node {
            TrieNode::Leaf(point) => {
                perms.push((path.clone(), point.clone()));
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

    fn trie_to_perms_rec_loop(
        &self,
        i: usize,
        node: &TrieNode,
        path: &mut String,
        perms: &mut Vec<(String, DisplayPoint)>,
        reverse: bool,
    ) {
        let character = self.keys.chars().nth(i).unwrap();
        path.push(character);
        self.trie_to_perms_rec(node, path, perms, reverse);
        path.pop();
    }

    pub fn trim(&mut self, character: char) -> TrimResult {
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
            TrieNode::Leaf(point) => TrimResult::Found(point.clone()),
            TrieNode::Node(_) => TrimResult::Changed,
        }
    }
}

pub(crate) enum TrimResult {
    Found(DisplayPoint),
    Changed,
    NoChange,
    Err,
}

impl<'a> IntoIterator for &'a Trie {
    type Item = (String, DisplayPoint);
    type IntoIter = TrieIterator<'a>;

    fn into_iter(self) -> TrieIterator<'a> {
        TrieIterator::new(self)
    }
}

pub struct TrieIterator<'a> {
    path: String,
    stack: Vec<&'a TrieNode>,
}

impl<'a> TrieIterator<'a> {
    fn new(trie: &'a Trie) -> Self {
        TrieIterator {
            stack: vec![&trie.root],
            path: String::new(),
        }
    }
}

impl<'a> Iterator for TrieIterator<'a> {
    type Item = (String, DisplayPoint);

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.stack.pop();
        while node.is_some() {
            match node.unwrap() {
                TrieNode::Leaf(point) => {
                    return Some((self.path.clone(), point.clone()));
                }
                TrieNode::Node(list) => {
                    for child in list {
                        self.stack.push(child);
                    }
                }
            }
            node = self.stack.pop();
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use editor::display_map::DisplayRow;

    use super::*;

    fn point(x: u32) -> DisplayPoint {
        DisplayPoint::new(DisplayRow(x), x)
    }

    #[test]
    fn test_trie_perms() {
        let mut trie = Trie::new("abc".to_string(), vec![point(0), point(1), point(2)], false);
        assert_eq!(
            trie.trie_to_perms(),
            vec![
                ("a".to_string(), point(0)),
                ("b".to_string(), point(1)),
                ("c".to_string(), point(2))
            ]
        );
        trie = Trie::new(
            "abc".to_string(),
            vec![point(0), point(1), point(2), point(3)],
            false,
        );
        assert_eq!(
            trie.trie_to_perms(),
            vec![
                ("aa".to_string(), point(0)),
                ("ab".to_string(), point(1)),
                ("b".to_string(), point(2)),
                ("c".to_string(), point(3))
            ]
        );
        trie = Trie::new(
            "abc".to_string(),
            vec![point(0), point(1), point(2), point(3), point(4)],
            false,
        );
        assert_eq!(
            trie.trie_to_perms(),
            vec![
                ("aa".to_string(), point(0)),
                ("ab".to_string(), point(1)),
                ("ac".to_string(), point(2)),
                ("b".to_string(), point(3)),
                ("c".to_string(), point(4))
            ]
        );
        trie = Trie::new(
            "abc".to_string(),
            vec![
                point(0),
                point(1),
                point(2),
                point(3),
                point(4),
                point(5),
                point(6),
                point(7),
                point(8),
                point(9),
            ],
            false,
        );
        assert_eq!(
            trie.trie_to_perms(),
            vec![
                ("aaa".to_string(), point(0)),
                ("aab".to_string(), point(1)),
                ("ab".to_string(), point(2)),
                ("ac".to_string(), point(3)),
                ("ba".to_string(), point(4)),
                ("bb".to_string(), point(5)),
                ("bc".to_string(), point(6)),
                ("ca".to_string(), point(7)),
                ("cb".to_string(), point(8)),
                ("cc".to_string(), point(9))
            ]
        );
        trie = Trie::new(
            "abc".to_string(),
            vec![
                point(0),
                point(1),
                point(2),
                point(3),
                point(4),
                point(5),
                point(6),
                point(7),
                point(8),
                point(9),
            ],
            true,
        );
        assert_eq!(
            trie.trie_to_perms(),
            vec![
                ("aa".to_string(), point(0)),
                ("ab".to_string(), point(1)),
                ("ac".to_string(), point(2)),
                ("ba".to_string(), point(3)),
                ("bb".to_string(), point(4)),
                ("bc".to_string(), point(5)),
                ("ca".to_string(), point(6)),
                ("cb".to_string(), point(7)),
                ("cca".to_string(), point(8)),
                ("ccb".to_string(), point(9))
            ]
        );

        let res = trie.trim('c');
        assert!(matches!(res, TrimResult::Changed));
        assert_eq!(
            trie.trie_to_perms(),
            vec![
                ("a".to_string(), point(6)),
                ("b".to_string(), point(7)),
                ("ca".to_string(), point(8)),
                ("cb".to_string(), point(9))
            ]
        );
        let res = trie.trim('c');
        assert!(matches!(res, TrimResult::Changed));
        assert_eq!(
            trie.trie_to_perms(),
            vec![("a".to_string(), point(8)), ("b".to_string(), point(9))]
        );
        let res = trie.trim('c');
        assert!(matches!(res, TrimResult::NoChange));
        assert_eq!(
            trie.trie_to_perms(),
            vec![("a".to_string(), point(8)), ("b".to_string(), point(9))]
        );
        let res = trie.trim('b');
        match res {
            TrimResult::Found(p) => {
                assert_eq!(p, point(9));
            }
            _ => panic!("Expected Found"),
        }
        let res = trie.trim('b');
        assert!(matches!(res, TrimResult::Err));
    }
}
