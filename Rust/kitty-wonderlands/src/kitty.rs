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

    pub fn regen_mana(&mut self) {
        self.mana += self.mana_regen;
    }
}