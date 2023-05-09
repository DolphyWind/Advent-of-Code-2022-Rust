use std::io;
use std::collections::HashSet;

#[derive(Copy, Clone)]
struct Point{
    x: i32,
    y: i32,
}

fn manhattan_distance(p1: Point, p2: Point) -> i32
{
    return p1.x.abs_diff(p2.x) as i32 + p1.y.abs_diff(p2.y) as i32;
}

fn can_have_beacon(pairs: &Vec<(Point, i32)>, current_point: Point) -> bool{
    for pair in pairs{
        let dist = manhattan_distance(pair.0, current_point);
        if dist <= pair.1{
            return false;
        }
    }
    return true;
}

fn check_borders(pairs: &Vec<(Point, i32)>, sensor_point: Point, dist: i32) -> Option<Point>{
    for i in 0..(dist + 1){
        let p0 = Point{x: sensor_point.x + i, y: sensor_point.y + (dist + 1 - i)};
        let p1 = Point{x: sensor_point.x - i, y: sensor_point.y + (dist + 1 - i)};
        let p2 = Point{x: sensor_point.x + i, y: sensor_point.y - (dist + 1 - i)};
        let p3 = Point{x: sensor_point.x - i, y: sensor_point.y - (dist + 1 - i)};
        if can_have_beacon(pairs, p0){
            return Option::from(p0);
        }
        if can_have_beacon(pairs, p1){
            return Option::from(p1);
        }
        if can_have_beacon(pairs, p2){
            return Option::from(p2);
        }
        if can_have_beacon(pairs, p3){
            return Option::from(p3);
        }
    }
    return None;
}

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        input = input[0..input.len() - 1].to_string();
        if input.is_empty(){
            break;
        }
        inputs.push(input);
    }

    let target_y = 2000000;
    let mut beacons_on_target_row: HashSet<i32> = HashSet::new();

    let mut pairs: Vec<(Point, i32)> = Vec::new();
    for input in inputs{
        let mut split: Vec<&str> = input.split(" ").collect();

        let sensor_x_str = &split[2];
        let sensor_x = sensor_x_str[2..(sensor_x_str.len() - 1)].parse::<i32>().unwrap();

        let sensor_y_str = &split[3];
        let sensor_y = sensor_y_str[2..(sensor_y_str.len() - 1)].parse::<i32>().unwrap();

        let beacon_x_str = &split[8];
        let beacon_x = beacon_x_str[2..(beacon_x_str.len() - 1)].parse::<i32>().unwrap();

        let beacon_y_str = &split[9];
        let beacon_y = beacon_y_str[2..].parse::<i32>().unwrap();

        if beacon_y == target_y{
            beacons_on_target_row.insert(beacon_x);
        }

        let dist = manhattan_distance(Point{x: sensor_x, y: sensor_y}, Point{x: beacon_x, y: beacon_y});
        let mut pair: (Point, i32) = (Point{x: sensor_x, y: sensor_y}, dist);
        pairs.push(pair);
    }

    let mut max_x = -2147483648;
    let mut min_x = 2147483647;
    for pair in &pairs{
        if pair.0.x + pair.1 > max_x{
            max_x = pair.0.x + pair.1;
        }
        if pair.0.x - pair.1 < min_x{
            min_x = pair.0.x - pair.1;
        }
    }

    let mut not_ok = 0;
    for current_x in min_x..(max_x + 1){
        if !can_have_beacon(&pairs, Point{x: current_x, y: target_y}){
            not_ok += 1;
        }
    }
    not_ok -= beacons_on_target_row.len();

    println!("Part One: {}", not_ok);

    let mut limit = 4000000i64;
    let four_mil = 4000000i64;

    for pair in &pairs{
        let point = pair.0;
        let dist = pair.1;
        let p = check_borders(&pairs, point, dist);
        if p.is_some(){
            let part2_answer = p.unwrap();
            if part2_answer.x >= 0 && part2_answer.x <= limit as i32 && part2_answer.y >= 0 && part2_answer.y <= limit as i32{
                println!("Part Two: {}", (part2_answer.x as i64) * four_mil + (part2_answer.y as i64));
                break;
            }
        }
    }
}
