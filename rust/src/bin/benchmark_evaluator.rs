use evaluator::Evaluator;
use indicatif::ProgressIterator;
use std::{
    thread::{self},
    time::{Duration, Instant},
};
use taking_game::taking_game::constructor::Constructor;

fn main() {
    // Setup
    let mut eval = Evaluator::new();

    let games = vec![
        (Constructor::rect(1, 3).build(), Some(3)),
        (Constructor::rect(4, 1).build(), Some(4)),
        (Constructor::rect(100, 1).build(), Some(100)),
        (Constructor::rect(1, 101).build(), Some(101)),
        (Constructor::rect(2, 2).build(), Some(0)),
        (Constructor::rect(3, 3).build(), Some(0)),
        (Constructor::rect(3, 4).build(), None),
        (Constructor::rect(4, 4).build(), Some(0)),
        (Constructor::rect(5, 4).build(), None),
        (
            Constructor::rect(3, 6)
                .combine(Constructor::rect(6, 3).build())
                .build(),
            Some(0),
        ),
        (
            Constructor::rect(1, 50)
                .combine(Constructor::rect(2, 9).build())
                .build(),
            None,
        ),
        (
            Constructor::rect(1, 10)
                .combine(Constructor::rect(2, 5).build())
                .connect_unit_to_all() 
                .build(),
            None,
        ),
        (
            Constructor::rect(1, 50)
                .combine(Constructor::rect(2, 9).build())
                .combine(Constructor::triangle(3).build())
                .build(),
            None,
        ),
        (
            Constructor::rect(2, 11)
                .combine(Constructor::rect(2, 11).build())
                .combine(Constructor::rect(2, 10).build())
                .build(),
            Some(0),
        ),
        (Constructor::hyper_cube(3, 2).build(), Some(0)),
    ];

    let cancel_flag = eval.get_cancel_flag();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(60));
        cancel_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    });

    // Time measurement
    let start = Instant::now();

    for (game, maybe_expected_nimber) in games.into_iter().progress() {
        let maybe_nimber = eval.get_nimber(game);
        match (maybe_nimber, maybe_expected_nimber) {
            (None, _) => println!("nimber computation failed"),
            (Some(_), None) => (),
            (Some(nimber), Some(expected_nimber)) => {
                if nimber != expected_nimber {
                    println!("Error: expected{expected_nimber}, found {nimber}")
                }
            }
        }
    }

    let duration = start.elapsed();
    // Output
    println!("Time elapsed: {:.6?}", duration);
    println!("Cache entries: {:.6?}", eval.get_cache_size());
}

//cargo run --bin benchmark_evaluator --no-default-features -- --optimized
//Time elapsed: 14.361108s
//Cache entries: 4898


//cargo run --bin benchmark_evaluator -- --optimized
// ~ 17%
//Time elapsed: 14.985881s
//Cache entries: 4898
