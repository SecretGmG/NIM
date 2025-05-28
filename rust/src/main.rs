mod pit;
mod taking_game;
use evaluator::*;
use taking_game::constructor::Constructor;
use pit::Pit;
use std::time::Instant;

fn main() {
    //print_triangle_nimbers(10);
    let mut eval = Evaluator::new();
    let g = Constructor::hyper_cuboid(vec![4, 5]).build();
    println!("{}", eval.get_nimber(g));

    let g = Constructor::hyper_cuboid(vec![4, 5]).build();
    println!("{}", eval.get_nimber(g));
}
#[cfg(test)]
mod test {
    use evaluator::Evaluator;
    use crate::pit::Pit;


    #[test]
    fn sqr2x5() {
        let mut eval = Evaluator::new();
        let g = Pit::empty_rect(4, 5).get_generalized();
        println!("{}", eval.get_nimber(g));
        let g = Pit::empty_rect(5, 4).get_generalized();
        println!("{}", eval.get_nimber(g));
        let g = Pit::empty_rect(4, 3).get_generalized();
        println!("{}", eval.get_nimber(g));
        let g = Pit::empty_rect(5, 5).get_generalized();
        println!("{}", eval.get_nimber(g));
    }
}

#[allow(dead_code)]
fn print_kayle_nimbers(max: usize) {
    let mut eval = Evaluator::new();
    for i in 0..max {
        let g = Constructor::kayles(i).build();
        println!("{}:{}", i, eval.get_nimber(g));
    }
}

#[allow(dead_code)]
fn print_triangle_nimbers(max: usize) {
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
    let reconstructed = Pit::reconstruct(c).unwrap();
    println!("{}", reconstructed);
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
