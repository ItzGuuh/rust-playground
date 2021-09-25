mod deck;
use deck::Deck;

fn main() {
    println!("Hello, world!");
    let mut new_deck = Deck::create_new_deck();
    println!("---------------- Printing new deck ----------------");
    new_deck.print_deck();
    println!("---------------- Printing cut deck ----------------");
    new_deck.cut(Some(26));
    new_deck.print_deck();
    println!("---------------- Printing shuffled deck ----------------");
    new_deck.shuffle();
    new_deck.print_deck();
}
