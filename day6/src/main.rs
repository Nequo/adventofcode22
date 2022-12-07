fn find_first_non_repeating_token(token_len: usize, input: &str) -> usize {
    let cs: Vec<char> = input.chars().collect();
    // Tried to do a tokenizer reading each character and looking ahead n characters.
    // This approach wastes a lot of memory creating a vec then sorting and deduping.
    for i in 0..cs.len()-token_len {
        let mut token: Vec<char> = Vec::new();
        for x in 0..token_len {
            token.push(cs[i+x]);
        }
        token.sort();
        token.dedup();
        if token.len() == token_len {
            return i+token_len
        }
    }
    0
}

fn main() {
    let input = include_str!("../input");
    println!("Part1: {:?}", find_first_non_repeating_token(4, &input));
    println!("Part2: {:?}", find_first_non_repeating_token(14, &input));
}
