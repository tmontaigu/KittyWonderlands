use std::collections::VecDeque;

use card;

pub struct Deck {
    cards: VecDeque<Box<card::GameCard>>,
    max_capacity: usize,
}


impl Deck {
    pub fn new() -> Self {
        Deck {
                cards: VecDeque::<Box<card::GameCard>>::with_capacity(50)
                max_capacity: 50
            }
    }

    pub fn pick_card(&mut self) -> Option<Box<card::GameCard>> {
        self.cards.pop_front()
    }

    pub fn put_back<T>(&mut self, card: Box<T>)
    where T: card::GameCard + 'static {
        self.cards.push_back(card);
    }

    fn populate_deck(&mut self) {
        for i in range
    }
}
