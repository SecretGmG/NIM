#![allow(dead_code)]

mod nim;
use std::time::Instant;

use nim::pit::Pit;

use crate::nim::generalized::data_base::DataBase;


fn main() {

    test_pit_game(Pit::empty_rect(5, 5));

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

    //println!("{}", data_base);
}

