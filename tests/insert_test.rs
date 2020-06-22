use seq_trie;

#[test]
fn test_insert_0() {
  let mut m = seq_trie::TrieTree::new();
  let seq_dict = ["na", "no", "nie", "nte"];
  let seq_dict: Vec<Vec<char>> =
    seq_dict.iter().map(|seq| seq.chars().collect()).collect();
  m.insert_collection(seq_dict.clone().into_iter());
  for seq in seq_dict.iter() {
    assert!(m.contains(seq));
  }
}

#[test]
fn test_insert_1() {
  let mut m = seq_trie::TrieTree::new();
  let seq_dict = ["na", "no", "nie", "nte"];
  let seq_dict: Vec<Vec<char>> =
    seq_dict.iter().map(|seq| seq.chars().collect()).collect();
  m.insert_collection(seq_dict.clone().into_iter());
  for seq in seq_dict.iter() {
    assert!(m.contains(seq));
  }
}
