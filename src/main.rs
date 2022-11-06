//#![allow(dead_code)]

mod nim;
use std::{time::Instant, fs::File, io::Write};
use nim::{pit::{Pit}, pit_reconstructor, generalized::constructor};

use crate::nim::{generalized::{data_base::DataBase}, pit_reconstructor::try_reconstruct};

fn main() {
    //println!("{}", get_latex_data_base(Pit::empty_rect(3,3)));
    let h_cube = constructor::hyper_cube(3, 3);
    println!("{}", h_cube);
    println!("nimber: {}", h_cube.get_nimber(&mut DataBase::new()));
}

fn print_triangle_nimbers(max :u16) {
    let mut db = DataBase::new();
    for i in 0..max{
        let g = nim::generalized::constructor::triangle(i);
        println!("{}:{}", i,g.get_nimber(&mut db));
    }
}
fn make_and_write_data_base(p:Pit){

    let mut db = DataBase::new();

    p.get_generalized().get_nimber(&mut db);

    let data = &*db.get_latex_repr();
    let mut f = File::create("db.tex").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");

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
    let mut data_base = DataBase::new();
    pit.get_generalized().get_nimber(&mut data_base);
    
    println!("{}", data_base);

    return data_base.get_latex_repr();

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