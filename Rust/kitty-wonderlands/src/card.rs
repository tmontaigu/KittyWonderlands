use kitty;

pub trait GameCard {
    fn name(&self) -> String;
    fn mana_cost(&self) -> i32;
    fn activate(&self, kitty: &mut kitty::Kitty);
}

pub struct KittyCard{
    name: &'static str,
    mana_cost: i32,
    action: fn(&KittyCard, &mut kitty::Kitty)
}

impl GameCard for KittyCard {

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn mana_cost(&self) -> i32 {
        self.mana_cost
    }

    fn activate(&self, kitty: &mut kitty::Kitty) {
        (self.action)(self, kitty)
    }

}

impl KittyCard {
    pub fn KittyThink() -> Self{
        KittyCard{
            name: "KittyThink",
            mana_cost: 5,
            action: kitty_think_action
        }
    }
}


fn kitty_think_action(card: &KittyCard, kitty: &mut kitty::Kitty) { //shall return a result
    if (kitty.mana() >= card.mana_cost()) {
        let kitty_regen = kitty.mana_regen();
        kitty.set_mana_regen(kitty_regen + 1);
        kitty.decrease_mana(card.mana_cost())
    }
}

