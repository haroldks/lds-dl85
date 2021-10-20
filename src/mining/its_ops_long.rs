use crate::data::dt_longed::DataLong;
use crate::mining::itemset_bitvector_trait::ItemsetBitvector;
use crate::mining::types_def::Item;

pub struct ItemsetOpsLong<'a> {
    // TODO: Optimization for valids words using valids chuncks and limits variables
    // TODO: Look for options to changes the Vec to &[]. It can be faster
    pub current: Vec<Item>,
    pub data: &'a DataLong,
    support: Option<usize>,
    frequency: Option<f32>,
    mask: Option<Vec<u64>>,
    mask_stack: Vec<Vec<u64>>,
    ntransactions: usize,
    nchunks: usize,
    updated: bool,
}


impl ItemsetBitvector for ItemsetOpsLong<'_> {
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

    fn support(&mut self) -> usize {
        return if self.support.is_some() && self.updated {
            self.support.unwrap()
        } else if self.mask.is_some() && self.updated {
            let mask = self.mask.as_ref().unwrap();
            self.support = Option::from(ItemsetOpsLong::count_in_vec(mask));
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
            for j in 0..self.nchunks {
                let mut mask_chunk = &mut cloned_mask[j];
                let target_chunk = &self.data.target[i][j];
                *mask_chunk = *mask_chunk & *target_chunk;
            }
            classes_cover.push(ItemsetOpsLong::count_in_vec(&cloned_mask));
        }
        classes_cover
    }

    fn top_class(&mut self) -> (usize, usize) {
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

    fn leaf_misclassication_error(&mut self) -> (usize, usize) {
        let classes_cover = self.classes_cover();
        let max_class = self.top_class();
        let error = classes_cover.iter().sum::<usize>() - max_class.1;
        (error, max_class.0)
    }

    fn get_infos(&self) -> (usize, usize, usize) {
        (self.data.ntransactions, self.data.nattributes, self.data.nclasses)
    }

    fn get_current(&self) -> Vec<Item> {
        self.current.clone()
    }
}


impl<'a> ItemsetOpsLong<'a> {
    pub fn new(data: &DataLong) -> ItemsetOpsLong {
        let ntransactions = data.ntransactions;
        let nchunks = data.data[0].len();
        let mut mask = Option::from(vec![<u64>::MAX; nchunks]);
        let dead_bits = 64 - match ntransactions % 64 {
            0 => { 0 }
            _ => { nchunks * 64 - ntransactions }
        };

        let mut first_chunk = &mut mask.as_mut().unwrap()[0];
        for i in (dead_bits..64).rev() {
            let int_mask = 1u64 << i;
            *first_chunk = *first_chunk & !int_mask;
        }
        let cloned_mask = mask.as_ref().unwrap().clone();

        ItemsetOpsLong { current: vec![], data, support: None, frequency: None, mask, mask_stack: vec![cloned_mask], ntransactions, updated: false, nchunks }
    }

    fn update_mask(&mut self, item: &Item) {
        let mask = self.mask.as_mut().unwrap();
        let mut item_vec = self.data.data[item.0].clone();

        for i in 0..self.nchunks {
            let mut a = &mut mask[i];
            let b = &mut item_vec[i];
            if !item.1 {
                *a = *a & !*b;
            } else {
                *a = *a & *b;
            }
        }

        self.updated = false;
    }

    fn gen_new_mask(&mut self) {
        self.mask = Option::from(vec![<u64>::MAX; self.nchunks]);
        let dead_bits = 64 - match self.ntransactions % 64 {
            0 => { 0 }
            _ => { self.nchunks * 64 - self.ntransactions }
        };

        let mut first_chunk = &mut self.mask.as_mut().unwrap()[0];
        for i in (dead_bits..64).rev() {
            let int_mask = 1u64 << i;
            *first_chunk = *first_chunk & !int_mask;
        }
    }

    fn compute_support_from_mask(&mut self) -> usize {
        if self.mask.is_none() {
            self.gen_new_mask();
        }

        for item in &self.current.clone() {
            self.update_mask(item);
        }
        let mask = self.mask.as_mut().unwrap();
        self.support = Option::from(ItemsetOpsLong::count_in_vec(mask));
        self.frequency = Option::from(self.support.unwrap() as f32 / self.ntransactions as f32);
        self.updated = true;
        self.support.unwrap()
    }


    fn count_in_vec(arr: &Vec<u64>) -> usize {
        arr.iter().map(|bv| bv.count_ones() as usize).collect::<Vec<usize>>().iter().sum()
    }

    pub fn frequency(&mut self) -> f32 {
        if !self.updated {
            self.support();
        }
        self.frequency.unwrap()
    }
}
