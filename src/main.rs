use trie_rs;

fn main() {
    let mut m = trie_rs::Tree::new();
    m.insert(1);
    println!("{:#?}", m);
}
