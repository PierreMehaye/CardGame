pub struct Card {
    pub name: &'static str,
    pub cost: i8,
    pub attack: i64,
    pub health: i64,
}

pub struct Player {
    pub name: &'static str,
    pub mana: i8,
    pub max_mana: i8,
    pub health: i8,
    pub max_health: i8,
}

#[derive(Default)]
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
        };
        assert_eq!(player.name, name);
        assert_eq!(player.mana, mana);
        assert_eq!(player.max_mana, max_mana);
        assert_eq!(player.health, health);
        assert_eq!(player.max_health, max_health);
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
    fn add_card_to_deck() {
        let mut deck: Deck = Default::default();
        assert_eq!(0, deck.deck_size());
        let card = Card {
            name: "test",
            cost: 1,
            attack: 2,
            health: 3,
        };
        deck.add_card(card);
        assert_eq!(1, deck.deck_size());
    }
}
