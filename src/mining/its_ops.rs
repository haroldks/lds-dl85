use bit_vec::BitVec;

use crate::data::dt::Data;
use crate::mining::itemset_bitvector_trait::ItemsetBitvector;
use crate::mining::types_def::*;

pub struct ItemsetOps<'a> {
    pub current: Vec<Item>,
    data: &'a Data,
    support: Option<usize>,
    frequency: Option<f32>,
    mask: Option<BitVec>,
    mask_stack: Vec<BitVec>,
    ntransactions: usize,
    updated: bool,
}

impl ItemsetBitvector for ItemsetOps<'_> {
    fn intersection_cover(&mut self, second_its: &Item) -> usize {
        self.current.push(*second_its);
        self.updated = false;
        self.support = None;
        self.update_mask(second_its);
        self.mask_stack.push(self.mask.as_ref().unwrap().clone());
        self.support()
    }

    fn temp_intersection(&mut self, second_its: &Item) -> usize {
        self.current.push(*second_its);
        self.updated = false;
        self.support = None;
        self.update_mask(second_its);
        self.mask_stack.push(self.mask.as_ref().unwrap().clone());
        let support = self.support();
        self.backtrack();
        support
    }

    fn backtrack(&mut self) {
        self.mask_stack.pop();
        self.current.pop();
        self.mask = Option::from(self.mask_stack[self.mask_stack.len() - 1].clone());
        self.updated = false;
        self.support = None;
        self.support();
    }

    fn reset(&mut self) {
        self.gen_new_mask();
        let cloned_mask = self.mask.as_ref().unwrap().clone();
        self.mask_stack = vec![cloned_mask];
        self.support = None;
        self.frequency = None;
        self.updated = false;
        self.current = vec![];
    }

    fn support(&mut self) -> usize {
        return if self.support.is_some() && self.updated {
            self.support.unwrap()
        } else if self.mask.is_some() && self.updated {
            let mask = self.mask.as_ref().unwrap();
            self.support = Option::from(ItemsetOps::count_in_vec(mask));
            self.frequency = Option::from(self.support.unwrap() as f32 / self.ntransactions as f32);
            self.updated = true;
            self.support.unwrap()
        } else {
            self.gen_new_mask();
            self.compute_support_from_mask()
        };
    }

    fn classes_cover(&mut self) -> Vec<usize> {
        let mut classes_cover = vec![];
        for i in 0..self.data.nclasses {
            let mut cloned_mask = self.mask.clone().unwrap();
            let target_chunk = &self.data.target[i];
            cloned_mask.and(target_chunk);
            classes_cover.push(ItemsetOps::count_in_vec(&cloned_mask));
        }
        classes_cover
    }

    fn top_class(&mut self) -> (usize, usize) {
        let classes_cover = self.classes_cover();
        let (max_idx, max_val) = classes_cover.iter().enumerate().fold(
            (0, classes_cover[0]),
            |(idxm, valm), (idx, val)| {
                if val > &valm {
                    (idx, *val)
                } else {
                    (idxm, valm)
                }
            },
        );
        (max_idx, max_val)
    }

    fn leaf_misclassication_error(&mut self) -> (usize, usize) {
        let classes_cover = self.classes_cover();
        let max_class = self.top_class();
        let error = classes_cover.iter().sum::<usize>() - max_class.1;
        (error, max_class.0)
    }

    fn get_infos(&self) -> (usize, usize, usize) {
        (
            self.data.ntransactions,
            self.data.nattributes,
            self.data.nclasses,
        )
    }

    fn get_current(&self) -> Vec<Item> {
        self.current.clone()
    }

    fn get_nclasses(&self) -> usize {
        self.data.nclasses
    }

    fn temp_classes_cover(&mut self, second_its: &Item) -> Vec<usize> {
        self.current.push(*second_its);
        self.updated = false;
        self.support = None;
        self.update_mask(second_its);
        self.mask_stack.push(self.mask.as_ref().unwrap().clone());
        let cover = self.classes_cover();
        self.backtrack();
        cover
    }
}

#[allow(dead_code)]
impl<'a> ItemsetOps<'a> {
    pub fn new(data: &Data) -> ItemsetOps {
        let ntransactions = data.ntransactions;
        let mask = Option::from(BitVec::from_elem(ntransactions, true));
        let cloned_mask = mask.as_ref().unwrap().clone();
        ItemsetOps {
            current: vec![],
            data,
            support: None,
            frequency: None,
            mask,
            mask_stack: vec![cloned_mask],
            ntransactions,
            updated: false,
        }
    }

    fn update_mask(&mut self, item: &Item) {
        let mask = self.mask.as_mut().unwrap();
        let mut item_vec = self.data.data[item.0].clone();
        if !item.1 {
            item_vec.negate();
        }
        mask.and(&item_vec);
        self.updated = false;
    }

    fn gen_new_mask(&mut self) {
        self.mask = Option::from(BitVec::from_elem(self.ntransactions, true));
    }

    fn compute_support_from_mask(&mut self) -> usize {
        if self.mask.is_none() {
            self.gen_new_mask();
        }
        for item in &self.current.clone() {
            self.update_mask(item);
        }
        let mask = self.mask.as_mut().unwrap();
        self.support = Option::from(mask.iter().filter(|x| *x).count());
        self.frequency = Option::from(self.support.unwrap() as f32 / self.ntransactions as f32);
        self.updated = true;
        self.support.unwrap()
    }

    fn count_in_vec(arr: &BitVec) -> usize {
        arr.iter().filter(|a| *a).count()
    }

    pub fn frequency(&mut self) -> f32 {
        if !self.updated {
            self.support();
        }
        self.frequency.unwrap()
    }
}
