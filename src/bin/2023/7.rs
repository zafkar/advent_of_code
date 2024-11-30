use advent_of_code::load_data;
use std::{cmp::Ordering, io::BufRead, iter::zip, str::FromStr};

const ADVENT_NUM: &str = "2023/7";

fn main() {
    let file = load_data(ADVENT_NUM, "input.txt").unwrap();

    let mut all_hands: Vec<Hand> = vec![];
    for line in file.lines() {
        let hand: Hand = line.unwrap().parse().unwrap();
        let h_type = hand.get_hand_type();
        println!("{hand:?}, {h_type:?}");
        all_hands.push(hand);
    }

    let mut total = 0;
    all_hands.sort();
    println!("\nSorted");
    for (i, h) in all_hands.iter().enumerate() {
        if h.num_joker() > 0 {
            println!(
                "{h:?} => {:?}, Joker {}, Chains => {:?}",
                h.get_hand_type(),
                h.num_joker(),
                h.get_chains()
            );
        }
        total += (i as u32 + 1) * h.bet;
    }
    println!("Total => {total}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u32);

#[derive(Debug)]
struct ParseError;

impl FromStr for Card {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s.chars().nth(0) {
            Some('2') => 2,
            Some('3') => 3,
            Some('4') => 4,
            Some('5') => 5,
            Some('6') => 6,
            Some('7') => 7,
            Some('8') => 8,
            Some('9') => 9,
            Some('T') => 10,
            Some('J') => 1,
            Some('Q') => 12,
            Some('K') => 13,
            Some('A') => 14,
            _ => return Err(ParseError),
        };

        Ok(Card(val))
    }
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    bet: u32,
}

impl FromStr for Hand {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_txt, bet_txt) = match s.split_once(' ') {
            Some((a, b)) => (a, b),
            _ => return Err(ParseError),
        };

        let bet = match bet_txt.trim().parse() {
            Ok(a) => a,
            _ => return Err(ParseError),
        };

        let mut cards: [Card; 5] = [Card(0); 5];
        for (i, c) in cards_txt
            .trim()
            .split("")
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<Card>())
            .enumerate()
        {
            cards[i] = match c {
                Ok(a) => a,
                _ => return Err(ParseError),
            };
        }
        let h = Hand { bet, cards };
        Ok(h)
    }
}

#[derive(Debug)]
#[repr(u8)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s_type = self.get_hand_type();
        let o_type = other.get_hand_type();
        match (s_type as u8, o_type as u8) {
            (s, o) if s > o => Ordering::Greater,
            (s, o) if s < o => Ordering::Less,
            (s, o) if s == o => {
                for (s_c, o_c) in zip(self.cards.iter(), other.cards.iter()) {
                    match s_c.cmp(o_c) {
                        a if a != Ordering::Equal => return a,
                        _ => (),
                    }
                }
                Ordering::Equal
            }
            _ => unimplemented!("Error in Ord cmp"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl Hand {
    fn sort_hand(&mut self) {
        self.cards.sort();
    }

    fn get_hand_type(self) -> HandType {
        //Initial
        let chains = self.get_chains();

        //Search for joker chain
        let mut joker_chain_len = 0;
        for ch in chains.iter() {
            if ch.0 == "J".parse::<Card>().unwrap() {
                joker_chain_len = ch.1;
                if joker_chain_len == 5 {
                    return HandType::FiveOfAKind;
                }
            }
        }

        let mut num_pair = 0;
        let mut num_three = 0;
        let mut value_pair: Vec<Card> = vec![];
        let mut _value_three = Card(0);
        for ch in chains.iter().filter(|(c, _len)| c.0 != 1) {
            match (ch.1, joker_chain_len) {
                (5, 0) => return HandType::FiveOfAKind,
                (4, 1) => return HandType::FiveOfAKind,
                (4, 0) => return HandType::FourOfAKind,
                (3, 2) => return HandType::FiveOfAKind,
                (3, 1) => return HandType::FourOfAKind,
                (3, 0) => {
                    num_three += 1;
                    _value_three = ch.0;
                }
                (2, 3) => return HandType::FiveOfAKind,
                (2, 2) => return HandType::FourOfAKind,
                (2, 1) => {
                    num_three += 1;
                    _value_three = ch.0;
                }
                (2, 0) => {
                    num_pair += 1;
                    value_pair.push(ch.0);
                }
                (1, 4) => return HandType::FiveOfAKind,
                (1, 3) => return HandType::FourOfAKind,
                (1, 2) => {
                    num_three += 1;
                    _value_three = ch.0;
                }
                (1, 1) => {
                    num_pair += 1;
                    value_pair.push(ch.0);
                }
                (1, 0) => (),
                a => {
                    println!("{a:?}")
                }
            };
        }
        match (num_pair, num_three, joker_chain_len) {
            (a, 0, 1) if a > 1 => HandType::OnePair,
            (1, 0, _) => match value_pair.first() {
                Some(_) => HandType::OnePair,
                _ => unimplemented!("Error while one pair"),
            },
            (0, 1, _) => HandType::ThreeOfAKind,
            (2, 1, _) => HandType::ThreeOfAKind,
            (1, 1, _) => match value_pair.first() {
                Some(_) => HandType::FullHouse,
                _ => unimplemented!("Error while one pair"),
            },
            (0, 3, _) => HandType::ThreeOfAKind,
            (_, a, _) if a > 1 => HandType::FullHouse,
            (a, 0, _) if a > 1 => match (value_pair.first(), value_pair.get(1)) {
                (Some(_), Some(_)) => HandType::TwoPair,
                _ => unimplemented!("Error while outputting 2 pairs"),
            },
            _ => {
                let mut tmp = self;
                tmp.sort_hand();
                if tmp.cards.last().is_some() {
                    HandType::HighCard
                } else {
                    unimplemented!("Can't work with empty hand");
                }
            }
        }
    }

    fn get_chains(self) -> Vec<(Card, u8)> {
        let mut tmp = self;
        tmp.sort_hand();

        let mut chains: Vec<(Card, u8)> = Vec::new();
        let mut curr_chain_len = 0;
        let mut curr_chain_type = tmp.cards[0];
        for (prev, curr) in zip(tmp.cards[..4].iter(), tmp.cards[1..].iter()) {
            if *prev == *curr {
                curr_chain_len += 1;
            } else {
                chains.push((curr_chain_type, curr_chain_len + 1));
                curr_chain_len = 0;
                curr_chain_type = *curr;
            }
        }

        //Handle last chain

        chains.push((curr_chain_type, curr_chain_len + 1));
        chains
    }

    fn num_joker(self) -> usize {
        self.cards.iter().filter(|a| a.0 == 1).count()
    }
}
