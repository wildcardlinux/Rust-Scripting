
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::rng;
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}


impl Deck {
    fn new() -> Self {
        // List of suits
    let suits = [
        String::from("Hearts"),
        String::from("Diamonds"),
        String::from("Clubs"),
        String::from("Spades"),
    ];
    // List of face values
    let face_values = [
        String::from("Ace"),
        String::from("Two"),
        String::from("Three"),
        String::from("Four"),
        String::from("Five"),
        String::from("Six"),
        String::from("Seven"),
        String::from("Eight"),
        String::from("Nine"),
        String::from("Ten"),
        String::from("Jack"),
        String::from("Queen"),
        String::from("King"),
    ];
        // Cards Drawn
    let mut cards = vec![];

    for suits in suits.iter() {
        for face_values in face_values.iter() {
            let card = format!("{} of {}", face_values, suits);
            cards.push(card);
        }
    };
    // Return the deck
    return Deck {
        cards
    };
}

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
   let mut deck = Deck::new(); 
    // Shuffle the deck
    deck.shuffle();
    // Print the shuffled deck
    println!("Here is your deck: {:#?}", deck);
}
