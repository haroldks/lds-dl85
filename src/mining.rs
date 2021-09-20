use bit_vec::BitVec;
use super::data::Data;

pub type Attribute = usize;
pub type Item = (Attribute, bool);
//pub type Itemset = Vec<Item>; // I can use a structure to have all data stored in it quickly like support, freq and so one when each function is called


pub struct ItemsetOps<'a>{

    pub current: Vec<Item>,
    data: &'a Data,
    support: Option<usize>,
    frequency: Option<f32>,
    mask: Option<BitVec>,
    ntransactions: usize,
    updated: bool

}

impl <'a> ItemsetOps <'a> {

    pub fn new(current: Vec<Item>, data: &Data, support: Option<usize>, frequency: Option<f32>, mask: Option<BitVec>, ntransactions:usize, updated:bool) -> ItemsetOps {
        ItemsetOps{current, data,  support, frequency, mask, ntransactions, updated}
    }

    pub fn union(&mut self, second_its: &Item){
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

    fn update_mask(&mut self, item: &Item){
          let mut  mask = self.mask.as_mut().unwrap();
            let mut item_vec = self.data.data[item.0].clone();
            if !item.1 {
                item_vec.negate();
            }
            mask.and(&item_vec);
            self.updated = false;
    }

    fn  compute_support_from_mask(&mut self) -> usize {
        if !self.mask.is_some(){
            self.mask = Option::from(BitVec::from_elem(self.ntransactions, true));
        }
        for mut item in &self.current.clone(){
           self.update_mask(&item);
        }
        let mut mask = self.mask.as_mut().unwrap();
        self.support = Option::from(mask.iter().filter(|x| *x).count());
        self.frequency = Option::from(self.support.unwrap() as f32 / self.ntransactions as f32);
        self.updated = true;
        self.support.unwrap()
    }


    pub fn support(&mut self) -> usize {
        return if self.support.is_some() && self.updated{
            self.support.unwrap()
        } else if self.mask.is_some() {
            let mask = self.mask.as_ref().unwrap();
            self.support = Option::from(mask.iter().filter(|x| *x).count());
            self.frequency = Option::from(self.support.unwrap() as f32 / self.ntransactions as f32);
            self.updated = true;
            self.support.unwrap()
        } else {
            self.mask = Option::from(BitVec::from_elem(self.ntransactions, true));
            self.compute_support_from_mask()
        }

    }


    pub fn freq(&mut self) -> f32 {
        if !self.updated{
            self.support();
        }
        self.frequency.unwrap()
    }

}


