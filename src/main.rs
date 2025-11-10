use std::env;

fn are_anagrams(a: &str, b: &str) -> bool {
    let mut chars_a: Vec<char> = a.chars().collect();
    let mut chars_b: Vec<char> = b.chars().collect();
    chars_a.sort_unstable();
    chars_b.sort_unstable();
    chars_a == chars_b
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} word1 word2", args[0]);
        return;
    }
    let word1 = &args[1];
    let word2 = &args[2];
    if are_anagrams(word1, word2) {
        println!("\"{}\" and \"{}\" are anagrams!", word1, word2);
    } else {
        println!("\"{}\" and \"{}\" are NOT anagrams.", word1, word2);
    }
}
