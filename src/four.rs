struct Pile {
    starting_cards: Vec<Card>,
    next_set: Vec<u32>,
}

impl Pile {
    fn new(input: String) -> Self {
        Self {
            starting_cards: get_cards(input),
            next_set: vec![],
        }
    }
}

struct Card {
    id: u32,
    winning: Vec<u32>,
    play: Vec<u32>,
    point_value: Option<u32>,
}

impl Card {
    fn new(id: u32, winning: Vec<u32>, play: Vec<u32>) -> Self {
        Self {
            id,
            winning,
            play,
            point_value: None,
        }
    }

    fn get_point_count(&self) -> u32 {
        println!("On Card {}", self.id);
        let mut tally: u32 = 0;
        let mut value = 0;
        for w in self.winning.iter() {
            println!("Looking for {w}");
            if self.play.contains(w) {
                println!("  Found it!");
                value += match tally {
                    0 | 1 => 1,
                    2 => 2,
                    3 => 4,
                    4 => 8,
                    5 => 16,
                    6 => 32,
                    7 => 64,
                    8 => 128,
                    9 => 256,
                    10 => 512,
                    _ => unreachable!(),
                };
                tally += 1;
                println!("   {tally} wins, {value} points");
            }
        }
        println!("     this card is worth {value} total points\n");
        value
    }

    fn part_two_count_points(&mut self) {
        let mut count = Vec::new();
        for w in self.winning.iter() {
            if self.play.contains(w) {
                count.push(*w);
            }
        }
        self.point_value = Some(self.get_point_count());
    }

    fn part_one_count_points(&self) -> u32 {
        self.get_point_count()
    }
}

pub fn run(input: String) {
    let mut pile: Pile = Pile::new(input);
    let part_one: u32 = pile
        .starting_cards
        .iter()
        .map(|x| x.part_one_count_points())
        .sum();
    println!("The cards are worth {part_one} points");
    pile.starting_cards.iter_mut().for_each(|x| {
        x.part_two_count_points();
    });
    println!("The cards are worth {part_one} points");
    // let part_two: usize = pile.starting_cards.iter().map(|x| x.id_links.len()).sum();
    // println!("The cards are worth {part_two} points");
}

fn get_cards(input: String) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for card in input.lines() {
        let (card_enum, numbers) = card.split_once(':').unwrap();
        let id = card_enum
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let (winning_numbers, play_numbers) = numbers.split_once('|').unwrap();
        let winning = winning_numbers
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let play = play_numbers
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        cards.push(Card::new(id, winning, play));
    }
    cards
}
