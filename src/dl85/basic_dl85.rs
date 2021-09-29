use crate::cache::trie::Trie;
use crate::data::dt_chuncked::DataChuncked;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::mining::types_def::{Attribute, Item};
use std::intrinsics::fadd_fast;
use crate::node::node::Node;

pub struct DL85<'a> {
    ntransactions: usize,
    nclasses: usize,
    nattributes: usize,
    min_support: u64,
    max_depth: u64,
    max_error: f64,
    time_limit: f64,
    cache: Trie,
    its_op: ItemsetOpsChunked<'a>,

}

impl <'a>DL85<'a> {
    pub fn new(min_support: u64, max_depth: u64,  max_error: f64, time_limit: f64, cache: Trie, its_op: ItemsetOpsChunked, ) -> DL85 {

        DL85 {
            ntransactions: its_op.data.ntransactions,
            nclasses: its_op.data.nclasses,
            nattributes: its_op.data.nattributes,
            min_support,
            max_depth,
            max_error,
            time_limit,
            cache,
            its_op,

        }
    }




    pub fn run(&mut self){

        let mut candidates_list: Vec<Attribute> = Vec::new();
        if self.min_support == 1{
            candidates_list = (0..self.nattributes).collect::<Vec<Attribute>>(); // TODO: Information Gain ???
        }
        else {
            for i in 0..self.nattributes {
                if self.its_op.temp_union(&(i, false)) >= self.min_support as usize && self.its_op.temp_union(&(i, true)) >= self.min_support as usize{
                    candidates_list.push(i);
                }
            }
        }

        println!("{:?}", candidates_list);

        let empty_itemset: Vec<Item> = vec![];
        self.recursive_dl85(empty_itemset, usize::MAX, candidates_list, self.max_error, 0);




    }


    fn recursive_dl85(&mut self, current_itemset: Vec<Item>, last_attribute: Attribute, next_candidates: Vec<Attribute>, upper_bound: f64, depth: u64){

        let cached_node = self.cache.get(&current_itemset).unwrap();
        if cached_node.is_new {
            println!("This is a new node");
        }
        new_candidates = self.get_next_sucessors(&next_candidates, last_attribute);
        for attribute in next_candidates{

            let items: Vec<Item> = vec![(attribute, false),(attribute, true)];
            let first_item_sup = self.its_op.union_cover(&items[0]);
            let first_node = self.cache.insert(&self.its_op.current);
            first_node.data = Node {
                current_depth: depth,
                test: items[0],
                leaf_error: 0.0,
                node_error: 0.0
            }

        }




    }

    fn get_next_sucessors(&mut self, candidates: &Vec<Attribute>, last_attribute: Attribute) -> Vec<Attribute> {
        let mut next_candidates = vec![];
        let current_support = self.its_op.support();

        for candidate in candidates{
            let left_sup = self.its_op.temp_union(&(*candidate, false));
            let right_sup = current_support - left_sup;
            if left_sup >= self.min_support as usize && right_sup >= self.min_support as usize{
                next_candidates.push(*candidate)
            }
        }
        next_candidates
    }



    fn get_candidates_support(&mut self, candidates: &Vec<usize>) -> Vec<(usize, usize)> {
        let mut all_supports = vec![];
        for candidate in candidates{
            println!("{:?}", candidate);
            let items = vec![(*candidate, true), (*candidate, false)];
            let mut c_supports = vec![];
            for it in items{
                c_supports.push(self.its_op.union_cover(&it));
                self.its_op.backtrack();
            }
            all_supports.push((c_supports[0], c_supports[1]));

        }
        all_supports
    }
}
