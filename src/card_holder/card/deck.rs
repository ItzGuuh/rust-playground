use rand::{Rng, thread_rng};
use strum::IntoEnumIterator;

use crate::card_holder::card::{Card, CardSuit, CardValue};
use crate::card_holder::CardHolder;

const MAX_CARDS: usize = 52;
const MIN_CARDS: usize = 0;

pub struct Deck {
    mount: CardHolder,
}

impl Deck {
    pub fn get_cards(&self) -> CardHolder {
        return self.mount.clone();
    }
    pub fn cut(&mut self, random_number: Option<usize>) {
        let cut_number = random_number.unwrap_or(thread_rng().gen_range(MIN_CARDS..MAX_CARDS));
        let mut top_half = self.mount.get_n_cards_from_top(Some(cut_number));
        top_half.add_cards(self.get_cards());
        self.mount = top_half;
    }
    pub fn full_shuffle(&mut self) {
        self.mount.shuffle();
        self.cut(None);
    }
}

impl Deck {
    pub fn create_new_deck() -> Deck {
        let mut cards = CardHolder::new(MAX_CARDS, None);
        for suit in CardSuit::iter() {
            for value in CardValue::iter() {
                cards.add_card(Card {
                    value,
                    suit: suit.clone(),
                })
            }
        }
        let deck = Deck { mount: cards };
        return deck;
    }
}
