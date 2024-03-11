use advent_of_code::load_data;
use std::io::BufRead;
use std::str::FromStr;

const ADVENT_NUM: &str = "4";

fn main() {
    let buffer = load_data(ADVENT_NUM, "input.txt").unwrap();
    let mut cards = Vec::new();
    let mut stack = EffectStack::default();
    for line in buffer.lines() {
        let card: ScratchCard = line
            .expect("Couldn't spearate lines")
            .parse()
            .expect("Error parsing line");

        stack.handle_card(card);
        stack.step();

        println!("{}; Stack => {:?}", card.get_points(), stack);

        cards.push(card);
    }

    let total: u32 = cards.into_iter().map(|c| c.get_points()).sum();
    println!("Total points : {}", total);
}

#[derive(Debug, Clone, Copy)]
struct Effect {
    time: usize,
    power: u32,
}

#[derive(Debug, Clone, Default)]
pub struct EffectStack(Vec<Effect>, u32);

impl EffectStack {
    pub fn handle_card(&mut self, c: ScratchCard) {
        let power = self.clone().get_current() + 1;
        self.1 += power;
        println!("Instances => {}", power);
        self.0.push(Effect {
            time: c.get_num_match() + 1,
            power,
        })
    }

    pub fn get_current(self) -> u32 {
        self.0.iter().map(|e| e.power).sum()
    }

    pub fn step(&mut self) {
        self.0.iter_mut().for_each(|e| e.time -= 1);
        self.0.retain(|e| e.time != 0);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScratchCard {
    _name: u32,
    winning_numbers: [u8; 10],
    scratch_numbers: [u8; 25],
}

#[derive(Debug)]
pub struct ScratchCardParsingError(String);

impl FromStr for ScratchCard {
    type Err = ScratchCardParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, data) = match s.split_once(':') {
            Some(a) => a,
            None => return Err(ScratchCardParsingError("Aled".to_string())),
        };

        let _name: u32;
        if let Some((_, name_txt)) = label.split_once(' ') {
            _name = match name_txt.trim().parse() {
                Ok(a) => a,
                Err(_) => return Err(ScratchCardParsingError("NAME to u8".to_string())),
            };
        } else {
            return Err(ScratchCardParsingError("Name split".to_string()));
        }

        let mut winning_numbers: [u8; 10] = [0; 10];
        let mut scratch_numbers: [u8; 25] = [1; 25];
        if let Some((winning_text, gotten_text)) = data.split_once('|') {
            for (index, num_txt) in winning_text.split_whitespace().enumerate() {
                if let Ok(val) = num_txt.trim().parse() {
                    winning_numbers[index] = val;
                } else {
                    return Err(ScratchCardParsingError("Winning err".to_string()));
                }
            }
            for (index, num_txt) in gotten_text.split_whitespace().enumerate() {
                if let Ok(val) = num_txt.trim().parse() {
                    scratch_numbers[index] = val;
                } else {
                    return Err(ScratchCardParsingError("SCratch err".to_string()));
                }
            }
        } else {
            return Err(ScratchCardParsingError("data split".to_string()));
        }

        Ok(ScratchCard {
            _name,
            winning_numbers,
            scratch_numbers,
        })
    }
}

impl ScratchCard {
    pub fn get_num_match(self) -> usize {
        self.scratch_numbers
            .into_iter()
            .filter(|m| self.winning_numbers.contains(m))
            .count()
    }

    pub fn get_points(self) -> u32 {
        let winning = self.get_num_match();
        if winning > 0 {
            1 << (winning - 1)
        } else {
            0
        }
    }
}
