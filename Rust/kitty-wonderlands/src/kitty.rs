use card;

pub struct Kitty {
    health: u32,
    mana: u32,
    mana_regen: u32,
    hand: Vec<Box<card::GameCard>>,
}

impl Kitty {
    pub fn new() -> Self {
        Kitty{health: 50 , mana: 0, mana_regen: 1, hand: Vec::<Box<card::GameCard>>::new()}
    }

    pub fn health(&self) -> u32 {
        self.health
    }

    pub fn increase_health(&mut self, inc: u32) {
        self.health += inc;
    }

    pub fn decrease_health(&mut self, dec: u32) {
        self.health -= dec;
    }

    pub fn mana(&self) -> u32 {
        self.mana
    }

    pub fn mana_regen(&self) -> u32 {
        self.mana_regen
    }

    pub fn decrease_mana(&mut self, dec: u32) {
        self.mana -= dec;
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