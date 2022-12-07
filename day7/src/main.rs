use std::collections::HashMap;
use std::path::PathBuf;

fn parse_input(input: &str) -> HashMap<String, usize> {
    let mut currentdir = PathBuf::new();
    let mut sizes: HashMap<String, usize> = HashMap::new();
    for l in input.lines() {
        if l.starts_with("$ cd ..") {
            currentdir = currentdir.parent().unwrap().to_path_buf();
            println!("currentdir: {:?}", currentdir);
            continue
        }
        else if l.starts_with("$ cd") {
            currentdir = currentdir.clone().join(&l.split(" ").last().unwrap().parse::<String>().unwrap());
            println!("currentdir: {:?}", &currentdir);
        }
        else if l.starts_with("$ ls") {
            println!("listing:");
        }
        else if l.starts_with("dir") {
            // add a node to the tree
            let dirname = l.split(" ").last().unwrap();
            println!("\tdirectory: {}", dirname);
        }
        else if l.starts_with(&['0','1','2','3','4','5','6','7','8','9']) {
            // add a size to the tree
            let size: usize = l.split(" ").take(1).next().unwrap().parse().unwrap();
            for d in currentdir.ancestors() {
                println!("{:?}", d);
                *sizes.entry(d.to_str().unwrap().to_string()).or_insert(0) += size;
            }
            println!("\tfile with size: {}", size);
        }
    }
    sizes
}

fn sum_of_smalls(sizes: &HashMap<String, usize>) -> usize {
    let mut totalsize = 0;
    for (k, v) in sizes {
        if k == "/" {
            println!("Root size: {}", v);
            println!("Free space: {}", 70000000 - v);
        }
        if v < &100000 {
            totalsize += v;
        }

    }
    totalsize
}

fn find_smallest_to_delete(sizes: &HashMap<String, usize>, free_space: usize) -> usize {
    let mut smallest = 70000000;
    for (_, v) in sizes {
        if v + free_space > 30000000  && v < &smallest {
            smallest = *v;
        }
    }
    smallest
}

fn main() {
    let input = include_str!("../input");
    println!("input {}", &input);
    let sizes = parse_input(&input);
    println!("{:?}", sizes);
    let total = sum_of_smalls(&sizes);
    println!("Part1: {:?}", total);
    let free_space = 70000000 - &sizes["/"];
    println!("Free space: {}", free_space);
    let to_delete = find_smallest_to_delete(&sizes, free_space);
    println!("Part2: {:?}", to_delete);
}
