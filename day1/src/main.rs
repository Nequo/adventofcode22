fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn add_consecutive_lines(input: &str) -> Vec<u32> {
    let mut calcounts: Vec<u32> = Vec::new();
    let mut current_calories = 0;
    // File hosts must exist in current path before this produces output
    for line in input.lines() {
        if line.is_empty() { 
            calcounts.push(current_calories);
            current_calories = 0; 
        }
        else { current_calories += line.parse::<u32>().unwrap()}
    }
    calcounts.sort();
    calcounts.reverse();
    calcounts
}

fn main() {
    let input = load_input();
    let calcounts = add_consecutive_lines(&input);

    println!("Part1: {:?}", calcounts.iter().max().unwrap());
    println!("Part2: {:?}", calcounts[..3].iter().sum::<u32>());
}
