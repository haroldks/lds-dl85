use std::cmp::max;
use crate::cache::trie::{Trie, TrieEdges, TrieNode};
use crate::data::dt_chuncked::DataChuncked;
use crate::mining::its_ops_chunked::ItemsetOpsChunked;
use crate::mining::types_def::{Attribute, Item};
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

impl<'a> DL85<'a> {
    pub fn new(min_support: u64, max_depth: u64, max_error: f64, time_limit: f64, cache: Trie, its_op: ItemsetOpsChunked) -> DL85 {
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




    fn recursion(mut cache: Trie, mut its_op: ItemsetOpsChunked<'a>, current_itemset: Vec<Item>, last_attribute: Attribute, next_candidates: Vec<Attribute>, mut upper_bound: f64, depth: u64, max_depth: u64, min_support: u64, max_error: f64, mut parent_node_data: Node) -> (Trie, ItemsetOpsChunked<'a>, Node) {



       // let mut cached_node = cache.get(&current_itemset);
        let mut child_upper_bound = upper_bound;
        let mut min_lb = <f64>::MAX;

        // Check if Cached node is a final node in the recursion

        //TODO: check node_data_info
        let current_support = its_op.support() as u64;
        println!("Depth: {:?}, max: {:?}", depth, max_depth);
        println!("Current: {:?}", current_itemset);
        println!("Parent : {:?}", parent_node_data);
        let data = DL85::check_if_stop_condition_reached (parent_node_data, upper_bound, min_support, current_support, depth, max_depth);

        if data.0{
            cache.update(&current_itemset, data.1);
            return (cache, its_op, data.1);
        }





        // if cached_node.is_some() {
        //     println!("The node exists in the cache.");
        //     let mut node_ref = cached_node.as_mut().unwrap();
        //     let current_support = its_op.support() as u64;
        //     DL85::check_if_stop_condition_reached(&mut node_ref, upper_bound, min_support, current_support, depth, max_depth);
        //     //*node_ref
        // }

        //let mut cached_node = cached_node.as_mut().unwrap();

        let new_candidates = DL85::get_next_sucessors(&next_candidates, last_attribute, &mut its_op, min_support);

        if new_candidates.len() == 0 {
            // cached_node.data.node_error = cached_node.data.leaf_error;
            parent_node_data.node_error = parent_node_data.leaf_error;
            parent_node_data.is_new = false;
            println!("No candidates: next_c : {:?}, last {:?}", new_candidates, last_attribute);
            println!("No candidates: {:?}, {:?}", current_itemset, parent_node_data);
            cache.update(&current_itemset, parent_node_data);
            // cached_node.is_new = false;
            // println!("There are not candidates. Node error becomes leaf error. upper_bound = {}, depth = {}", upper_bound, depth);
            return (cache, its_op, parent_node_data);
        }

        for attribute in &new_candidates {
            println!("We came here. {:?}", attribute);
            // let first_lb = -1f64;
            // let second_lb = -1f64;

            let items: Vec<Item> = vec![(*attribute, false), (*attribute, true)];
            let first_item_sup = its_op.union_cover(&items[0]); // Here current is supposed to be updated


            let mut child_item_set = current_itemset.clone();
            child_item_set.push(items[0]);
            child_item_set.sort();


            let mut first_node_data = DL85::retrieve_cache_emplacement_for_current_its(&mut cache, &items[0], depth, &mut its_op); // Error computation // cache_ref, item_ref, depth

            // first_node_data.lower_bound = match first_node_data.is_new {
            //     true => {
            //         if first_lb > first_node_data.lower_bound {
            //             first_node_data.lower_bound
            //         } else {
            //             first_lb
            //         }
            //     },
            //     _ => { first_lb }
            // };
            println!("Before first split Upper Bound : {:?}", child_upper_bound );
            let data = DL85::recursion(cache, its_op, child_item_set.clone(), *attribute, new_candidates.clone(), child_upper_bound, depth + 1, max_depth, min_support, max_error, first_node_data);
            // *first_node = data.0;
            cache = data.0;
            its_op = data.1;
            first_node_data = data.2;
            //let first_node =  cache.get(&child_item_set).unwrap();
            cache.update(&child_item_set, first_node_data);
            let first_split_error = first_node_data.node_error;
            its_op.backtrack();
            println!("Just after first node computation. Error: {:?}", first_split_error);
            if first_node_data.node_error < upper_bound{
                println!("Second node computation.");
                let second_item_sup = its_op.union_cover(&items[1]);

                let mut child_item_set = current_itemset.clone();
                child_item_set.push(items[1]);
                let mut second_node_data = DL85::retrieve_cache_emplacement_for_current_its(&mut cache, &items[1], depth, &mut its_op); // Error computation // cache_ref, item_ref, depth


                //
                // second_node.data.lower_bound = match second_node.is_new {
                //     true => {
                //         if second_lb > second_node.data.lower_bound {
                //             second_node.data.lower_bound
                //         } else {
                //             second_lb
                //         }
                //     },
                //     _ => { second_lb }
                // };

                let remaining_ub = child_upper_bound - first_split_error;
                child_item_set.sort();
                let data = DL85::recursion(cache, its_op, child_item_set.clone(), *attribute, new_candidates.clone(), remaining_ub, depth + 1, max_depth, min_support, max_error, second_node_data);
                // *second_node = data.0;
                cache = data.0;
                its_op = data.1;
                second_node_data = data.2;
                // second_node = cache.get(&child_item_set).unwrap();
                cache.update(&child_item_set, second_node_data);
                let second_split_error = second_node_data.node_error;
                its_op.backtrack();


                let feature_error = first_split_error + second_split_error;
                println!("Just after second node computation. feature Error: {:?} and past_upper_bound: {:?}", feature_error, child_upper_bound);
                if feature_error < child_upper_bound {
                    parent_node_data.node_error = feature_error;
                    parent_node_data.test = *attribute;
                    child_upper_bound = feature_error;
                    cache.update(&current_itemset, parent_node_data);

                }




                // if cached_node.data.node_error ==  cached_node.data.lower_bound{
                //     println!("We get the best solution. So, we break the remaining attributes");
                //     break
                // }


            }
            else {
                // if first_node_data.node_error == <f64>::MAX {
                //     let temp_lb = first_node.data.lower_bound + second_lb;
                //     min_lb = match min_lb < temp_lb {
                //         true => {min_lb},
                //         _ => {temp_lb}
                //     };
                // }
                //
                //     else{
                //         let temp_lb = first_node.data.node_error + second_lb;
                //     }
                continue;
                }
            println!("New upper bound: {:?}", child_upper_bound);


            }

        return (cache, its_op, parent_node_data)
        }




    pub fn run(&mut self) -> (Trie, ItemsetOpsChunked<'a>, Node) {
        let mut candidates_list: Vec<Attribute> = Vec::new();
        if self.min_support == 1 {
            candidates_list = (0..self.nattributes).collect::<Vec<Attribute>>(); // TODO: Information Gain ???
        } else {
            for i in 0..self.nattributes {
                if self.its_op.temp_union(&(i, false)) >= self.min_support as usize && self.its_op.temp_union(&(i, true)) >= self.min_support as usize {
                    candidates_list.push(i);
                }
            }
        }

        println!("{:?}", candidates_list);

        let mut cache  = Trie::new();
        let mut its_ops = ItemsetOpsChunked::new(self.its_op.data, Option::from(self.min_support as usize), None, self.ntransactions, false, self.its_op.data.data[0].len());

        let empty_itemset: Vec<Item> = vec![];
        let mut data = DL85::recursion(cache, its_ops, empty_itemset, <usize>::MAX, candidates_list, self.max_error, 0, self.max_depth, self.min_support, self.max_error, Node::new(<usize>::MAX, 0));
        return data;
    }

    fn check_if_stop_condition_reached(mut node: Node, upper_bond: f64, min_support: u64, current_support: u64, depth: u64, max_depth: u64) -> (bool, Node) { // TODO: Here we check if the node already exists. If not we create new one and return his address

        // if node.is_new {
        //     println!("This is a new node");
        //     return (false, node);
        // }

        if depth == max_depth || current_support < (2 * min_support)  as u64 {
            println!("We have reached the either the max depth or there is not enough elements to make a split with an upper_bound = {} and a leaf_error = {}", upper_bond, node.leaf_error);
            node.node_error = node.leaf_error;
            node.is_leaf = true;
            node.is_new = false;
            return (true, node);
        }
        if upper_bond <= node.lower_bound{
            println!("Infeasible solution because upper_bound = {} <= saved_lower_bound = {}", upper_bond, node.lower_bound);
            return (true, node);
        }

        if node.leaf_error == 0. {
            println!("We can not split anymore because we are at a leaf and a pure one = {} and a leaf_error = {}", upper_bond, node.leaf_error);
            node.node_error = node.leaf_error;
            node.is_leaf = true;
            node.is_new = false;
            return (true, node);
        }
        println!("We can go further.");
        return (false, node);

    }


    pub fn retrieve_cache_emplacement_for_current_its(cache_ref: &'a mut Trie, item : &Item, depth: u64, its_op: &mut ItemsetOpsChunked) -> Node { //TODO:  Here we do the creation of the new cache emplacement and compute the error
        let mut its = its_op.current.clone();
        its.sort();
        let mut node  = cache_ref.insert(&its);
        println!("Ohhh {:?}: ", its_op.current);


        if node.is_new{
            let error = its_op.leaf_misclassication_error();
            node.data = Node::new(item.0, depth);
            node.data.leaf_error = error.0 as f64;
            node.is_new = false;
        }
        node.data.clone()



    }

    fn get_cached_node(cache: &'a mut Trie, itemset: &Vec<Item>) -> Option<&'a mut TrieNode> {
        let node = cache.get(itemset);
        return node;
    }


    // fn recursive_dl85(&mut self, current_itemset: &Vec<Item>, last_attribute: Attribute, next_candidates: Vec<Attribute>, upper_bound: f64, depth: u64) -> TrieNode {
    //
    //     let  mut cache_ref = &mut self.cache;
    //
    //
    //
    //     // let mut cached_node = DL85::get_cached_node(&mut cache_ref, current_itemset);
    //     let mut cached_node = cache_ref.get(current_itemset);
    //
    //     if cached_node.is_some() {  // TODO: If there is not node
    //         println!("The node exists in the cache.");
    //         let mut node_ref = cached_node.as_mut().unwrap();
    //         let current_support = self.its_op.support() as u64;
    //         DL85::check_if_stop_condition_reached(&mut node_ref, upper_bound, self.min_support, current_support, depth, self.max_depth);
    //        // return *node_ref;
    //     }
    //
    //     let mut cached_node = cached_node.as_mut().unwrap();
    //
    //
    //     let new_candidates = DL85::get_next_sucessors(&next_candidates, last_attribute, &mut self.its_op, self.min_support);
    //
    //     if new_candidates.len() == 0 {
    //         cached_node.data.node_error = cached_node.data.leaf_error;
    //         cached_node.is_new = false;
    //         println!("There are not candidates. Node error becomes leaf error. upper_bound = {}, depth = {}", upper_bound, depth);
    //         //return cached_node;
    //     }
    //
    //     for attribute in new_candidates {
    //
    //         let first_lb = -1f64;
    //         let second_lb = -1f64;
    //
    //         let items: Vec<Item> = vec![(attribute, false), (attribute, true)];
    //         let first_item_sup = self.its_op.union_cover(&items[0]); // Here current is supposed to be updated
    //
    //
    //         let mut child_item_set = current_itemset.clone();
    //         child_item_set.push(items[0]);
    //
    //
    //
    //         let mut first_node = DL85::retrieve_cache_emplacement_for_current_its(cache_ref, &items[0], depth, &mut self.its_op); // Error computation // cache_ref, item_ref, depth
    //         first_node.data = Node::new(items[0], depth);
    //
    //         first_node.data.lower_bound = match first_node.is_new {
    //             true => {
    //                 if first_lb > first_node.data.lower_bound{
    //                     first_node.data.lower_bound
    //                 }
    //                 else {
    //                     first_lb
    //                 }
    //             },
    //             _ => {first_lb}
    //         };
    //
    //      self.recursive_dl85(&child_item_set, attribute, new_candidates.clone(), 0.0, depth + 1);
    //
    //         let first_split_error = first_node.data.node_error;
    //         self.its_op.backtrack();
    //
    //
    //
    //
    //
    //
    //     }
    //
    //
    //     return  TrieNode {
    //         item: (0, false),
    //         data: Node {
    //             current_depth: 0,
    //             test: (0, false),
    //             leaf_error: 0.0,
    //             node_error: 0.0,
    //             lower_bound: 0.0
    //         },
    //         sub_trie: TrieEdges { edges: vec![] },
    //         is_new: false
    //     };
    // }



    fn get_next_sucessors(candidates: &Vec<Attribute>, last_attribute: Attribute, its_op: &mut ItemsetOpsChunked<'a>, min_support: u64) -> Vec<Attribute> {
        let mut next_candidates = vec![];
        let current_support = its_op.support();

        for candidate in candidates {
            if *candidate == last_attribute{
                continue
            }
            let left_sup = its_op.temp_union(&(*candidate, false));
            let right_sup = current_support - left_sup;
            if left_sup >= min_support as usize && right_sup >= min_support as usize {
                next_candidates.push(*candidate)
            }
        }
        next_candidates
    }


    fn get_candidates_support(&mut self, candidates: &Vec<usize>) -> Vec<(usize, usize)> {
        let mut all_supports = vec![];
        for candidate in candidates {
            println!("{:?}", candidate);
            let items = vec![(*candidate, true), (*candidate, false)];
            let mut c_supports = vec![];
            for it in items {
                c_supports.push(self.its_op.union_cover(&it));
                self.its_op.backtrack();
            }
            all_supports.push((c_supports[0], c_supports[1]));
        }
        all_supports
    }
}
