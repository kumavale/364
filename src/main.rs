extern crate ncurses;

mod aa;
mod deck;
use deck::*;

use ncurses::*;
use std::time::Instant;

fn main() {
    // Setup locale
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");

    // Setup ncurses
    initscr();
    raw();
    noecho();

    let mut deck = Deck::new();
    deck.shuffle();

    addstr("press space key to start...");
    while getch() != 0x20 {}

    // timer start
    let start_time = Instant::now();

    for card in deck.iter() {
        clear();
        card.draw();
        getch();
    }

    endwin();

    // print time
    let end_time = start_time.elapsed();
    println!("elapsed: {}sec", end_time.as_secs());
}

