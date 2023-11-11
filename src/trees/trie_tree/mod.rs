use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

pub struct TrieTree {
    root: TrieNode,
}

impl TrieTree {
    pub fn new() -> Self {
        TrieTree {
            root: TrieNode::new(),
        }
    }
}

impl TrieTree {
    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_insert_with(|| Box::new(TrieNode::new()));
        }
        current_node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for ch in word.chars() {
            match current_node.children.get(&ch) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.is_end_of_word
    }
}