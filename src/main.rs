use crate::card_holder::card::deck::Deck;

mod card_holder;

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
