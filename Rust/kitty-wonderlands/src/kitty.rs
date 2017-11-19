use card;
use deck;

use rand;
use rand::Rng;


pub struct Kitty {
    health: u32,
    mana: u32,
    mana_regen: u32,
    hand: Vec<Box<card::GameCard>>,
    deck: deck::Deck,
}

const max_cards_in_hand: u8 = 5;

impl Kitty {
    pub fn new() -> Self {
        Kitty::new_with_stats(50, 0, 1)
    }

    pub fn new_with_stats(health: u32, mana: u32, mana_regen: u32) -> Self {
        Kitty {
            health: health,
            mana: mana,
            mana_regen: mana_regen,
            hand: Vec::<card::BoxedCard>::new(),
            deck: deck::Deck::new(),
        }
    }

    pub fn health(&self) -> u32 {
        self.health
    }

    pub fn increase_health(&mut self, inc: u32) {
        self.health += inc;
    }

    pub fn decrease_health(&mut self, dec: u32) {
        self.health = self.health.saturating_sub(dec);
    }

    pub fn mana(&self) -> u32 {
        self.mana
    }

    pub fn mana_regen(&self) -> u32 {
        self.mana_regen
    }

    pub fn decrease_mana(&mut self, dec: u32) {
        self.mana = self.mana.saturating_sub(dec);
    }

    pub fn regen_mana(&mut self) {
        self.mana += self.mana_regen;
    }

    pub fn increase_mana_regen(&mut self) {
        self.mana_regen += 1;
    }

    pub fn decrease_mana_regen(&mut self) {
        self.mana_regen -= 1;
    }

    pub fn create_deck(&mut self) {
        self.deck.populate_deck();
    }

    pub fn fill_hand(&mut self) {
        for _ in 0..(max_cards_in_hand as usize - self.hand.len()) {
            self.hand.push(self.deck.pick_card().unwrap());
        }
    }

    pub fn select_card(&mut self) -> &card::GameCard {
        let n = rand::random::<u8>() % max_cards_in_hand;
        &self.hand[n as usize]
    }
}