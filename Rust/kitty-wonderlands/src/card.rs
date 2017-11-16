use kitty::Kitty;

#[derive(Debug)]
pub enum CardError {
    OutOfMana,
}

pub type CardResult = Result<(), CardError>;

pub trait GameCard {
    fn name(&self) -> String;
    fn mana_cost(&self) -> u32;
    fn rarity(&self) -> u32;
    fn activate(&self, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult;
}

pub struct KittyCard{
    name: &'static str,
    mana_cost: u32,
    action: fn(&KittyCard, &mut Kitty, &mut Kitty) -> CardResult
}

impl GameCard for KittyCard {

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn mana_cost(&self) -> u32 {
        self.mana_cost
    }

    fn rarity(&self) -> u32 {
        (1 / self.mana_cost()) as u32 * 100
    }

    fn activate(&self, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult {
        (self.action)(self, owner, enemy)
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

    pub fn KittySteal() -> Self{
        KittyCard{
            name: "KittySteal",
            mana_cost: 10,
            action: kitty_steal_action
        }
    }

    pub fn KittyPanacea() -> Self{
        KittyCard{
            name: "KittyPanacea",
            mana_cost: 2,
            action: kitty_panacea_action
        }
    }

    pub fn KittyRazor() -> Self{
        KittyCard{
            name: "KittyRazor",
            mana_cost: 2,
            action: kitty_razor_action
        }
    }

    pub fn KittyHellIsOthers() -> Self{
        KittyCard{
            name: "KittyHellIsOthers",
            mana_cost: 100,
            action: kitty_hell_is_others_action
        }
    }
}


fn kitty_think_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult{ //shall return a result
    if owner.mana() < card.mana_cost() {
        return Err(CardError::OutOfMana);
    }

    owner.increase_mana_regen();
    owner.decrease_mana(card.mana_cost());
    Ok(())
}

fn kitty_steal_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult {
    if owner.mana() < card.mana_cost() {
        return Err(CardError::OutOfMana);
    }

    if enemy.mana_regen() > 1 {
        enemy.decrease_mana_regen();
    }
    owner.increase_mana_regen();
    owner.decrease_mana(10);
    Ok(())
}

fn kitty_panacea_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult{
    if owner.mana() < card.mana_cost() {
        return Err(CardError::OutOfMana);
    }

    owner.increase_health(10);
    owner.decrease_mana(card.mana_cost());
    Ok(())
}

fn kitty_razor_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult {
    if owner.mana() < card.mana_cost() {
        return Err(CardError::OutOfMana);
    }

    enemy.decrease_health(10);
    owner.decrease_mana(card.mana_cost());

    Ok(())
}

fn kitty_hell_is_others_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult {
    if owner.mana() < card.mana_cost() {
        return Err(CardError::OutOfMana);
    }

    enemy.decrease_health(u32::max_value());
    owner.decrease_mana(card.mana_cost());
    Ok(())
}