fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn calculate_score(input: &str) -> i32 {
    let mut score: i32 = 0;
    for line in input.lines() {
        match line {
            "A X" => score += 1 + 3,
            "A Y" => score += 2 + 6,
            "A Z" => score += 3,
            "B X" => score += 1,
            "B Y" => score += 2 + 3,
            "B Z" => score += 3 + 6,
            "C X" => score += 1 + 6,
            "C Y" => score += 2,
            "C Z" => score += 3 + 3,
             _    => score += 0,
        }
    }
    score
}

fn calculate_score1(input: &str) -> i32 {
    let mut score: i32 = 0;
    for line in input.lines() {
        match line {
            // lose
            "A X" => score += 0 + 3,
            "B X" => score += 0 + 1,
            "C X" => score += 0 + 2,
            // draw
            "A Y" => score += 3 + 1,
            "B Y" => score += 3 + 2,
            "C Y" => score += 3 + 3,
            // win
            "A Z" => score += 6 + 2,
            "B Z" => score += 6 + 3,
            "C Z" => score += 6 + 1,
            // should probably error out
            // but currently just ignore
             _    => score += 0,
        }
    }
    score
}

fn main() {
    let input = load_input();
    println!("{}", calculate_score(&input));
    println!("{}", calculate_score1(&input));
}
