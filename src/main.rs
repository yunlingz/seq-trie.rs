use seq_trie;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let wrong_msg = "usage: exec [file_path]";
    let file = match args.get(1usize) {
        Some(v) => match fs::File::open(v) {
            Ok(file) => file,
            Err(_) => panic!(wrong_msg),
        },
        None => panic!(wrong_msg),
    };
    let lines: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .filter(|l| l.is_ok())
        .map(|l| {
            l.unwrap()
                .to_lowercase()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect()
        })
        .collect();
    // println!("{:#?}", lines);

    let mut trie = seq_trie::TrieTree::new();
    trie.insert_iterator(lines.into_iter());
    // println!("{:#?}", trie);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");
        let seq: Vec<char> = input.trim().chars().collect();
        const MAX_MATCH: usize = 10;
        if let Some(all_results) = trie.prefix_match_maxn(&seq, MAX_MATCH) {
            println!("┌────────────────");
            for (i, result) in all_results.into_iter().enumerate() {
                println!("│ {}. {}", i + 1, result.into_iter().collect::<String>());
            }
            println!("└────────────────\n");
        } else {
            println!("┌────────────────");
            println!("│ NO MATCH");
            println!("└────────────────\n");
        }
    }
}
