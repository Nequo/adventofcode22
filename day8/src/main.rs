fn create_grid(input: &str) -> Vec<Vec<usize>> {
    let mut grid = Vec::new();
    for l in input.lines() {
        let row: Vec<usize> = l.chars().map(|c| (c.to_digit(10).unwrap() as usize)).collect();
        grid.push(row);
    }
    grid
}

fn get_from_grid(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    grid[x][y]
}

fn tree_is_visible(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    let height = grid.len();
    let width = grid[x].len();
    if x == 0 || x == height-1 || y == 0 || y == width-1 {
        return true
    }
    let current_tree = get_from_grid(grid, x, y);
    //check to the top
    let mut visible_from_top: bool = true;
    for i in 0..x {
        let tree = get_from_grid(grid, i, y);
        if tree >= current_tree {  visible_from_top = false ; break}
    }
    //println!("{} visible from top?, {}", current_tree, visible_from_top);
    //check to the bottom
    let mut visible_from_bottom: bool = true;
    for i in x+1..height {
        let tree = get_from_grid(grid, i, y);
        if tree >= current_tree { visible_from_bottom = false ; break}
    }
    //println!("{} visible from bottom?, {}", current_tree, visible_from_bottom);
    //check to the left
    let mut visible_from_left: bool = true;
    for i in 0..y {
        let tree = get_from_grid(grid, x, i);
        if tree >= current_tree { visible_from_left = false ; break}
    }
    //println!("{} visible from left?, {}", current_tree, visible_from_left);
    //check to the right
    let mut visible_from_right: bool = true;
    for i in y+1..width {
        let tree = get_from_grid(grid, x, i);
        if tree >= current_tree { visible_from_right = false ; break}
    }
    //println!("{} visible from right?, {}", current_tree, visible_from_right);
    return visible_from_top || visible_from_bottom || visible_from_left || visible_from_right
}

fn tree_score(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let height = grid.len();
    let width = grid[x].len();
    let current_tree = get_from_grid(grid, x, y);
    //check to the top
    let mut score_top: usize = 0;
    for i in (0..x).rev() {
        let tree = get_from_grid(grid, i, y);
        if tree >= current_tree { score_top += 1; break;}
        score_top += 1;
    }
    //println!("{} visible from top?, {}", current_tree, visible_from_top);
    //check to the bottom
    let mut score_bottom: usize = 0;
    for i in x+1..height {
        let tree = get_from_grid(grid, i, y);
        if tree >= current_tree { score_bottom +=1; break}
        score_bottom += 1;
    }
    //println!("{} visible from bottom?, {}", current_tree, visible_from_bottom);
    //check to the left
    let mut score_left: usize = 0;
    for i in (0..y).rev() {
        let tree = get_from_grid(grid, x, i);
        if tree >= current_tree { score_left +=1; break}
        score_left += 1;
    }
    //println!("{} visible from left?, {}", current_tree, visible_from_left);
    //check to the right
    let mut score_right: usize = 0;
    for i in y+1..width {
        let tree = get_from_grid(grid, x, i);
        if tree >= current_tree { score_right +=1; break}
        score_right += 1;
    }
    //println!("{} visible from right?, {}", current_tree, visible_from_right);
    return score_top * score_bottom * score_left * score_right
}

//fn visible_trees(grid: &Vec<Vec<usize>>) -> usize {
//    let total = 0;
//    for i in 1..grid.len()-1 {
//        for j in  1..grid[i].len()-1 {
//            println!("{}", grid[i][j]);
//        }
//    }
//    total
//}


fn main() {
    let input = include_str!("../input");
    let grid = create_grid(&input);
    let mut sum_visible: usize = 0;
    let mut best_score: usize = 0;
    for i in 0..grid.len() {
        for j in  0..grid[i].len() {
            if tree_is_visible(&grid, i, j) {
                //println!("{} from row {} column {} is visible", grid[i][j], i, j);
                sum_visible += 1;
            }
            let score = tree_score(&grid, i, j);
            if score > best_score { best_score = score }
        }
    }
    println!("Part1: {:?}", sum_visible);
    println!("Part2: {:?}", best_score);
    //let total = visible_trees(&grid);
    //println!("Part1: {:?}", total);
}
