use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::{EnumIter, IntoEnumIterator};
use std::convert::TryFrom;

#[derive(Clone)]
pub struct CardAmount {
    cards: Vec<Card>,
    card_amount: usize,
}

impl CardAmount {
    pub fn allow_more_cards(&self) -> usize {
        return self.card_amount - self.cards.len();
    }
    pub fn add_cards(&mut self, cards: CardAmount) {
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
    pub fn get_n_cards_from_top(&mut self, random_number: Option<usize>) -> CardAmount {
        let n_top_numbers = random_number.unwrap_or(1);
        let mut retreived_cards = Vec::with_capacity(n_top_numbers);
        for i in 1..u8::try_from(n_top_numbers).ok().unwrap() {
            let card = self.cards.pop();
            match card {
                Some(x) => retreived_cards.push(x),
                None => break,
            }
        }
        return CardAmount { cards: retreived_cards, card_amount: n_top_numbers};
    }
}

impl CardAmount {
    pub fn new(card_amount: usize, cards: Option<Vec<Card>>) -> CardAmount {
        let new_cards = cards.unwrap_or(Vec::with_capacity(card_amount));
        return CardAmount { cards: new_cards, card_amount }
    }
}





#[derive(Debug, EnumIter, Clone)]
pub enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, EnumIter, Clone)]
pub enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}




const MAX_CARDS: usize = 52;
const MIN_CARDS: usize = 0;

pub struct Deck {
    mount: CardAmount,
}
impl Deck {
    pub fn get_cards(&self) -> CardAmount {
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
        let mut cards = CardAmount::new(MAX_CARDS, None);
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


fn main() {
    let mut new_deck = Deck::create_new_deck();

    println!("---------------- Printing new deck ----------------");
    new_deck.get_cards().print_cards();
    println!("---------------- Printing cut deck ----------------");
    new_deck.cut(Some(26));
    new_deck.get_cards().print_cards();
    println!("---------------- Printing shuffled deck ----------------");
    new_deck.get_cards().shuffle();
    new_deck.get_cards().print_cards();
    println!("---------------- Printing full shuffled deck ----------------");
    new_deck.full_shuffle();
    new_deck.get_cards().print_cards();

    let my_cards = new_deck.get_cards().get_n_cards_from_top(Some(5));
}
