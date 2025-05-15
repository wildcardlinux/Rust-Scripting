#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    // List of suits
    // List of face values
    let suits = [
        String::from("Hearts"),
        String::from("Diamonds"),
        String::from("Clubs"),
        String::from("Spades"),
    ];
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
    // Create a deck of cards
    let deck = Deck {
            cards
    };
    
    println!("Here is your deck: {:#?}", deck);
}
