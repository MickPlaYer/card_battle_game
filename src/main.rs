use card_battle_game::{Game, SystemCall};
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
}
