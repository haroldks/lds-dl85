mod mining;

mod data;

use mining::types_def::*;
use crate::data::dt::*;
use crate::mining::its_ops::ItemsetOps;



//
// fn from_item_to_attribute(item: usize, nattributes:usize){
//     let attribute =  if item%2 == 0{
//         nattributes /
//     };

// }


fn main() {

    let data = Data::new("datasets/pendigits.txt".to_string()).unwrap();

    let items: Vec<Item> = vec![(12, false)];

    let mut its_ops = ItemsetOps::new(items, &data, None, None, None, data.ntransactions, false);



    // println!("{:?}",its_ops.freq());
    println!("{:?}",its_ops.frequency());
    // println!("{:?}", its_ops.union_cover(&(11, true)));
    // println!("{:?}", its_ops.union_cover(&(19, true)));
     println!("{:?}", its_ops.classes_cover());
    // println!("{:?}", its_ops.current);



}
