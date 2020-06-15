use trie_rs;

fn main() {
  let mut m = trie_rs::TrieTree::new();
  let mut word_list = vec![];
  for word in ["noe", "no", "n", "na"].iter() {
    let ch_vec: Vec<char> = word.chars().collect();
    word_list.push(ch_vec);
  }
  for word in word_list.iter() {
    m.insert(word);
  }
  let r = m.prefix_match(&word_list[2]);
  println!("{:?}", r);
  // --------------
  // for word in ["noe", "aoe"].iter() {
  //   let ch_vec: Vec<char> = word.chars().collect();
  //   word_list.push(ch_vec);
  //   // word_list.push(vec![]);
  // }
  // // test
  // let mut insert_string_record = vec![];
  // insert_string_record.push(format!("{:#?}", m));
  // for word in word_list.iter() {
  //   m.insert(word);
  //   insert_string_record.push(format!("{:#?}", m));
  //   m.insert(word);
  //   insert_string_record.push(format!("{:#?}", m));
  // }
  // let mut remove_string_record = vec![];
  // remove_string_record.push(format!("{:#?}", m));
  // for word in word_list.iter().rev() {
  //   m.remove(&word[..2]);
  //   m.remove(word);
  //   remove_string_record.push(format!("{:#?}", m));
  //   m.remove(word);
  //   remove_string_record.push(format!("{:#?}", m));
  // }
  // remove_string_record = remove_string_record.into_iter().rev().collect();
  // for i in 0..insert_string_record.len() {
  //   println!("{}", insert_string_record[i] == remove_string_record[i]);
  // }
}
