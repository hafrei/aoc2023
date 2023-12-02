#[derive(Copy, Clone, Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            id: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn is_possible(&self) -> bool {
        self.red <= POSSIBLE_GAME.red
            && self.green <= POSSIBLE_GAME.green
            && self.blue <= POSSIBLE_GAME.blue
    }
}

const POSSIBLE_GAME: Game = Game {
    id: 0, //the template game has no id
    red: 12,
    green: 13,
    blue: 14,
};

pub fn run(input: String) {
    let games = build_games(input);
    let sum: u32 = games
        .iter()
        .cloned()
        .filter(|x| x.is_possible())
        .map(|x| x.id)
        .sum();
    println!("The sum of part one is {sum}");
}

fn build_games(input: String) -> Vec<Game> {
    let mut games = Vec::new();

    input.lines().for_each(|l| {
        let mut game = Game::new();
        let (id, viewed) = l.split_once(':').unwrap();
        game.id = parse_number(id.to_string());
        for shown in viewed.split(';') {
            for single in shown.split(',') {
                if single.contains("blue") {
                    let count: u32 = parse_number(single.to_string());
                    if count > game.blue {
                        game.blue = count;
                    }
                }
                if single.contains("red") {
                    let count: u32 = parse_number(single.to_string());
                    if count > game.red {
                        game.red = count;
                    }
                }
                if single.contains("green") {
                    let count: u32 = parse_number(single.to_string());
                    if count > game.green {
                        game.green = count;
                    }
                }
            }
        }
        games.push(game);
    });

    games
}

fn parse_number(line: String) -> u32 {
    line.chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}
