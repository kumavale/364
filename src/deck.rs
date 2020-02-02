use self::Suit::*;
use std::slice::Iter;
use std::fmt;

extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

extern crate ncurses;
use ncurses::*;

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

impl Suit {
    fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Spades, Clubs, Diamonds, Hearts];
        SUITS.iter()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub val: usize,
    pub suit: Suit,
}

#[derive(Clone, Copy)]
pub struct Deck {
    pub cards: [Card; 13*4],
}

impl fmt::Debug for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cards[..].fmt(f)
    }
}

impl Card {
    pub fn new() -> [Card; 13*4] {
        let mut suits = [Card { val: 0, suit: Spades }; 13*4];

        for (i, suit) in (0..).zip(Suit::iterator()) {
            for j in 0..13 {
                suits[13*i+j].val  = j+1;
                suits[13*i+j].suit = *suit;
            }
        }
        suits
    }

    pub fn draw(&self) {
        // prototype
        printw(&format!("{:?}\n", self));
    }
}

impl Deck {
    pub fn new() -> Self {
        Deck { cards: Card::new() }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn iter(&self) -> Iter<Card> {
        self.cards.iter()
    }
}

