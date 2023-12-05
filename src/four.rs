struct Card {
    id: u32,
    winning: Vec<u32>,
    play: Vec<u32>,
}

impl Card {
    fn count_points(&self) -> u32 {
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
}

pub fn run(input: String) {
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
        cards.push(Card {id, winning, play});
    }
    let part_one: u32 = cards.iter().map(|x| x.count_points()).sum();
    println!("The cards are worth {part_one} points");
}
