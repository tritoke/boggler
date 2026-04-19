use std::path::PathBuf;
use std::process::Command;
use trie_rs::TrieBuilder;

fn main() {
    let aspell_dump = Command::new("aspell")
        .arg("dump")
        .arg("master")
        .output()
        .expect("Failed to run aspell, is aspell installed?");
    let words = String::from_utf8(aspell_dump.stdout)
        .expect("Failed to decode aspell dictionary, invalid UTF8?");
    let mut builder = TrieBuilder::new();
    let mut word = String::new();
    for dict_word in words.lines() {
        for mut c in dict_word.chars() {
            c = c.to_ascii_uppercase();
            if 'A' <= c && c <= 'Z' {
                word.push(c);
            }
        }

        if word.len() >= 3 {
            builder.push(&word);
        }
        word.clear();
    }

    let trie = builder.build();
    let trie_data = postcard::to_allocvec(&trie).expect("Failed to serialise trie");
    let outfile: PathBuf = [std::env::var("OUT_DIR").unwrap().as_str(), "trie.postcard"].iter().collect();
    std::fs::write(outfile, trie_data).expect("Failed to write serialised trie data");
}
