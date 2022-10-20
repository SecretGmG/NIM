mod nim;
use nim::{pit::Pit};

fn main() {

    let g = Pit::empty_rect(3, 2).get_generalized();

    print!("{}", g);


    print!("{}", g.calculate_nimber());
    
    
}


