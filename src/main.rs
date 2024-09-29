use card_game::*;


pub fn main() {
    let player1 = Player {
        name: "Player 1",
        mana: 0,
        max_mana: 0,
        health: 2,
        max_health: 2,
        deck: Default::default()
    };
    let player2 = Player {
        name: "Player 2",
        mana: 0,
        max_mana: 0,
        health: 2,
        max_health: 2,
        deck: Default::default()
    };
    let game = Game {
        player1: &player1,
        player2: &player2,
        current_player: &player1,
    };
    display_game(&game);
}
