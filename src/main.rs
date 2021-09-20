use crate::data::*;

mod mining;
mod data;


use bit_vec::BitVec;



fn main() {

    let mut data = Data::new("datasets/pendigits.txt".to_string()).unwrap();

    println!("{:?}", data.nclasses);



}
