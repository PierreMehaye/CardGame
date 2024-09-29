#[derive(Debug, PartialEq, Default, Clone)]
pub struct Card {
    pub name: &'static str,
    pub cost: i8,
    pub attack: i64,
    pub health: i64,
}

#[derive(PartialEq, Debug)]
pub struct Player {
    pub name: &'static str,
    pub mana: i8,
    pub max_mana: i8,
    pub health: i32,
    pub max_health: i8,
    pub deck: Deck,
}

impl Player
{
    pub fn is_player_alive(&self) -> bool {
        self.health > 0
    }

    pub fn damage(&mut self, damage_done:i32){
        self.health = self.health - damage_done
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct Deck {
    card_deck: Vec<Card>,
}

impl Deck {
    pub fn add_card(&mut self, card_to_add: Card) {
        self.card_deck.push(card_to_add)
    }

    pub fn deck_size(&self) -> usize {
        self.card_deck.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.card_deck.pop()
    }
}

pub struct Game<'a> {
    pub player1: &'a mut Player,
    pub player2: &'a mut Player,
    current_turn : i16
}

impl<'a> Game<'a> {
    fn current_player_mut(&mut self) -> &mut Player {
        if self.current_turn % 2 == 0{
            self.player2
        }
        else{
            self.player1
        }
        
    }

    fn current_player(&self) -> &Player {
        if self.current_turn % 2 == 0{
            self.player2
        }
        else{
            self.player1
        }
        
    }

    fn next_turn(&mut self) {
        self.current_turn +=1;
        let current_player = self.current_player_mut();
        if current_player.max_mana <= 10{
            current_player.max_mana +=1;
        }
        current_player.mana = current_player.max_mana;
    }
}

fn display_player(player: &Player){
    println!("{}:{} HP", player.name, player.health)
}

fn display_current_turn(game: &Game){
    println!("Current turn : {}", game.current_player().name)
}

fn display_split(){
    println!("--------------------------------")
}

pub fn display_game(game: &Game){
    display_split();
    display_player(game.player1);
    display_split();
    display_player(game.player2);
    display_split();
    display_current_turn(&game);
    display_split();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_player() {
        let mana = 1;
        let max_mana = 2;
        let health = 3;
        let max_health = 4;
        let name = "test_name";
        let player = Player {
            name: name,
            mana: mana,
            max_mana: max_mana,
            health: health,
            max_health: max_health,
            deck: Default::default(),
        };
        assert_eq!(player.name, name);
        assert_eq!(player.mana, mana);
        assert_eq!(player.max_mana, max_mana);
        assert_eq!(player.health, health);
        assert_eq!(player.max_health, max_health);
        assert_eq!(player.deck.deck_size(), 0);
    }

    #[test]
    fn player_alive_until_damaged(){
        let mut player = Player {
            name: "test",
            mana: 0,
            max_mana: 0,
            health: 2,
            max_health: 2,
            deck: Default::default()
        };
        assert_eq!(player.is_player_alive(), true);
        player.damage(1);
        assert_eq!(player.is_player_alive(), true);
        player.damage(1);
        assert_eq!(player.is_player_alive(), false);
    }

    #[test]
    fn create_card() {
        let cost = 1;
        let attack = 2;
        let health = 3;
        let name = "test_name";
        let card = Card {
            name: name,
            cost: cost,
            attack: attack,
            health: health,
        };
        assert_eq!(card.name, name);
        assert_eq!(card.cost, cost);
        assert_eq!(card.attack, attack);
        assert_eq!(card.health, health);
    }

    #[test]
    fn add_card_to_deck_then_draw() {
        let mut deck: Deck = Default::default();
        assert_eq!(0, deck.deck_size());
        let card = Card {
            name: "test",
            cost: 1,
            attack: 2,
            health: 3,
        };
        deck.add_card(card.clone());
        assert_eq!(1, deck.deck_size());
        let mut drawn_card = deck.draw();
        assert_eq!(card, drawn_card.unwrap_or_default());
        drawn_card = deck.draw();
        assert_eq!(None, drawn_card)
    }

    #[test]
    fn create_game_and_play_two_turns() {
        let mut player1 = Player {
            name: "player1",
            mana: 1,
            max_mana: 1,
            health: 3,
            max_health: 4,
            deck: Default::default(),
        };
        let mut player2 = Player {
            name: "player2",
            mana: 0,
            max_mana: 0,
            health: 3,
            max_health: 4,
            deck: Default::default(),
        };
        let mut game = Game {
            player1: &mut player1,
            player2: &mut player2,
            current_turn: 1
        };
        assert_eq!(game.current_player().name, "player1");
        assert_eq!(game.current_player().mana, 1);
        assert_eq!(game.current_player().max_mana, 1);
        game.next_turn();
        assert_eq!(game.current_player().name, "player2");
        assert_eq!(game.current_player().mana, 1);
        assert_eq!(game.current_player().max_mana, 1);
        game.next_turn();
        assert_eq!(game.current_player().name, "player1");
        assert_eq!(game.current_player().mana, 2);
        assert_eq!(game.current_player().max_mana, 2);
    }
}
