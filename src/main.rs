use trie_rs;

fn main() {
    let mut m = trie_rs::TrieTree::new();
    let mut word_list = vec![];
    for word in ["name", "time"].iter() {
        let ch_vec: Vec<char> = word.chars().collect();
        word_list.push(ch_vec);
    }
    for word in word_list.iter() {
        m.insert(word);
    }
    println!("{:#?}", m);
    println!("{:#?}", m.contains(&word_list[0][..3]));
}
