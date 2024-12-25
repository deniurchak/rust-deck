use std::env;
mod game;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_cards = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(5) // Default to 5 if parsing fails
    } else {
        5 // Default to 5 if no argument is provided
    };
    let mut deck = game::Deck::new();
    deck.shuffle();
    let hand = deck.deal(num_cards);
    println!("Here is your hand: {hand:#?}");
}
