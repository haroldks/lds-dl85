mod mining;

mod data;
mod cache;
mod node;

use mining::types_def::*;
use crate::data::dt::*;
use crate::data::dt_chuncked::*;
use crate::data::dt_longed::*;
use crate::mining::its_ops::ItemsetOps;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::cache::trie::*;
use bit_vec::BitVec;

use std::time::Instant;

//
// fn from_item_to_attribute(item: usize, nattributes:usize){
//     let attribute =  if item%2 == 0{
//         nattributes /
//     };

// }


fn main() {




    // let data = Data::new("datasets/pendigits.txt".to_string()).unwrap();
    // let items: Vec<Item> = vec![(11, false)];
    // let mut its_ops = ItemsetOps::new(items, &data, None, None, None, data.ntransactions, false);
    // let start = Instant::now();
    // println!("{:?}",its_ops.support());
    // println!("{:?}", its_ops.union_cover(&(12, true)));
    // let duration = start.elapsed();
    // println!("{:?}", duration);


    // let datac = DataChuncked::new("datasets/pendigits.txt".to_string()).unwrap();
    // let itemsd: Vec<Item> = vec![(11, false)];
    // let mut its_opsd = ItemsetOpsChunked::new(itemsd, &datac, None, None, None, datac.ntransactions, false,  datac.data[0].len());
    // let start = Instant::now();
    // println!("{:?}",its_opsd.support());
    // println!("{:?}", its_opsd.classes_cover());
    // let duration = start.elapsed();
    // println!("{:?}", duration);


     //println!("{:?}",datac.data[0].len());






    // println!("{:?}", its_ops.union_cover(&(19, true)));
     //println!("{:?}", its_opsd.union_cover(&(19, true)));
    //println!("{:?}", its_ops.classes_cover());
    //println!("{:?}", its_ops.top_class());
   // println!("{:?}", its_opsd.classes_cover());
    //println!("{:?}", its_ops.current);
    // let data = DataLong::new("datasets/pendigits.txt".to_string()).unwrap();
    // println!("{:?}", data.data[0]);

     let a = "1000";
    // println!("{:b}, {:?}, {:?}",2, (1 as u64).leading_zeros(), 2 as u64 & 1 as u64);
    // println!("{:?}", <u64>::from_str_radix(a, 2).unwrap());
    // println!("{:?}", "1111101111111111111111111101110111110111111101101111111110111111".chars().count());

    let mut cache = Trie::new();
    let mut its: Vec<Item> = vec![(2, false), (1, true), (0, true)];
    its.sort();
    println!("{:?}", its);
    let lol = cache.insert(&its);
    let mut its: Vec<Item> = vec![(1, true), (0, true), (4, false)];
    its.sort();
    let lol = cache.insert(&its);
    lol.edges.push(TrieEdge::new((3, false)));
    let mdr = cache.get(&its).unwrap();

    println!("{:?}", cache.root)



}

