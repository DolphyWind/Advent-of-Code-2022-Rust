use std::ops;
use std::io;

/*
This code below theoretically should work but the input data is too big and
code tries every path possible to find the shortest one so it'd take a lot
of time to compute correct result.
*/

#[derive(Debug, Clone, Copy)]
pub struct Position
{
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        return Position{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::Neg for Position{
    type Output = Position;

    fn neg(self) -> Self::Output {
        return Position{
            x: -self.x,
            y: -self.y,
        };
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

pub struct Direction;
impl Direction {
    #[inline]
    fn up() -> Position{
        return Position{
            x: 0,
            y: -1,
        };
    }
    fn down() -> Position{
        return Position{
            x: 0,
            y: 1,
        };
    }
    fn left() -> Position{
        return Position{
            x: -1,
            y: 0,
        };
    }
    fn right() -> Position{
        return Position{
            x: 1,
            y: 0,
        };
    }
}

fn get_elevation(mut ch: char) -> i32{
    if ch == 'S'{
        ch = 'a';
    }
    if ch == 'E'{
        ch = 'z';
    }
    return (ch as i32) - ('a' as i32);
}

#[derive(Debug)]
pub struct MemoryCell
{
    pub ptg: Vec<Position>, // ptg stands for possible to go
    pub current_elevation: i32,
    pub current_pos: Position,
}

#[derive(Debug)]
pub struct Traveller
{
    pub total_steps: i32,
    pub target_pos: Position,
    pub memories: Vec<MemoryCell>,
    pub been_to: Vec<Position>,
    pub solutions: Vec<i32>,
}

impl Traveller{
    fn step(&mut self, board: &Vec<String>){
        if self.memories.is_empty(){
            return;
        }

        let memory_len = self.memories.len();
        let current_pos = self.memories[memory_len - 1].current_pos;
        self.been_to.push(current_pos);

        if self.memories[memory_len - 1].ptg.is_empty() || current_pos == self.target_pos{
            if self.target_pos == current_pos{
                self.solutions.push(self.total_steps);
            }
            self.been_to.retain(|&x| x != current_pos);
            self.memories.pop();
            self.total_steps -= 1;
            self.step(board);
            return;
        }
        let dir: Position = self.memories[memory_len - 1].ptg[0];
        self.memories[memory_len - 1].ptg.remove(0);
        let new_pos: Position = current_pos + dir;
        self.total_steps += 1;
        self.generate_new_memory(new_pos, board);
    }

    fn generate_new_memory(&mut self, pos: Position, board: &Vec<String>){
        let directions: [Position; 4] = [Direction::right(), Direction::up(), Direction::left(), Direction::down()];
        let mut new_ptg: Vec<Position> = Vec::new();
        let current_elev: i32 = get_elevation(board[pos.y as usize].chars().nth(pos.x as usize).unwrap());

        for dir in directions{
            let next_pos: Position = pos + dir;
            if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= board[0].len() as i32 || next_pos.y >= board.len() as i32 {
                continue;
            }
            if self.been_to.contains(&next_pos){
                continue;
            }
            
            let next_elev: i32 = get_elevation(board[next_pos.y as usize].chars().nth(next_pos.x as usize).unwrap());

            if next_elev - current_elev > 1{
                continue;
            }
            new_ptg.push(dir);
        }

        self.memories.push(MemoryCell{
            ptg: new_ptg,
            current_elevation: current_elev,
            current_pos: pos,
        });
    }
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

    let mut start_pos: Position = Position{
        x: 0,
        y: 0,
    };

    let mut target_pos: Position = Position{
        x: 0,
        y: 0,
    };

    for y in 0..inputs.len(){
        for x in 0..inputs[0].len(){
            let ch: char = inputs[y].chars().nth(x).unwrap();
            if ch == 'S'{
                start_pos = Position{
                    x: x as i32,
                    y: y as i32,
                };
            }
            else if ch == 'E' {
                target_pos = Position{
                    x: x as i32,
                    y: y as i32,
                }
            }
        }
    }

    let mut t = Traveller{
        total_steps: 0,
        target_pos,
        memories: Vec::new(),
        been_to: Vec::new(),
        solutions: Vec::new(),
    };
    t.generate_new_memory(start_pos, &mut inputs);
    while !t.memories.is_empty(){
        t.step(&mut inputs);
    }

    t.solutions.sort();
    println!("Part One: {}", t.solutions[0]);
}
