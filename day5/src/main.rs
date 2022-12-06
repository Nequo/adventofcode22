fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn crates_ascii2vec(crates: &str) -> Vec<Vec<char>> {
    let mut r: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        r.push(Vec::new());
    }
    for l in crates.lines().rev() {
        if l.starts_with(" 1") { continue }
        // This makes the string split nicely with a space if there is no crate
        let newl = l.replace("    ", " ");
        let v: Vec<_> = newl.split(" ").collect();
        println!("{:?}", v);
        //["[Z]", "[M]", "[P]"]
        //["[N]", "[C]", ""]
        //["", "[D]", ""]
        for i in 0..(v.len()) {
            if v[i] != "" {
                // Convoluted way of getting the letter N out of [N]
                let letter = v[i].as_bytes()[1];
                r[i].push(letter as char)
            }
        }
    }
    r
}

fn process_moves(crates: &mut Vec<Vec<char>>, moves: &str) {
    for l in moves.lines() {
        let parts: Vec<&str> = l.split(" ").collect();
        let num_crates: usize = parts[1].parse().unwrap(); 
        let from_stack: usize = parts[3].parse().unwrap(); 
        let to_stack: usize = parts[5].parse().unwrap();
        for _ in 0..num_crates {
            let c: char = crates[from_stack - 1].pop().unwrap();
            crates[to_stack - 1].push(c);
        }
    }
}

fn process_moves2(crates: &mut Vec<Vec<char>>, moves: &str) {
    for l in moves.lines() {
        println!("{:?}", crates);
        let parts: Vec<&str> = l.split(" ").collect();
        let num_crates: usize = parts[1].parse().unwrap(); 
        let from_stack: usize = parts[3].parse().unwrap(); 
        let to_stack: usize = parts[5].parse().unwrap();
        for i in 0..num_crates {
            let index_remove = crates[from_stack - 1].len() + i - num_crates;
            println!("{:?}", index_remove);
            let c: char = crates[from_stack - 1].remove(index_remove);
            crates[to_stack - 1].push(c);
        }
    }
}

fn get_top_of_crates(crates_vec: Vec<Vec<char>>) -> String {
    let mut top = String::new(); 
    for c in &crates_vec { 
        if c.len() > 0 { 
            top.push(c[c.len() - 1]); 
        } 
    } 
    top
}

fn main() {
    let input = load_input();
    let (crates, moves) = input.split_once("\n\n").unwrap();
    let mut crates_vec = crates_ascii2vec(&crates);
    let mut crates_vec2 = crates_vec.clone();
    process_moves(&mut crates_vec, moves);
    process_moves2(&mut crates_vec2, moves);
    println!("Part1: {}", get_top_of_crates(crates_vec));
    println!("Part2: {}", get_top_of_crates(crates_vec2));
}
