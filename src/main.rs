//#![allow(dead_code)]

mod nim;
mod vec_ops;
use std::{time::Instant, fs::File, io::Write};
use nim::{pit::Pit, generalized::constructor};
use nim::evaluator::Evaluator;

use crate::nim::evaluator_db::DataBase;
use crate::nim::generalized::GeneralizedNimGame;
use crate::nim::pit_reconstructor::try_reconstruct;

fn main() {
    time_both_eval(&constructor::rect(2, 8));
}

fn time_both_eval(g: &GeneralizedNimGame){
    println!("{}",g);
    println!("old:");
    time_old_eval(g);
    println!("new:");
    time_new_eval(g);
}

fn time_new_eval(g: &GeneralizedNimGame){
    let starting_test = Instant::now();
    let mut eval = Evaluator::new();
    println!("{}", eval.calc_nimber(&g));
    println!("{:?}", starting_test.elapsed());
}
fn time_old_eval(g: &GeneralizedNimGame){
    let starting_test = Instant::now();
    let mut db = DataBase::new();
    println!("{}", db.get_nimber(g));

    println!("{:?}", starting_test.elapsed());
}

fn print_triangle_nimbers(max :u16) {
    let mut db = DataBase::new();
    for i in 0..max{
        let g = nim::generalized::constructor::triangle(i);
        println!("{}:{}", i,db.get_nimber(&g));
    }
}
fn make_and_write_data_base(p:Pit){

    let mut db = DataBase::new();

    db.get_nimber(&p.get_generalized());

    let latex_repr = &*db.get_latex_repr();
    let mut f = File::create("db.tex").expect("Unable to create file");
    f.write_all(latex_repr.as_bytes()).expect("Unable to write data");

}
fn test_reconstruction(pit: Pit){
    println!("{}", pit);
    let g = pit.get_generalized();
    if g.get_parts().len() == 0 {return;}
    let c = &g.get_parts()[0];
    println!("{}", c);
    let reconstructed = try_reconstruct(c).unwrap();
    println!("{}", reconstructed);
}

fn get_latex_data_base(pit: Pit) -> String{
    let mut db = DataBase::new();
    db.get_nimber(&pit.get_generalized());
    
    println!("{}", db);

    return db.get_latex_repr();

}
fn test_pit_game(pit :Pit){
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