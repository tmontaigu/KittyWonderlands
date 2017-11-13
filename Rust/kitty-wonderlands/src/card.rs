use kitty;

pub trait GameCard {
    fn new() -> Self;
    fn name(&self) -> String;
    fn mana_cost(&self) -> i32;
    fn activate(&self, kitty: &mut kitty::Kitty);
}


impl GameCard for KittyCard {
    fn name(&self) -> String {
        self.name.as_string()
    }

    fn mana_cost(&self) -> i32 {
        self.mana_cost
    }

    fn activate(&self, kitty: &mut kitty::Kitty) {
        self.action(self, kitty)
    }
}

pub struct KittyCard{
    name: &'static str,
    mana_cost: i32,
    action: fn(&KittyCard, &mut kitty::Kitty)
}


fn kitty_think_action(card &KittyCard, kitty: &mut kitty::Kitty) {

}

