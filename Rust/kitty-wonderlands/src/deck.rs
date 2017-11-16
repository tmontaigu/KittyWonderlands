use std::collections::VecDeque;

use card;

struct Deck {
    cards: VecDeque<Box<card::GameCard>>
}

