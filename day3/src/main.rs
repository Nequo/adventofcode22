use itertools::Itertools;

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn char_to_priority(c: char) -> usize {
    match c {
        'a'..='z' => {
            c as usize - 96
        }
        'A'..='Z' => {
            c as usize - 38
        }
        _ => 0
    }
}

fn find_repeated_type(l: String) -> char  {
    let (compartment1, compartment2) = l.split_at(l.len()/2);
    for c in compartment1.chars().into_iter() {
        if compartment2.contains(c){
            return c
        }
    }
    '_'
}

fn sum_of_priorities(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += char_to_priority(find_repeated_type(line.to_string()));
    }
    sum
}

// TODO: find better way of iterating over lines 3 at a time
fn sum_of_priorities2(input: &str) -> usize {
    let mut sum = 0;
    for mut chunk in &input.lines().chunks(3){
        let l1 = chunk.next().unwrap();
        let l2 = chunk.next().unwrap();
        let l3 = chunk.next().unwrap();
        for c in l1.chars().into_iter() {
            if l2.contains(c) && l3.contains(c) {
                sum += char_to_priority(c);
                break;
            }
        }
    }
    sum
}

fn main() {
    let input = load_input();
    println!("Part1: {:?}", sum_of_priorities(&input));
    println!("Part2: {:?}", sum_of_priorities2(&input));
}
