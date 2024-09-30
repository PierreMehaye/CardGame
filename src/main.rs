use card_game::*;
use std::io;

pub fn get_int_from_user() -> i8{
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    match  input_string.trim().parse::<i8>(){
        Ok(n) => n,
        Err(_e) => {
            println!("Invalid input : not a valid number");
            get_int_from_user()
        }
    }
}

pub fn main() {
    let mut player1 = Player {
        name: "Player 1",
        mana: 1,
        max_mana: 1,
        health: 2,
        max_health: 2,
        deck: Default::default()
    };
    let mut player2 = Player {
        name: "Player 2",
        mana: 0,
        max_mana: 0,
        health: 2,
        max_health: 2,
        deck: Default::default()
    };
    let mut game = Game {
        player1: &mut player1,
        player2: &mut player2,
        current_turn: 1
    };
    while !game.check_for_winner(){
        display_game(&game);
        game.display_current_player_possible_actions();
        let user_input = get_int_from_user();
        game.action(user_input);
    }
    
}
