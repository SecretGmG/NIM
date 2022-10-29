#![allow(dead_code)]

mod nim;
use std::time::Instant;
use nim::pit::{Pit, Cell, Wall};

use crate::nim::generalized::data_base::DataBase;



fn main() {
    test_pit_game(get_default_test_case());
}

fn get_default_test_case() -> Pit{

    //new version: nimber 1, 2.6 s, 669 entries
    //new version with 2 passes: nimber 1 2.5 s 585 entries
    //new version with 3 passes: nimber 1 2.7 s 566 entries
    //new version with 4 passes: nimber 1 3.0 s 560 entries
    //new version with 5 passes: nimber 1 3.2 s 554 entries
    //new version with 6 passes: nimber 1 3.9 s 554 entries


    let o = (Cell::On, Wall::None, Wall::None);
    let o_ = (Cell::On, Wall::None, Wall::Wall);

    return Pit::new(
        vec![
            vec![o,  o_,  o,  o],
            vec![o_, o_,  o,  o],
            vec![o_, o_,  o,  o],
            vec![o_, o ,  o,  o],
        ]);

}



fn test_pit_game(pit :Pit){
    let starting_test = Instant::now();

    let data_base = &mut DataBase::new();
    println!("{}", pit);

    let generalized = pit.get_generalized();
    
    println!("{}", generalized);

    let nimber =generalized.get_nimber(data_base);

    println!("nimber: {}", nimber);

    println!("{:?}", starting_test.elapsed());
    println!("{}", data_base.len());
}