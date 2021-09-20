mod mining;
mod data;

use crate::data::*;
use crate::mining::*;
use bit_vec::BitVec;
use crate::mining::ItemsetOps;
use std::thread::current;


fn main() {

    let mut data = Data::new("datasets/pendigits.txt".to_string()).unwrap();

    let items: Vec<Item> = vec![(12, true), (1, true)];

    let mut its_ops = ItemsetOps::new(items, &data, None, None, None, data.ntransactions, false);



    // println!("{:?}",its_ops.freq());
    println!("{:?}",its_ops.freq());
    println!("{:?}", its_ops.union_cover(&(11, true)));
    println!("{:?}", its_ops.union_cover(&(19, true)));
     println!("{:?}", data.data[19]);
    // println!("{:?}", its_ops.current);



}
