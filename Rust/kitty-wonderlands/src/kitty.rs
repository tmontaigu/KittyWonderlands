// Kittys are the players

pub struct Kitty {
    health: i32,
    mana: i32,
    mana_regen: i32
}

impl Kitty {
    pub fn new() -> Self {
        Kitty{health: 50 , mana: 0, mana_regen: 1}
    }


    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn mana(&self) -> i32 {
        self.mana
    }

    pub fn mana_regen(&self) -> i32 {
        self.mana_regen
    }

    pub fn decrease_mana(&mut self, dec : i32) {
        self.mana -= dec;
    }

    pub fn regen_mana(&mut self) {
        self.mana += self.mana_regen;
    }

    pub fn set_mana_regen(&mut self, new_value: i32) {
        self.mana_regen = new_value;
    }
}