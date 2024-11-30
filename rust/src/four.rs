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

    pub fn card_count(&mut self) -> u32 {
        let mut card_idx = 1;
        let mut cards: u32 = 0;
        println!("");
        loop {
            if card_idx > self.starting_cards.len() {
                break;
            }
            let (times, set) = if card_idx == 1 {
                (1, self.starting_cards[card_idx - 1].part_two_count_points())
            } else {
                let times = self
                    .next_set
                    .iter()
                    .filter(|x| **x == card_idx as u32)
                    .count()
                    + 1;
                (
                    times,
                    self.starting_cards[card_idx - 1].part_two_count_points(),
                )
            };

            cards += times as u32;

            if set.len() > 0 {
                for e in 1..=set.len() {
                    for _ in 1..=times {
                        let adder = card_idx + e;
                        self.next_set.push(adder as u32);
                    }
                }
            }
            card_idx += 1;
        }
        cards
    }
}

struct Card {
    id: u32,
    winning: Vec<u32>,
    play: Vec<u32>,
}

impl Card {
    fn new(id: u32, winning: Vec<u32>, play: Vec<u32>) -> Self {
        Self { id, winning, play }
    }

    fn get_point_count(&self) -> u32 {
        println!("On Card {}", self.id);
        let mut tally: u32 = 0;
        let mut value = 0;
        for w in self.winning.iter() {
            println!("Looking for {w}");
            if self.play.contains(w) {
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

    fn part_two_count_points(&self) -> Vec<u32> {
        let mut count = Vec::new();
        for w in self.winning.iter() {
            if self.play.contains(w) {
                count.push(*w);
            }
        }
        count
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
    let part_two = pile.card_count();
    println!("There are {part_two} cards total");
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
