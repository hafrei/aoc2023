#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PartTwoCard {
    A,
    K,
    Q,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    J,
}

impl CardMatch for PartTwoCard {
    fn determine_card(c: char) -> Self {
        match c {
            'A' => PartTwoCard::A,
            'K' => PartTwoCard::K,
            'Q' => PartTwoCard::Q,
            'T' => PartTwoCard::T,
            '9' => PartTwoCard::C9,
            '8' => PartTwoCard::C8,
            '7' => PartTwoCard::C7,
            '6' => PartTwoCard::C6,
            '5' => PartTwoCard::C5,
            '4' => PartTwoCard::C4,
            '3' => PartTwoCard::C3,
            '2' => PartTwoCard::C2,
            'J' => PartTwoCard::J,
            _ => unreachable!(),
        }
    }

    fn top_two(cards: &Vec<Self>) -> ((Self, usize), (Self, usize))
    where
        Self: Sized,
    {
        let wildcards = cards
            .clone()
            .into_iter()
            .filter(|x| *x != PartTwoCard::J)
            .collect::<Vec<Self>>();
        let reference = cards.clone().to_owned();
        let mut final_high = (PartTwoCard::J, 0);
        let mut final_second_high = (PartTwoCard::J, 0);
        if wildcards.is_empty() {
            for card in cards {
                let tally = cards.iter().filter(|x| *x == card).count();
                if tally > final_high.1 || (tally >= final_high.1 && *card < final_high.0) {
                    final_second_high = final_high;
                    final_high = (*card, tally);
                } else if tally >= final_second_high.1 && *card != final_high.0 {
                    final_second_high = (*card, tally);
                }
            }
        }

        for not_joker in wildcards {
            let mut high = (PartTwoCard::J, 0);
            let mut second_high = (PartTwoCard::J, 0);
            let try_permutation: Vec<Self> = reference
                .clone()
                .iter_mut()
                .map(|x| if *x == PartTwoCard::J { not_joker } else { *x })
                .collect();

            for card in &try_permutation {
                let tally = try_permutation.iter().filter(|x| x == &card).count();
                if tally > high.1 || (tally >= high.1 && *card < high.0) {
                    second_high = high;
                    high = (*card, tally);
                } else if tally >= second_high.1 && *card != high.0 {
                    second_high = (*card, tally);
                }
            }
            if high.1 > final_high.1 {
                final_high = high;
                final_second_high = second_high;
            }
        }
        if final_second_high.1 >= final_high.1
            && final_high.0 != final_second_high.0
            && final_second_high.0 <= final_high.0
        {
            (final_second_high, final_high)
        } else {
            (final_high, final_second_high)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PartOneCard {
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

impl CardMatch for PartOneCard {
    fn determine_card(c: char) -> Self {
        match c {
            'A' => PartOneCard::A,
            'K' => PartOneCard::K,
            'Q' => PartOneCard::Q,
            'J' => PartOneCard::J,
            'T' => PartOneCard::T,
            '9' => PartOneCard::C9,
            '8' => PartOneCard::C8,
            '7' => PartOneCard::C7,
            '6' => PartOneCard::C6,
            '5' => PartOneCard::C5,
            '4' => PartOneCard::C4,
            '3' => PartOneCard::C3,
            '2' => PartOneCard::C2,
            _ => unreachable!(),
        }
    }

    fn top_two(cards: &Vec<Self>) -> ((Self, usize), (Self, usize))
    where
        Self: Sized,
    {
        let mut high = (PartOneCard::C2, 0);
        let mut second_high = (PartOneCard::C2, 0);
        for card in cards {
            let tally = cards.iter().filter(|x| *x == card).count();
            if tally > high.1 || (tally >= high.1 && *card < high.0) {
                second_high = high;
                high = (*card, tally);
            } else if tally >= second_high.1 && *card != high.0 {
                second_high = (*card, tally);
            }
        }
        (high, second_high)
    }
}

trait CardMatch {
    fn determine_card(c: char) -> Self;
    fn top_two(cards: &Vec<Self>) -> ((Self, usize), (Self, usize))
    where
        Self: Sized;
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
struct Hand<C>
where
    C: CardMatch,
{
    hand_type: Type,
    cards: Vec<C>,
    bid: u32,
}

impl<C> Hand<C>
where
    C: CardMatch + PartialEq + Copy + std::fmt::Debug,
{
    pub fn new<'a>(cards: &'a str, bid: u32, card_type: impl Fn(char) -> C) -> Self {
        let mut set = Vec::new();
        for c in cards.chars() {
            set.push(card_type(c));
        }
        let hand_type = Hand::determine_hand_type(&set);
        Self {
            hand_type,
            cards: set,
            bid,
        }
    }

    fn determine_hand_type(cards: &Vec<C>) -> Type {
        let (high, second_high) = C::top_two(cards);
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
            _ => {
                println!("broke with {:?} and {:?}", high, second_high);
                unreachable!()
            }
        }
    }
}

pub fn run(input: String) {
    let mut hands: Vec<Hand<PartOneCard>> = Vec::new();
    let mut two_hands: Vec<Hand<PartTwoCard>> = Vec::new();
    for l in input.lines() {
        let Some((hand, bid)) = l.split_once(' ') else {
            unreachable!()
        };
        let b = bid.parse::<u32>().unwrap();
        let one_hand = Hand::new(hand, b, PartOneCard::determine_card);
        hands.push(one_hand);
        let two_hand = Hand::new(hand, b, PartTwoCard::determine_card);
        two_hands.push(two_hand);
    }

    hands.sort();
    two_hands.sort();

    let part_one_total_hands: u32 = hands.len() as u32;
    let part_one: u32 = hands
        .iter()
        .enumerate()
        .map(|(e, c)| c.bid * (part_one_total_hands - e as u32))
        .sum();
    println!("Part one total winnings: {part_one}");
    let part_two_total_hands: u32 = two_hands.len() as u32;
    let part_two: u32 = two_hands
        .iter()
        .enumerate()
        .map(|(e, c)| c.bid * (part_two_total_hands - e as u32))
        .sum();
    println!("Total winnings: {part_two}");
}
