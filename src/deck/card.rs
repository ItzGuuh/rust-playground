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