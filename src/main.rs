use bungos_algortimers::trees::trie_tree::TrieTree;

fn main() {
    let mut trie_tree = TrieTree::new();
    trie_tree.insert("hello");
    trie_tree.insert("world");
    trie_tree.insert("hell");
    trie_tree.insert("he");
    trie_tree.insert("h");
    println!("hello: {}", trie_tree.search("hello"));
    println!("world: {}", trie_tree.search("world"));
    println!("hell: {}", trie_tree.search("hell"));
    println!("he: {}", trie_tree.search("he"));
    println!("h: {}", trie_tree.search("h"));
    println!("helloo: {}", trie_tree.search("helloo"));
    println!("worldd: {}", trie_tree.search("worldd"));
    println!("hel: {}", trie_tree.search("hel"));
    println!("h: {}", trie_tree.search("h"));
    println!("hh: {}", trie_tree.search("hh"));
    println!("hhe: {}", trie_tree.search("hhe"));
    println!("hhello: {}", trie_tree.search("hhello"));
    println!("hhelloo: {}", trie_tree.search("hhelloo"));
    println!("hhellooo: {}", trie_tree.search("hhellooo"));
    println!("hhelloooo: {}", trie_tree.search("hhelloooo"));
    println!("lo: {}", trie_tree.search("lo"));
}