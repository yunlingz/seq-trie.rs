use trie_rs;

fn main() {
  let mut m = trie_rs::TrieTree::new();
  m.insert("name");
  m.insert("nam");
  m.insert("nati");
  m.insert("nami");
  m.insert("我们");
  println!("{:#?}", m);
  println!("{:#?}", m.contains("nam"));
}
