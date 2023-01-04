//#![allow(dead_code)]

mod nim;
mod vec_ops;
use crate::nim::evaluator::impartial::{Impartial, ImpartialPart};
use crate::nim::evaluator_db::DataBase;
use crate::nim::generalized::GeneralizedNimGame;
use nim::evaluator::Evaluator;
use nim::generalized::closed_generalized::ClosedGeneralizedNimGame;
use nim::{generalized::constructor::*, pit::Pit};
use std::{fs::File, io::Write, time::Instant};

fn main() {
    let mut generic_eval: Evaluator<ClosedGeneralizedNimGame, GeneralizedNimGame> =
        Evaluator::new();
    let mut old_eval = DataBase::new();
    for i in 0..6 {
        let test = Pit::empty_rect(i, i).get_generalized();

        time_generic(&test, &mut generic_eval);
        time_old_eval(&test, &mut old_eval);
    }
}

fn time_generic<Part, Whole>(g: &Whole, eval: &mut Evaluator<Part, Whole>)
where
    Part: ImpartialPart<Part, Whole>,
    Whole: Impartial<Part, Whole>,
{
    println!("Generic Evaluator, no bounds");
    let starting_test = Instant::now();
    println!("{}", eval.get_nimber_of_whole(&g));
    println!("{:?}", starting_test.elapsed());
}

fn time_old_eval(g: &GeneralizedNimGame, db: &mut DataBase) {
    println!("Old Evaluator");
    let starting_test = Instant::now();
    println!("{}", db.get_nimber(g));
    println!("{:?}", starting_test.elapsed());
}
#[allow(dead_code)]
fn print_triangle_nimbers(max: u16) {
    let mut db = DataBase::new();
    for i in 0..max {
        let g = Constructor::triangle(i).build();
        println!("{}:{}", i, db.get_nimber(&g));
    }
}
#[allow(dead_code)]
fn make_and_write_data_base(p: Pit) {
    let mut db = DataBase::new();

    db.get_nimber(&p.get_generalized());

    let latex_repr = &*db.get_latex_repr();
    let mut f = File::create("db.tex").expect("Unable to create file");
    f.write_all(latex_repr.as_bytes())
        .expect("Unable to write data");
}
#[allow(dead_code)]
fn test_reconstruction(pit: Pit) {
    println!("{}", pit);
    let g = pit.get_generalized();
    if g.get_parts().len() == 0 {
        return;
    }
    let c = &g.get_parts()[0];
    println!("{}", c);
    let reconstructed = Pit::try_reconstruct(c).unwrap();
    println!("{}", reconstructed);
}
#[allow(dead_code)]
fn get_latex_data_base(pit: Pit) -> String {
    let mut db = DataBase::new();
    db.get_nimber(&pit.get_generalized());

    println!("{}", db);

    return db.get_latex_repr();
}
#[allow(dead_code)]
fn test_pit_game(pit: Pit) {
    let starting_test = Instant::now();

    let db = &mut DataBase::new();
    println!("{}", pit);

    let g = pit.get_generalized();

    println!("{}", g);

    let nimber = db.get_nimber(&g);

    println!("nimber: {}", nimber);

    println!("{:?}", starting_test.elapsed());
    println!("{}", db.len());

    //println!("{}", data_base);
}
