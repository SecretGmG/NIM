use indicatif::ProgressIterator;
use std::{
    thread::{self},
    time::{Duration, Instant},
};
use taking_game::taking_game::constructor::Constructor;

fn main() {
    let games = vec![
        (Constructor::rect(1, 3).build(), false),
        (Constructor::rect(4, 1).build(), false),
        (Constructor::rect(100, 1).build(), false),
        (Constructor::rect(1, 101).build(), false),
        (Constructor::rect(2, 2).build(), true),
        (Constructor::rect(3, 3).build(), false),
        (Constructor::rect(3, 4).build(), false),
        (Constructor::rect(4, 4).build(), true),
        (Constructor::rect(5, 4).build(), false),
        (
            Constructor::rect(3, 6)
                .combine(Constructor::rect(6, 3).build())
                .build(),
            true,
        ),
        (
            Constructor::rect(1, 50)
                .combine(Constructor::rect(2, 9).build())
                .build(),
            false,
        ),
        (
            Constructor::rect(1, 10)
                .combine(Constructor::rect(2, 5).build())
                .connect_unit_to_all() 
                .build(),
            false,
        ),
        (
            Constructor::rect(1, 50)
                .combine(Constructor::rect(2, 9).build())
                .combine(Constructor::triangle(3).build())
                .build(),
            false,
        ),
        (
            Constructor::rect(2, 11)
                .combine(Constructor::rect(2, 11).build())
                .combine(Constructor::rect(2, 10).build())
                .build(),
            true,
        ),
        (Constructor::hyper_cube(3, 2).build(), true),
    ];

    // Time measurement
    let start = Instant::now();

    for (game, expecded_symmetry) in games.into_iter().progress() {
        if !(game.find_symmetry().is_some() == expecded_symmetry){
            println!("finding symmetry failed");
        }
    }

    let duration = start.elapsed();
    // Output
    println!("Time elapsed: {:.6?}", duration);
}

//cargo run --bin benchmark_symmetry_finder --no-default-features -- --optimized
//Time elapsed: 5ms


//cargo run --bin benchmark_symmetry_finder -- --optimized
//Time elapsed: 118.244500ms