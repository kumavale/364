extern crate ncurses;

mod aa;
use aa::*;
mod deck;
use deck::*;

use ncurses::*;

fn main() {
    initscr();
    raw();
    noecho();

    let mut deck = Deck::new();
    deck.shuffle();

    printw("press space key to start...");
    while getch() != 0x20 {}

    // time start

    for card in deck.iter() {
        clear();
        card.draw();
        getch();
    }

    endwin();

    // time stop, print
}

