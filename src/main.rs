mod deck;
use deck::Deck;

fn main() {
    let mut new_deck = Deck::create_new_deck();

    println!("---------------- Printing new deck ----------------");
    new_deck.print_deck();
    println!("---------------- Printing cut deck ----------------");
    new_deck.cut(Some(26));
    new_deck.print_deck();
    println!("---------------- Printing shuffled deck ----------------");
    new_deck.shuffle();
    new_deck.print_deck();
    println!("---------------- Printing full shuffled deck ----------------");
    new_deck.full_suffle();
    new_deck.print_deck();

    let my_cards = new_deck.get_n_cards_from_top(Some(5));
}
