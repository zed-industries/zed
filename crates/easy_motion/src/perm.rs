#[derive(Default, Debug)]
enum TrieNode {
    #[default]
    Leaf,
    Node(Vec<TrieNode>),
}

impl TrieNode {
    pub fn insert(&mut self) -> &mut Self {
        match self {
            Self::Leaf => {
                *self = Self::Node(vec![TrieNode::Leaf]);
            }
            Self::Node(hash_map) => {
                hash_map.push(TrieNode::Leaf);
            }
        }
        self
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Leaf => 0,
            Self::Node(hash_map) => hash_map.len(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Trie {
    keys: String,
    root: TrieNode,
}

impl Trie {
    pub fn new(keys: String, len: usize) -> Self {
        let mut trie = Trie {
            keys,
            root: TrieNode::default(),
        };
        let mut p = Vec::new();
        for _ in 0..len {
            p = trie.next_perm(p);
        }
        trie
    }

    fn next_perm(&mut self, pointer: Vec<usize>) -> Vec<usize> {
        match self.root {
            TrieNode::Leaf => {
                self.root = TrieNode::Node(vec![TrieNode::Leaf]);
                vec![0]
            }
            TrieNode::Node(_) => self.next_perm_internal(pointer),
        }
    }

    fn next_perm_internal(&mut self, mut pointer: Vec<usize>) -> Vec<usize> {
        let max_len = self.keys.len();
        let len = pointer.len();
        let last_index = pointer.last().unwrap().clone();
        if last_index < max_len - 1 {
            pointer[len - 1] += 1;
            // get parent to current node
            let node = self.pointer_to_node(&pointer[0..len - 1]);
            node.insert();
            return pointer;
        } else {
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
                // "a" -> "aa", "ab"
                node.insert();
                node.insert();
                return pointer;
            }
        }

        // current layer is completely full so we need to add a new layer
        pointer.fill(0);
        pointer.push(1);
        let node = self.pointer_to_node(&pointer[0..len]);
        node.insert();
        node.insert();
        pointer
    }

    fn pointer_to_node(&mut self, pointer: &[usize]) -> &mut TrieNode {
        if pointer.is_empty() {
            return &mut self.root;
        }
        let mut node = &mut self.root;
        for i in pointer {
            node = match node {
                TrieNode::Leaf => return node,
                TrieNode::Node(list) => list.get_mut(i.clone()).unwrap(),
            };
        }
        node
    }

    pub fn trie_to_perms(&self) -> Vec<String> {
        let mut perms = Vec::new();
        let mut path = String::new();
        self.trie_to_perms_rec(&self.root, &mut path, &mut perms, false);
        return perms;
    }

    pub fn trie_to_perms_rev(&self) -> Vec<String> {
        let mut perms = Vec::new();
        let mut path = String::new();
        self.trie_to_perms_rec(&self.root, &mut path, &mut perms, true);
        return perms;
    }

    fn trie_to_perms_rec(
        &self,
        node: &TrieNode,
        path: &mut String,
        perms: &mut Vec<String>,
        reverse: bool,
    ) {
        match node {
            TrieNode::Leaf => {
                perms.push(path.clone());
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
        perms: &mut Vec<String>,
        reverse: bool,
    ) {
        let character = self.keys.chars().nth(i).unwrap();
        path.push(character);
        self.trie_to_perms_rec(node, path, perms, reverse);
        path.pop();
    }

    fn trim(&mut self, character: char) -> Option<&TrieNode> {
        let node = match &mut self.root {
            TrieNode::Leaf => {
                return None;
            }
            TrieNode::Node(ref mut map) => {
                let index = self.keys.find(character).unwrap();
                dbg!(index);
                map.swap_remove(index)
            }
        };
        self.root = node;
        Some(&self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_perms() {
        let mut trie = Trie::new("abc".to_string(), 3);
        assert_eq!(trie.trie_to_perms(), vec!["a", "b", "c"]);
        trie = Trie::new("abc".to_string(), 4);
        assert_eq!(trie.trie_to_perms(), vec!["aa", "ab", "b", "c"]);
        trie = Trie::new("abc".to_string(), 5);
        assert_eq!(trie.trie_to_perms(), vec!["aa", "ab", "ac", "b", "c"]);
        trie = Trie::new("abc".to_string(), 6);
        assert_eq!(
            trie.trie_to_perms(),
            vec!["aa", "ab", "ac", "ba", "bb", "c"]
        );
        trie = Trie::new("abc".to_string(), 9);
        assert_eq!(
            trie.trie_to_perms(),
            vec!["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]
        );
        trie = Trie::new("abc".to_string(), 10);
        assert_eq!(
            trie.trie_to_perms(),
            vec!["aaa", "aab", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]
        );
        trie = Trie::new("abc".to_string(), 10);
        assert_eq!(
            trie.trie_to_perms_rev(),
            vec!["cc", "cb", "ca", "bc", "bb", "ba", "ac", "ab", "aab", "aaa"]
        );

        let res = trie.trim('a');
        assert!(res.is_some());
        assert_eq!(trie.trie_to_perms_rev(), vec!["c", "b", "ab", "aa"]);
        let res = trie.trim('a');
        assert!(res.is_some());
        assert_eq!(trie.trie_to_perms_rev(), vec!["b", "a"]);
        let res = trie.trim('b');
        assert!(matches!(res, Some(&TrieNode::Leaf)));
        let res = trie.trim('b');
        assert!(matches!(res, None));
    }
}
