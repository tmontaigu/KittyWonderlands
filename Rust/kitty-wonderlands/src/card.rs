use kitty::Kitty;

#[derive(Debug)]
#[derive(PartialEq)]
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

fn has_enough_mana<T: GameCard>(card: &T, kitty: &Kitty) -> CardResult{
    if kitty.mana() < card.mana_cost() {
        return Err(CardError::OutOfMana);
    }

    Ok(())
}

fn kitty_think_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult{ //shall return a result
    has_enough_mana(card, owner)?;

    owner.increase_mana_regen();
    owner.decrease_mana(card.mana_cost());
    Ok(())
}

fn kitty_steal_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult {
    has_enough_mana(card, owner)?;

    if enemy.mana_regen() >= 1 {
        enemy.decrease_mana_regen();
    }
    owner.increase_mana_regen();
    owner.decrease_mana(10);
    Ok(())
}

fn kitty_panacea_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult{
    has_enough_mana(card, owner)?;

    owner.increase_health(10);
    owner.decrease_mana(card.mana_cost());
    Ok(())
}

fn kitty_razor_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult {
    has_enough_mana(card, owner)?;

    enemy.decrease_health(10);
    owner.decrease_mana(card.mana_cost());

    Ok(())
}

fn kitty_hell_is_others_action(card: &KittyCard, owner: &mut Kitty, enemy: &mut Kitty) -> CardResult {
    has_enough_mana(card, owner)?;

    enemy.decrease_health(u32::max_value());
    owner.decrease_mana(card.mana_cost());
    Ok(())
}



#[cfg(test)]
mod test {
    use card;
    use kitty::Kitty;
    use card::GameCard;
    use super::{KittyCard, CardError};

    #[test]
    fn test_kitty_think() {
        let starting_mana: u32 = 5;
        let starting_mana_regen: u32 = 1;

        let mut k1 = Kitty::new_with_stats(50, starting_mana, starting_mana_regen);
        let mut k2 = Kitty::new();

        let k_think = KittyCard::KittyThink();
        assert_eq!(Ok(()), k_think.activate(&mut k1, &mut k2));

        assert_eq!(k1.mana(), starting_mana - k_think.mana_cost());
        assert_eq!(k1.mana_regen(), starting_mana_regen + 1);
    }

    #[test]
    fn test_kitty_think_oom() {
        let mut k1 = Kitty::new_with_stats(50, 0, 1);
        let mut k2 = Kitty::new();

        let hellio = KittyCard::KittyThink();
        assert_eq!(Err(CardError::OutOfMana), hellio.activate(&mut k1, &mut k2));
    }


    #[test]
    fn test_steal() {
        let starting_mana: u32 = 10;
        let starting_mana_regen: u32 = 1;

        let mut k1 = Kitty::new_with_stats(50, starting_mana, starting_mana_regen);
        let mut k2 = Kitty::new_with_stats(50, starting_mana, 1);

        let k_steal = KittyCard::KittySteal();

        assert_eq!(Ok(()), k_steal.activate(&mut k1, &mut k2));

        assert_eq!(k1.mana(), starting_mana - k_steal.mana_cost());
        assert_eq!(k1.mana_regen(), starting_mana_regen + 1);
        assert_eq!(k2.mana_regen(), 0);

    }

    #[test]
    fn test_steal_oom() {
        let mut k1 = Kitty::new_with_stats(50, 0, 1);
        let mut k2 = Kitty::new();

        let hellio = KittyCard::KittyThink();
        assert_eq!(Err(CardError::OutOfMana), hellio.activate(&mut k1, &mut k2));
    }


    #[test]
    fn test_panacea() {
        let starting_mana: u32 = 10;
        let starting_mana_regen: u32 = 1;

        let mut k1 = Kitty::new_with_stats(50, starting_mana, starting_mana_regen);
        let mut k2 = Kitty::new_with_stats(50, starting_mana, 1);

        let k_panacea = KittyCard::KittyPanacea();

        assert_eq!(Ok(()), k_panacea.activate(&mut k1, &mut k2));
        assert_eq!(k1.mana(), starting_mana - k_panacea.mana_cost());
        assert_eq!(k1.health(), 60);
    }

    #[test]
    fn test_razor() {
        let starting_mana: u32 = 10;
        let starting_mana_regen: u32 = 1;

        let mut k1 = Kitty::new_with_stats(50, starting_mana, starting_mana_regen);
        let mut k2 = Kitty::new_with_stats(50, starting_mana, 1);

        let k_razor = KittyCard::KittyRazor();

        assert_eq!(Ok(()), k_razor.activate(&mut k1, &mut k2));
        assert_eq!(k1.mana(), starting_mana - k_razor.mana_cost());
        assert_eq!(k2.health(), 40);
    }

    #[test]
    fn test_hell_is_others() {
        let starting_mana: u32 = 100;
        let starting_mana_regen: u32 = 1;

        let mut k1 = Kitty::new_with_stats(50, starting_mana, starting_mana_regen);
        let mut k2 = Kitty::new_with_stats(50, starting_mana, 1);

        let k_hio = KittyCard::KittyHellIsOthers();

        assert_eq!(Ok(()), k_hio.activate(&mut k1, &mut k2));
        assert_eq!(k1.mana(), starting_mana - k_hio.mana_cost());
        assert_eq!(k2.health(), 0);
    }
}