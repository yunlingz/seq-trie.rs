use trie_rs;

fn main() {
  let mut m = trie_rs::TrieTree::new();
  println!("{:#?}", m);
  println!("-------------");
  let mut word_list = vec![];
  for word in ["noe", "no"].iter() {
    let ch_vec: Vec<char> = word.chars().collect();
    word_list.push(ch_vec);
  }
  for word in word_list.iter() {
    m.insert(word);
    println!("{:#?}", m);
  }
  println!("-------------");
  // // let tmp: [char; 0] = [];
  // // m.insert(&tmp);
  // println!("{:#?}", m);
  // println!("{:#?}", m.contains(&word_list[0][..3]));
  for word in word_list.iter() {
    m.remove(word);
    println!("{:#?}", m);
  }
}
