use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Display;

pub struct Deck {
    cards: Vec<Box<Card>>,
    drawn_cards: Vec<Box<Card>>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for _ in 0..2 {
            for n in 1..=10 {
                cards.push(Box::new(Card::Number(n)));
            }
            cards.push(Box::new(Card::Double));
            cards.push(Box::new(Card::Half));
            cards.push(Box::new(Card::Add));
        }
        cards.shuffle(&mut thread_rng());
        let drawn_cards = Vec::new();
        Deck { cards, drawn_cards }
    }

    pub fn draw(&mut self) -> Option<&Card> {
        let card = match self.cards.pop() {
            Some(card) => card,
            None => return None,
        };
        self.drawn_cards.push(card);
        Some(self.drawn_cards.last().unwrap())
    }

    pub fn full_shuffle(&mut self) {
        self.cards.append(&mut self.drawn_cards);
        self.cards.shuffle(&mut thread_rng())
    }
}

#[derive(Debug, Clone)]
pub enum Card {
    Number(u32),
    Double,
    Half,
    Add,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let describe = match self {
            Card::Number(number) => number.to_string(),
            _ => format!("{:?}", self),
        };

        write!(f, "Card[{}]", describe)
    }
}
