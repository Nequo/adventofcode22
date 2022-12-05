fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

// Converts a string "1-4" into [1, 2, 3, 4]
// This is needed only if we actually need to know about items in the range.
// Both parts could be solved with just parsing the first and last item in the range
// but I didn't know what part 2 would hold until I got there :)
fn expand_range(range: &str) -> Vec<usize> {
    let (start, end) = range.split_once('-').unwrap();
    let start = start.parse::<usize>().unwrap();
    let end = end.parse::<usize>().unwrap();
    (start..end+1).collect()
}

fn check_overlap(a: Vec<usize>, b: Vec<usize>) -> bool {
    (a[0] <= b[0] && a[a.len()-1] >= b[b.len()-1]) ||
    (b[0] <= a[0] && b[b.len()-1] >= a[a.len()-1])
}

fn check_partial_overlap(a: Vec<usize>, b: Vec<usize>) -> bool {
    (a[a.len()-1] >= b[0] && a[a.len()-1] <= b[b.len()-1]) ||
    (b[b.len()-1] >= a[0] && b[b.len()-1] <= a[a.len()-1])
}


// fn check_partial_overlap(a: Vec<usize>, b: Vec<usize>) -> bool {
//     for n in 0..a.len() {
//         if b.contains(&a[n]) {
//             return true
//         }
//     }
//     false
// }

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines(){
        let (elf1, elf2) = line.split_once(',').unwrap();
        let elf1 = expand_range(&elf1);
        let elf2 = expand_range(&elf2);
        if check_overlap(elf1, elf2) { sum+= 1 }
    }
    sum
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines(){
        let (elf1, elf2) = line.split_once(',').unwrap();
        let elf1 = expand_range(&elf1);
        let elf2 = expand_range(&elf2);
        if check_partial_overlap(elf1, elf2) { sum+= 1 }
    }
    sum
}

fn main() {
    let input = load_input();
    println!("Part1: {:?}", part1(&input));
    println!("Part2: {:?}", part2(&input));
    //println!("Part2: {:?}", &input);
}