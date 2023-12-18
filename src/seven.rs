#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
}

impl Card {
    pub fn new(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::C9,
            '8' => Card::C8,
            '7' => Card::C7,
            '6' => Card::C6,
            '5' => Card::C5,
            '4' => Card::C4,
            '3' => Card::C3,
            '2' => Card::C2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Hand {
    hand_type: Type,
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    pub fn new<'a>(cards: &'a str, bid: u32) -> Self {
        let mut set = Vec::new();
        for c in cards.chars() {
            set.push(Card::new(c));
        }
        let hand_type = Hand::determine_hand_type(&set);
        Self {
            hand_type,
            cards: set.try_into().unwrap(),
            bid,
        }
    }

    fn determine_hand_type(cards: &Vec<Card>) -> Type {
        let mut high = (Card::C2, 0);
        let mut second_high = (Card::C2, 0);
        // println!("Cards: {cards:?}");
        for card in cards {
            let tally = cards.iter().filter(|x| *x == card).count();
            if tally > high.1 {
                second_high = high;
                high = (*card, tally);
            } else if tally > second_high.1 && *card != high.0 {
                second_high = (*card, tally);
            }
        }
        // println!("high:{high:?}\nsecond: {second_high:?}\n");
        match high.1 {
            5 => Type::FiveKind,
            4 => Type::FourKind,
            3 => {
                if second_high.1 == 2 {
                    Type::FullHouse
                } else {
                    Type::ThreeKind
                }
            }
            2 => {
                if second_high.1 == 2 {
                    Type::TwoPair
                } else {
                    Type::Pair
                }
            }
            1 => Type::HighCard,
            _ => unreachable!(),
        }
    }
}

pub fn run(input: String) {
    let mut hands: Vec<Hand> = Vec::new();
    for l in input.lines() {
        let Some((hand, bid)) = l.split_once(' ') else {
            unreachable!()
        };
        let hand = Hand::new(hand, bid.parse::<u32>().unwrap());
        hands.push(hand);
    }

    for hand in &hands {
        println!("{hand:?}");
    }
    hands.sort();
    println!("");
    for hand in &hands {
        println!("{hand:?}");
    }
    let total_hands: u32 = hands.len() as u32;
    let part_one: u32 = hands
        .iter()
        .enumerate()
        .map(|(e, c)| c.bid * (total_hands - e as u32))
        .sum();
    println!("Total winnings: {part_one}");
}
