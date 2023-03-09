use std::cmp::max;
use std::io;

fn is_visible(trees: &Vec<Vec<char>>, x:usize, y: usize) -> bool{
    if x == 0 || y == 0 || y == trees.len() || x == trees[0].len(){
        return true;
    }
    let mut current_val = trees[y][x] as u32;
    let mut has_found_tree = false;

    for i in 0..x{
        let current_tree = trees[y][i] as u32;
        if current_tree >= current_val{
            has_found_tree = true;
            break;
        }
    }
    if !has_found_tree{
        return true;
    }

    has_found_tree = false;
    for i in 0..y{
        let current_tree = trees[i][x] as u32;
        if current_tree >= current_val{
            has_found_tree = true;
            break;
        }
    }
    if !has_found_tree{
        return true;
    }

    has_found_tree = false;
    for i in (x+1)..trees[0].len(){
        let current_tree = trees[y][i] as u32;
        if current_tree >= current_val{
            has_found_tree = true;
            break;
        }
    }
    if !has_found_tree{
        return true;
    }

    has_found_tree = false;
    for i in (y+1)..trees.len(){
        let current_tree = trees[i][x] as u32;
        if current_tree >= current_val{
            has_found_tree = true;
            break;
        }
    }
    if !has_found_tree{
        return true;
    }

    return false;
}

fn get_scenic_score(trees: &Vec<Vec<char>>, x:usize, y: usize) -> i32{
    if x == 0 || y == 0 || y == trees.len() || x == trees[0].len(){
        return 0;
    }

    let mut scores: [i32; 4] = [0, 0, 0, 0];
    let current_val = trees[y][x] as i32;

    let mut toplen: i32 = current_val;
    for i in (0..x).rev(){
        let mut current_tree = trees[y][i] as i32;
        scores[0] += 1;
        if current_tree >= current_val {
            break;
        }
    }

    toplen = current_val;
    for i in (0..y).rev(){
        let mut current_tree = trees[i][x] as i32;
        scores[1] += 1;
        if current_tree >= current_val {
            break;
        }
    }

    toplen = current_val;
    for i in (x+1)..trees[0].len(){
        let mut current_tree = trees[y][i] as i32;
        scores[2] += 1;
        if current_tree >= current_val {
            break;
        }
    }

    toplen = current_val;
    for i in (y+1)..trees.len(){
        let mut current_tree = trees[i][x] as i32;
        scores[3] += 1;
        if current_tree >= current_val {
            break;
        }
    }

    return scores[0] * scores[1] * scores[2] * scores[3];
}

fn main() {
    let mut inputs: Vec<Vec<char>> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        input = input[0..input.len() - 1].to_string();
        if input.is_empty() {
            break;
        }
        inputs.push(input.chars().collect());
    }

    let mut count = 0;
    for y in 0..inputs.len(){
        for x in 0..inputs[0].len(){
            if is_visible(&inputs, x, y){
                count += 1;
            }
        }
    }

    println!("Part One: {}", count);

    let mut best_scenery = 0;
    for y in 0..inputs.len(){
        for x in 0..inputs[0].len(){
            let points = get_scenic_score(&inputs, x, y);
            if points > best_scenery{
                best_scenery = points;
            }
        }
    }

    println!("Part Two: {}", best_scenery);

}
