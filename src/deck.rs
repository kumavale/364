use self::Suit::*;
use std::slice::Iter;
use std::fmt;
use crate::aa::*;

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
        match self.val {
            1  => addstr(&to_suit(ACE,   self.suit)),
            2  => addstr(&to_suit(TWO,   self.suit)),
            3  => addstr(&to_suit(THREE, self.suit)),
            4  => addstr(&to_suit(FOUR,  self.suit)),
            5  => addstr(&to_suit(FIVE,  self.suit)),
            6  => addstr(&to_suit(SIX,   self.suit)),
            7  => addstr(&to_suit(SEVEN, self.suit)),
            8  => addstr(&to_suit(EIGHT, self.suit)),
            9  => addstr(&to_suit(NINE,  self.suit)),
            10 => addstr(&to_suit(TEN,   self.suit)),
            11 => addstr(&to_suit(JACK,  self.suit)),
            12 => addstr(&to_suit(QUEEN, self.suit)),
            13 => addstr(&to_suit(KING,  self.suit)),
            _  => panic!("invalid val: {}", self.val),
        };
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

fn to_suit(aa: &str, suit: Suit) -> String {
    match suit {
        Spades   => aa.replace("#", "♠"),
        Clubs    => aa.replace("#", "♣"),
        Diamonds => aa.replace("#", "♦"),
        Hearts   => aa.replace("#", "♥"),
    }
}

