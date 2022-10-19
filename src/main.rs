mod nim;
use nim::pit::Pit;

fn main() {
    println!("Hello World!");

    let mut counter = 0;

    let mut tries = 0;

    while counter < 5 {
        let pit = Pit::random_rect((0,1,0,), (1,1), 5, 4);

        tries += 1;



        match pit.get_generalized().find_symmetry()
        {
            None => (),
            Some(symmetry) => {
                println!("{}", pit);
                println!("{:?}", symmetry);
                counter += 1;
            },
        }
    };

    
    println!("Done! with {} tries", tries);
    
}


