mod nim;
use crate::nim::generalized::nimber::DataBase;
use nim::{pit::Pit};

fn main() {
       test_i_games(10, 4, 3, (1,5,2), (3,2));
}


fn test_i_games(i: u16, x: u8, y: u8, cell_type_distribution: (u32,u32,u32), wall_type_distribution: (u32,u32)){
    for _ in 0..i{
        let newPit = Pit::random_rect(cell_type_distribution, wall_type_distribution, x, y);

        println!("{}", newPit);
        println!("Nimber: {}", newPit.get_generalized().calculate_nimber(&mut DataBase::none()));
        println!("---------------------------");
    }
}


