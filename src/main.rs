use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();
        let suits = ["Spades", "Clubs", "Diamonds", "Hearts"];
        let values = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
        ];

        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Here is your deck: {deck:#?}");
}
