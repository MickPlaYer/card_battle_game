use card_game::{Deck, Game, SystemCall};
use read_input::shortcut::input;

fn main() {
    let mut game = Game::new();
    loop {
        match game.update() {
            SystemCall::WaitForInput(message) => {
                println!("{}", message);
                game.input(input().get());
            }
            SystemCall::Next => (),
            SystemCall::Stop => break,
        }
    }

    let mut deck = Deck::new();

    fn draw_from_deck(deck: &mut Deck) {
        let card = deck.draw();
        let card = match card {
            Some(card) => card,
            None => {
                println!("Shuffle the full deck.");
                deck.full_shuffle();
                deck.draw().unwrap()
            }
        };
        println!("Draw a card: {}", card);
    }

    for _ in 0..60 {
        draw_from_deck(&mut deck);
    }
}
