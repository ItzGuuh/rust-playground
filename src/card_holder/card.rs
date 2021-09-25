use strum::EnumIter;

pub mod deck;
pub mod hand;

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
