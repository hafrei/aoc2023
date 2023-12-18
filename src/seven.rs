#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    pub fn new<'a>(cards: &'a str, bid: u32) -> Self {
        let mut set = Vec::new();
        for c in cards.chars() {
            set.push(Card::new(c));
        }
        Self {
            cards: set.try_into().unwrap(),
            bid,
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
    for hand in hands {
        println!("{hand:?}");
    }
}
