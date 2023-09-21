//#![allow(dead_code)]

mod nim;
mod util;
use crate::nim::generalized::TakingGame;
use nim::{generalized::constructor::*, pit::Pit};
use std::time::Instant;
use evaluator::*;

fn main() {
    print_kayle_nimbers(40);
    //print_triangle_nimbers(5);
}

fn print_kayle_nimbers(max: u16){
    let mut eval = Evaluator::new();
    for i in 0..max{
        let g = Constructor::kayles(i).build();
        println!("{}:{}", i, eval.get_nimber(g));
    }
}


#[allow(dead_code)]
fn print_triangle_nimbers(max: u16) {
    let mut eval = Evaluator::new();
    for i in 0..max {
        let g = Constructor::triangle(i).build();
        println!("{}:{}", i, eval.get_nimber(g));
    }
}

#[allow(dead_code)]
fn test_reconstruction(pit: Pit) {
    println!("{}", pit);
    let g = pit.get_generalized();
    if g.clone().get_parts().len() == 0 {
        return;
    }
    let c = &g.get_parts()[0];
    println!("{:?}", c);
    //let reconstructed = Pit::try_reconstruct(c).unwrap();
    //println!("{}", reconstructed);
}
#[allow(dead_code)]
fn test_pit_game(pit: Pit) {
    let starting_test = Instant::now();

    let db = &mut Evaluator::new();
    println!("{}", pit);

    let g = pit.get_generalized();

    println!("{:?}", g);

    let nimber = db.get_nimber(g);

    println!("nimber: {}", nimber);

    println!("{:?}", starting_test.elapsed());

    //println!("{}", data_base);
}
