use bit_vec::BitVec;

use crate::data::dt_chuncked::DataChuncked;
use crate::mining::types_def::*;

pub struct ItemsetOpsChunked<'a> {
    pub current: Vec<Item>,
    data: &'a DataChuncked,
    support: Option<usize>,
    frequency: Option<f32>,
    mask: Option<Vec<BitVec>>,
    ntransactions: usize,
    nchunks: usize,
    updated: bool,
}


#[allow(dead_code)]
impl<'a> ItemsetOpsChunked<'a> {
    pub fn new(current: Vec<Item>, data: &DataChuncked, support: Option<usize>, frequency: Option<f32>, mask: Option<Vec<BitVec>>, ntransactions: usize, updated: bool, nchunks: usize) -> ItemsetOpsChunked {
        ItemsetOpsChunked { current, data, support, frequency, mask, ntransactions, updated, nchunks }
    }

    pub fn union(&mut self, second_its: &Item) {
        self.current.push(*second_its);
        self.updated = false;
        self.update_mask(&second_its);
        self.support();
    }

    pub fn union_cover(&mut self, second_its: &Item) -> usize {
        self.current.push(*second_its);
        self.updated = false;
        self.update_mask(&second_its);
        self.support()
    }

    fn update_mask(&mut self, item: &Item) {
        let mask = self.mask.as_mut().unwrap();
        let mut item_vec = self.data.data[item.0].clone();

        for i in 0..self.nchunks {
            let mut a = &mut mask[i];
            let mut b = &mut item_vec[i];
            if !item.1 {
                b.negate();
                a.and(b);
            } else {
                a.and(b);
            }
        }

        self.updated = false;
    }

    fn gen_new_mask(&mut self) {
        self.mask = Option::from(vec![BitVec::from_elem(64, true); self.nchunks]);
        let dead_bits = 64 - match self.ntransactions % 64 {
            0 => { 0 }
            _ => { self.nchunks * 64 - self.ntransactions }
        };
        let last_chunk = &mut self.mask.as_mut().unwrap()[self.nchunks - 1];
        for i in (dead_bits..64).rev() {
            last_chunk.set(i, false);
        }
    }

    fn compute_support_from_mask(&mut self) -> usize {
        if !self.mask.is_some() {
            self.gen_new_mask();
        }

        for item in &self.current.clone() {
            self.update_mask(&item);
        }
        let mask = self.mask.as_mut().unwrap();
        self.support = Option::from(ItemsetOpsChunked::count_in_vec(&mask));
        self.frequency = Option::from(self.support.unwrap() as f32 / self.ntransactions as f32);
        self.updated = true;
        self.support.unwrap()
    }


    pub fn support(&mut self) -> usize {
        return if self.support.is_some() && self.updated {
            self.support.unwrap()
        } else if self.mask.is_some() && self.updated {
            let mask = self.mask.as_ref().unwrap();
            self.support = Option::from(ItemsetOpsChunked::count_in_vec(&mask));
            self.frequency = Option::from(self.support.unwrap() as f32 / self.ntransactions as f32);
            self.updated = true;
            self.support.unwrap()
        } else {
            self.gen_new_mask();
            self.compute_support_from_mask()
        };
    }

    fn count_in_vec(arr: &Vec<BitVec>) -> usize {
        arr.iter().map(|bv| bv.iter().filter(|x| *x).count()).collect::<Vec<usize>>().iter().sum()
    }

    pub fn frequency(&mut self) -> f32 {
        if !self.updated {
            self.support();
        }
        self.frequency.unwrap()
    }

    pub fn classes_cover(&mut self) -> Vec<usize> {
        let mut classes_cover = vec![];
        for i in 0..self.data.nclasses {
            let mut cloned_mask = self.mask.clone().unwrap();
            for j in 0..self.nchunks {
                let mask_chunk = &mut cloned_mask[j];
                let target_chunk = &self.data.target[i][j];
                mask_chunk.and(target_chunk);
            }
            classes_cover.push(ItemsetOpsChunked::count_in_vec(&cloned_mask));
        }
        classes_cover
    }


    pub fn top_class(&mut self) -> (usize, usize) {
        let classes_cover = self.classes_cover();
        let (max_idx, max_val) =
            classes_cover.iter().enumerate().
                fold((0, classes_cover[0]), |(idxm, valm), (idx, val)|
                    if val > &valm {
                        (idx, *val)
                    } else {
                        (idxm, valm)
                    },
                );
        (max_idx, max_val)
    }
}
