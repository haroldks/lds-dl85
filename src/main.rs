mod mining;

mod data;

use mining::types_def::*;
use crate::data::dt::*;
use crate::data::dt_chuncked::*;
use crate::mining::its_ops::ItemsetOps;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use bit_vec::BitVec;

use std::time::Instant;
//
// fn from_item_to_attribute(item: usize, nattributes:usize){
//     let attribute =  if item%2 == 0{
//         nattributes /
//     };

// }


fn main() {




    let data = Data::new("datasets/pendigits.txt".to_string()).unwrap();
    let items: Vec<Item> = vec![(11, false)];
    let mut its_ops = ItemsetOps::new(items, &data, None, None, None, data.ntransactions, false);
    let start = Instant::now();
    println!("{:?}",its_ops.support());
    println!("{:?}", its_ops.union_cover(&(12, true)));
    let duration = start.elapsed();
    println!("{:?}", duration);


    let datac = DataChuncked::new("datasets/pendigits.txt".to_string()).unwrap();
    let itemsd: Vec<Item> = vec![(11, false)];
    let mut its_opsd = ItemsetOpsChunked::new(itemsd, &datac, None, None, None, datac.ntransactions, false,  datac.data[0].len());
    let start = Instant::now();
    println!("{:?}",its_opsd.support());
    println!("{:?}", its_opsd.union_cover(&(12, true)));
    let duration = start.elapsed();
    println!("{:?}", duration);


     //println!("{:?}",datac.data[0].len());






    // println!("{:?}", its_ops.union_cover(&(19, true)));
     //println!("{:?}", its_opsd.union_cover(&(19, true)));
    //println!("{:?}", its_ops.classes_cover());
    //println!("{:?}", its_ops.top_class());
   // println!("{:?}", its_opsd.classes_cover());
    //println!("{:?}", its_ops.current);



}
