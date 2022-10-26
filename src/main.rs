#![allow(dead_code)]

mod nim;
use std::time::Instant;

use crate::nim::generalized::{data_base::DataBase, GeneralizedNimGame};
use nim::pit::Pit;

fn main() {
    run_basic_test();
}

fn run_basic_test() {
    let now = Instant::now();
    let data_base = &mut DataBase::new();
    println!(
        "{}",
        Pit::empty_rect(3, 4)
            .get_generalized()
            .get_nimber(data_base)
    );
    println!("{:?}", now.elapsed());
    println!("{}", data_base.len())
}


