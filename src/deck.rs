mod card {
    use strum::EnumIter;

    #[derive(Debug, EnumIter)]
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

    pub struct Card {
        pub value: CardValue,
        pub suit: CardSuit,
    }
}
use card::{Card, CardSuit, CardValue};
use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;

const MAX_CARDS: usize = 52;
const MIN_CARDS: usize = 0;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn add_card(&mut self, card: Card) {
        if self.cards.len() < 52 {
            self.cards.push(card);
        }
    }
    pub fn print_deck(&self) {
        for card in self.cards.iter() {
            println!("card {:?} of {:?}", card.value, card.suit);
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn cut(&mut self, random_number: Option<usize>) {
        let mut top_half = self
            .cards
            .split_off(random_number.unwrap_or(thread_rng().gen_range(MIN_CARDS..MAX_CARDS)));
        top_half.append(&mut self.cards);
        self.cards = top_half;
    }
}

impl Deck {
    pub fn create_new_deck() -> Deck {
        let mut cards = Vec::with_capacity(MAX_CARDS);
        for suit in CardSuit::iter() {
            for value in CardValue::iter() {
                cards.push(Card {
                    value: value,
                    suit: suit.clone(),
                })
            }
        }
        let deck = Deck { cards: cards };
        return deck;
    }
}
