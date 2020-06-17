use seq_trie;

fn main() {
    let mut m = seq_trie::TrieTree::new();
    let mut word_list = vec![];
    // ----------------
    for word in ["na", "noe", "no", "n"].iter() {
        let ch_vec: Vec<char> = word.chars().collect();
        word_list.push(ch_vec);
    }
    for word in word_list.iter() {
        m.insert(word);
    }
    let r = m.prefix_match(&word_list[3]);
    println!("{:?}", r);
    // --------------
    // for word in ["noe", "na", "nae"].iter() {
    //     let ch_vec: Vec<char> = word.chars().collect();
    //     word_list.push(ch_vec);
    //     // word_list.push(vec![]);
    // }
    // // test
    // let mut insert_string_record = vec![];
    // insert_string_record.push(format!("{:#?}", m));
    // for word in word_list.iter() {
    //     m.insert(word);
    //     insert_string_record.push(format!("{:#?}", m));
    //     println!("{:#?}", m);
    // }
    // println!("--------------");
    // let mut remove_string_record = vec![];
    // remove_string_record.push(format!("{:#?}", m));
    // for word in word_list.iter().rev() {
    //     m.remove(word);
    //     remove_string_record.push(format!("{:#?}", m));
    //     println!("{:#?}", m);
    // }
    // remove_string_record = remove_string_record.into_iter().rev().collect();
    // for i in 0..insert_string_record.len() {
    //     // println!("{}", insert_string_record[i] == remove_string_record[i]);
    // }
}
