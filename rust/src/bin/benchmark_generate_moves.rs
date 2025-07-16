use indicatif::ProgressIterator;
use std::{
    thread::{self},
    time::{Duration, Instant},
};
use taking_game::taking_game::constructor::Constructor;

fn main() {
    // Setup
    let games = vec![
        //(Constructor::rect(1, 3).build(), Some(3)),
        //(Constructor::rect(4, 1).build(), Some(4)),
        //(Constructor::rect(100, 1).build(), Some(100)),
        //(Constructor::rect(1, 101).build(), Some(101)),
        (Constructor::rect(2, 2).build(), Some(0)),
        //(Constructor::rect(3, 3).build(), Some(0)),
        //(Constructor::rect(3, 4).build(), None),
        //(Constructor::rect(4, 4).build(), Some(0)),
        //(Constructor::rect(5, 4).build(), None),
        //(Constructor::rect(5, 5).build(), None),
        //(Constructor::rect(5, 6).build(), None),
        //(Constructor::rect(6, 6).build(), None),
        //(Constructor::hyper_cube(3, 2).build(), Some(0)),
    ];

    // Time measurement
    let start = Instant::now();

    for (game, _) in games.into_iter().progress() {
        println!("{:#?}", game.get_moves());
    }

    let duration = start.elapsed();
    // Output
    println!("Time elapsed: {:.6?}", duration);
}
//cargo run --bin benchmark_generate_moves -- --optimized
//Time elapsed: 169.712600ms
//cargo run --bin benchmark_generate_moves --features "no_sort" -- --optimized
//Time elapsed: 137.701200ms