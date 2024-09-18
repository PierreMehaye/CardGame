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
    pub health: i8,
    pub max_health: i8,
    deck: Deck,
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
    player1: &'a Player,
    player2: &'a Player,
    current_player: &'a Player,
}

impl<'a> Game<'a> {
    fn current_player(&self) -> &Player {
        &self.current_player
    }

    fn next_turn(&mut self) {
        if self.current_player == self.player1 {
            self.current_player = self.player2;
        } else {
            self.current_player = self.player1;
        }
    }
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
        let player1 = Player {
            name: "player1",
            mana: 1,
            max_mana: 2,
            health: 3,
            max_health: 4,
            deck: Default::default(),
        };
        let player2 = Player {
            name: "player2",
            mana: 1,
            max_mana: 2,
            health: 3,
            max_health: 4,
            deck: Default::default(),
        };
        let mut game = Game {
            player1: &player1,
            player2: &player2,
            current_player: &player1,
        };
        assert_eq!(game.current_player(), &player1);
        game.next_turn();
        assert_eq!(game.current_player(), &player2);
        game.next_turn();
        assert_eq!(game.current_player(), &player1);
    }
}
