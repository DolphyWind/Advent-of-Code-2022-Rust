use std::io;

// https://adventofcode.com/2022/day/2

// A for rock, B for paper, C for scissors.
fn get_score(opponents_play: char, your_play: char) -> i32
{
    if opponents_play == your_play{
        return 3;
    }

    if (opponents_play == 'A' && your_play == 'B') || (opponents_play == 'B' && your_play == 'C') || (opponents_play == 'C' && your_play == 'A'){
        return 6;
    }

    return 0;
}

// A for rock, B for paper, C for scissors.
fn get_score_from_play(play: char) -> i32{
    return match play {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    }
}

// A for rock, B for paper, C for scissors.
fn get_win_character(play: char) -> char{
    return match play {
        'A' => 'B',
        'B' => 'C',
        'C' => 'A',
        _ => '\0',
    }
}

// A for rock, B for paper, C for scissors.
fn get_lose_character(play: char) -> char{
    return match play {
        'A' => 'C',
        'B' => 'A',
        'C' => 'B',
        _ => '\0',
    }
}

fn main()
{
    // Part One
    let mut score = 0;
    loop
    {
        let mut strategy_str = String::new();
        io::stdin().read_line(&mut strategy_str).expect("failed to readline");
        strategy_str = strategy_str[0..strategy_str.len() - 1].to_string();
        if strategy_str.is_empty(){
            break;
        }

        let opponents_play = strategy_str.chars().nth(0).unwrap();
        let mut my_play = strategy_str.chars().nth(2).unwrap();

        my_play = match my_play {
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',
            _ => '\0'
        };

        score += get_score_from_play(my_play) + get_score(opponents_play, my_play);
    }
    println!("Part One: {}", score);

    score = 0;
    loop
    {
        let mut strategy_str = String::new();
        io::stdin().read_line(&mut strategy_str).expect("failed to readline");
        strategy_str = strategy_str[0..strategy_str.len() - 1].to_string();
        if strategy_str.is_empty(){
            break;
        }

        let opponents_play = strategy_str.chars().nth(0).unwrap();
        let mut game_status = strategy_str.chars().nth(2).unwrap();
        let mut my_play :char = '\0';

        if game_status == 'X'{
            my_play = get_lose_character(opponents_play)
        }
        else if game_status == 'Y' {
            my_play = opponents_play;
        }
        else {
            my_play = get_win_character(opponents_play)
        }

        score += get_score(opponents_play, my_play) + get_score_from_play(my_play);
    }
    println!("Part Two: {}", score);


}
