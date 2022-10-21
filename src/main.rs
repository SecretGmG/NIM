mod nim;
use std::time::Instant;

use crate::nim::generalized::{nimber::DataBase, GeneralizedNimGame};
use nim::{pit::Pit};


fn main() {

    let now = Instant::now();

    test_i_games(1, 6, 3, (0,1,0),(1,0));   

    println!("{:?}", now.elapsed());

}


fn test_generalized_nim_game(vec_of_groups : Vec<Vec<u16>>){

    let s = "------------------";

    println!("vec_of_groups:\n{:?}\n{}", vec_of_groups, s);

    let g = GeneralizedNimGame::new(vec_of_groups);

    println!("{}\n{}{}", g,s,s);

    let split = g.get_split();

    println!("split:\n");

    for part in split{

        println!("{}\n{}", part,s);
    }

}


fn test_i_games(i: u16, x: u8, y: u8, cell_type_distribution: (u32,u32,u32), wall_type_distribution: (u32,u32)){
    for _ in 0..i{
        let new_pit = Pit::random_rect(cell_type_distribution, wall_type_distribution, x, y);

        println!("{}", new_pit);
        let mut data_base = DataBase::none();
        println!("Nimber: {}", new_pit.get_generalized().calculate_nimber(&mut data_base));
        println!("---------------------------");

        //println!("{}", data_base);
    }
}
