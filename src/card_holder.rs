use rand::{seq::SliceRandom, thread_rng};
use std::convert::TryFrom;

use crate::card_holder::card::Card;

pub mod card;

#[derive(Clone)]
pub struct CardHolder {
    cards: Vec<Card>,
    amount: usize,
}

impl CardHolder {
    pub fn allow_more_cards(&self) -> usize {
        return self.amount - self.cards.len();
    }
    pub fn add_cards(&mut self, cards: CardHolder) {
        if !(self.allow_more_cards() < cards.cards.len()) { return; }
        for card in cards.cards {
            self.cards.push(card);
        }
    }
    pub fn add_card(&mut self, card: Card) {
        if self.allow_more_cards() > 0 {
            self.cards.push(card);
        }
    }
    pub fn print_cards(&self) {
        for card in self.cards.iter() {
            println!("card {:?} of {:?}", card.value, card.suit);
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn get_n_cards_from_top(&mut self, random_number: Option<usize>) -> CardHolder {
        let n_top_numbers = random_number.unwrap_or(1);
        let mut retreived_cards = Vec::with_capacity(n_top_numbers);
        for _ in 1..u8::try_from(n_top_numbers).ok().unwrap() {
            let card = self.cards.pop();
            match card {
                Some(x) => retreived_cards.push(x),
                None => break,
            }
        }
        return CardHolder { cards: retreived_cards, amount: n_top_numbers};
    }
}

impl CardHolder {
    pub fn new(amount: usize, cards: Option<Vec<Card>>) -> CardHolder {
        let new_cards = cards.unwrap_or(Vec::with_capacity(amount));
        return CardHolder { cards: new_cards, amount }
    }
}
