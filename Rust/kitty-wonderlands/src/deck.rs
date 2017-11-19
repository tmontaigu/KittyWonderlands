use std::collections::VecDeque;
use rand;
use rand::Rng;

use card::{GameCard, KittyCard, rarity_sum};


pub struct Deck {
    cards: VecDeque<Box<GameCard>>,
    max_capacity: usize,
}


impl Deck {
    pub fn new() -> Self {
        Deck {
                cards: VecDeque::<Box<GameCard>>::with_capacity(50),
                max_capacity: 50
            }
    }

    pub fn pick_card(&mut self) -> Option<Box<GameCard>> {
        self.cards.pop_front()
    }

    pub fn put_back<T>(&mut self, card: Box<T>)
    where T: GameCard + 'static {
        self.cards.push_back(card);
    }

    pub fn populate_deck(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..self.max_capacity {
            let n = rng.gen::<u32>() % rarity_sum;
            let card = match n {
                1  => KittyCard::KittyHellIsOthers(),
                2...11 => KittyCard::KittySteal(),
                12...31 => KittyCard::KittyThink(),
                32...71 => KittyCard::KittyPanacea(),
                _ => KittyCard::KittyRazor()
            };
            self.cards.push_back(Box::new(card));
        }
    }
}
