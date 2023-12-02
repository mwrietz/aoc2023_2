// 2023 day 2

struct Count {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    number: u32,
    sets: Vec<Count>,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let games = parse_games();

    // determine valid games
    let reds: u32 = 12;
    let greens: u32 = 13;
    let blues: u32 = 14;
    let mut sum_of_valid_game_numbers: u32 = 0;
    for game in games {
        let mut valid = true;
        for set in game.sets {
            if set.red > reds || set.green > greens || set.blue > blues {
                valid = false;
            }
        }
        if valid {
            sum_of_valid_game_numbers += game.number;
        }
    }
    println!("sum_of_valid_game_numbers: {}", sum_of_valid_game_numbers);
}

fn part2() {
    let games = parse_games();

    // determine fewest numbers 
    let mut powers: Vec<u32> = vec![];
    for game in games {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for set in game.sets {
            if set.red > red { red = set.red; }
            if set.green > green { green = set.green; }
            if set.blue > blue { blue = set.blue; }
        }
        powers.push(red*green*blue);
    }

    let sum: u32 = powers.iter().sum();
    println!("sum of powers: {}", sum);
}

fn parse_games() -> Vec<Game> {
    let mut games: Vec<Game> = vec![];
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    for line in lines {

        // get game number
        let colon_position = line.find(':').unwrap();
        let game_name = &line[0..colon_position];
        let game_number: u32 = game_name.split(' ').last().unwrap().trim().parse().unwrap();
        let mut game_sets: Vec<Count> = vec![];

        // get set counts
        let sets_slice = &line[(colon_position + 1)..];
        let sets_slice_vec: Vec<&str> = sets_slice.split("; ").collect();
        
        for set in sets_slice_vec {
            game_sets.push(get_count(set));
        }

        // add game number and set counts to game struct
        let g = Game {
            number: game_number,
            sets: game_sets,
        };

        // push game number and sets to games vector
        games.push(g);
    }

    games
}

fn get_count(set: &str) -> Count {
    let mut count = Count {
        red: 0,
        green: 0,
        blue: 0,
    };
    let qtys: Vec<&str> = set.split(", ").collect();
    for q in qtys {
        let val: Vec<&str> = q.trim_start().split(' ').collect();
        match val[1].trim() {
            "red" => { count.red += val[0].parse::<u32>().unwrap() },
            "green" => { count.green += val[0].parse::<u32>().unwrap() }, 
            "blue" => { count.blue += val[0].parse::<u32>().unwrap() },
            _ => break,
        }
    }

    count
}
