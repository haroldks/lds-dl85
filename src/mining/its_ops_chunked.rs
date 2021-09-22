use crate::mining::types_def::*;
use crate::data::dt::Data;
use bit_vec::BitVec;


pub struct ItemsetOpsChunked<'a>{

    pub current: Vec<Item>,
    data: &'a Data,
    support: Option<usize>,
    frequency: Option<f32>,
    mask: Option<Vec<BitVec>>,
    ntransactions: usize,
    updated: bool

}
