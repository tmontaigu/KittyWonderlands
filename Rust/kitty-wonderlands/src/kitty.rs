use card;
use deck;

pub struct Kitty {
    health: u32,
    mana: u32,
    mana_regen: u32,
    hand: Vec<Box<card::GameCard>>,
    deck: deck::Deck,
}

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
}