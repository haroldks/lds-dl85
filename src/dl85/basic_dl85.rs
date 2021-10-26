use std::time::Duration;
use std::time::Instant;

use clokwerk::{Scheduler, TimeUnits};
use float_cmp::{ApproxEq, F64Margin};
use plotters::prelude::*;

use crate::cache::trie::{Trie};
use crate::mining::itemset_bitvector_trait::ItemsetBitvector;
use crate::mining::types_def::{Attribute, Item};
use crate::node::node::Node;

static mut CURRENT_ERROR: f64 = 0.;
static mut ERRORS: Vec<f32> = vec![];

#[allow(unused_variables)]
fn make_a_plot(array: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data.png", (640, 480)).into_drawing_area();
    let lol = array.iter().enumerate().map(|x| (x.0 as f32, *x.1)).collect::<Vec<(f32, f32)>>();

    if let Err(e) = root.fill(&WHITE) {
        println!("Writing error: {}", e.to_string());
    };
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Error Plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..(lol.len() as f32), IntoLogRange::log_scale(170f32..<f32>::MAX))?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        lol.clone(),
        &RED,
    ))?;
    // Similarly, we can draw point series
    // chart.draw_series(PointSeries::of_element(
    //     lol,
    //     5,
    //     &RED,
    //     &|c, s, st| {
    //         return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //             + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
    //             + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
    //     },
    // ))?;
    Ok(())
}


#[allow(dead_code)]
pub struct DL85 {
    // TODO: Allow it to use generic types for differents ITS and DATA. Also solve the problem of the cache and its by removing them from the attributes'
    ntransactions: usize,
    nattributes: usize,
    nclasses: usize,

}

#[allow(unused_assignments)]
impl<'a> DL85 {
    pub fn new(data: (usize, usize, usize)) -> DL85 {
        DL85 {
            ntransactions: data.0,
            nattributes: data.1,
            nclasses: data.2,

        }
    }

    pub fn run<T: ItemsetBitvector>(&mut self, min_support: u64, max_depth: u64, max_error: f64, time_limit: f64, error_save_time: i32, use_info_gain: bool, use_discrepancy: bool, reload_cache: bool, mut its_ops: T, mut cache: Trie) -> (Trie, T, Node, Instant) {
        let init_distribution = its_ops.classes_cover();
        println!("Train distribution: {:?}", init_distribution);


        let mut scheduler = Scheduler::new(); // Scheduler for the error save time

        #[allow(unused_variables)]
            let thread_handle; // The thread handler to stop

        let mut candidates_list: Vec<Attribute> = Vec::new();


        if min_support == 1 {
            candidates_list = (0..self.nattributes).collect::<Vec<Attribute>>(); // TODO: Information Gain ???
        } else {
            for i in 0..self.nattributes {
                if its_ops.temp_intersection(&(i, false)) >= min_support as usize && its_ops.temp_intersection(&(i, true)) >= min_support as usize {
                    candidates_list.push(i);
                }
            }
        }





        if use_info_gain {
            let data = DL85::sort_by_information_gain(its_ops, candidates_list);
            its_ops = data.0;
            candidates_list = data.1;
        }


        if error_save_time >= 0 {
            unsafe {
                scheduler.every((error_save_time as u32).seconds()).run(move || {
                    let temp_error = CURRENT_ERROR;
                    ERRORS.push(temp_error as f32);
                });
            };

            thread_handle = scheduler.watch_thread(Duration::from_millis(100));
        }

        return if !use_discrepancy {
            let empty_itemset: Vec<Item> = vec![];

            let now = Instant::now();

            let data = DL85::recursion(cache, its_ops, empty_itemset, <usize>::MAX, candidates_list, max_error, 0, max_depth, use_discrepancy, None, None, min_support, max_error, Node::new(<usize>::MAX, 0), now, time_limit, use_info_gain, reload_cache);
            println!("Duration:  {:?} milliseconds", data.3.elapsed().as_millis());

            if error_save_time > 0 {
                //thread_handle.stop();
                unsafe {
                    println!("Errors for each {} seconds : {:?}", error_save_time, ERRORS);
                    if let Err(e) = make_a_plot(ERRORS.clone()) {
                        println!("Writing error: {}", e.to_string());
                    };
                }
            }

            data
        } else {


            let len = candidates_list.len() - 1;
            let mut max_discrepancy = len;
            for i in 1..max_depth {
                max_discrepancy += len - i as usize;
            }
            println!("Max discrepancy: {}", max_discrepancy ); // TODO: Change max discrepancy handling
            let empty_itemset: Vec<Item> = vec![];
            let mut reload_cache =  false;
            let now = Instant::now();
            let total = Instant::now();
            println!("Starting Discrepancy {}", 0);
            let mut data= DL85::recursion(cache, its_ops, empty_itemset.clone(), <usize>::MAX, candidates_list.clone(), max_error, 0, max_depth, use_discrepancy, Some(0), Some(0), min_support, max_error, Node::new(<usize>::MAX, 0), now, time_limit, use_info_gain, reload_cache);
            println!("Duration:  {:?} milliseconds for discrepancy: {}", data.3.elapsed().as_millis(), 0);
            println!("Cache size for discrepancy {},  :  {}",  0, data.0.cachesize);

            reload_cache = true;
            for discrepancy in 1..max_discrepancy+1{
                cache = data.0;
                its_ops =  data.1;
                its_ops.reset();
                println!("Starting Discrepancy {}", discrepancy);
                let now = Instant::now();
                data = DL85::recursion(cache, its_ops, empty_itemset.clone(), <usize>::MAX, candidates_list.clone(), max_error, 0, max_depth, use_discrepancy, Some(0), Some(discrepancy as u64), min_support, max_error, Node::new(<usize>::MAX, 0), now, time_limit, use_info_gain, reload_cache);
                println!("Duration:  {:?} milliseconds for discrepancy: {}", data.3.elapsed().as_millis(), discrepancy);
                println!("Cache size for discrepancy {},  :  {}",  discrepancy, data.0.cachesize);
            }
            println!("Total Duration:  {:?} milliseconds for the LDS", total.elapsed().as_millis());




            if error_save_time > 0 {
                //thread_handle.stop();
                unsafe {
                    println!("Errors for each {} seconds : {:?}", error_save_time, ERRORS);
                    if let Err(e) = make_a_plot(ERRORS.clone()) {
                        println!("Writing error: {}", e.to_string());
                    };
                }
            }
            data
        };


    }


    fn recursion<T: ItemsetBitvector>(mut cache: Trie, mut its_op: T, current_itemset: Vec<Item>, last_attribute: Attribute, next_candidates: Vec<Attribute>, upper_bound: f64, depth: u64, max_depth: u64, use_discrepancy: bool, current_discrepancy: Option<u64>, max_discrepancy: Option<u64>, min_support: u64, max_error: f64, mut parent_node_data: Node, instant: Instant, time_limit: f64, use_info_gain: bool, reload_cache: bool) -> (Trie, T, Node, Instant) {
        unsafe {
            CURRENT_ERROR = cache.root.data.node_error;
        }


        let mut child_upper_bound = upper_bound;
        let _min_lb = <f64>::MAX;

        let mut out_of_time = false; // TODO: Use a function to check the out of time.
        if time_limit > 0. {
            if instant.elapsed().as_secs() as f64 > time_limit {
                out_of_time = true;
            }
        }

        let current_support = its_op.support() as u64;


        let data = match use_discrepancy {
            false => { DL85::check_if_stop_condition_reached(parent_node_data, upper_bound, min_support, current_support, depth, max_depth, out_of_time, reload_cache, None, None) }
            _ => { DL85::check_if_stop_condition_reached(parent_node_data, upper_bound, min_support, current_support, depth, max_depth, out_of_time, reload_cache, current_discrepancy, max_discrepancy) }
        };



        if data.0 {
            cache.update(&current_itemset, data.1);
            return (cache, its_op, data.1, instant);
        }

        if out_of_time {
            parent_node_data.node_error = parent_node_data.leaf_error;
            return (cache, its_op, parent_node_data, instant);
        }


        let mut new_candidates = DL85::retrieve_next_sucessors(&next_candidates, last_attribute, &mut its_op, min_support);

        if new_candidates.is_empty() {
            parent_node_data.node_error = parent_node_data.leaf_error;
            parent_node_data.is_explored = true;
            cache.update(&current_itemset, parent_node_data);
            return (cache, its_op, parent_node_data, instant);
        }

        if use_info_gain {
            let data = DL85::sort_by_information_gain(its_op, new_candidates);
            its_op = data.0;
            new_candidates = data.1;
        }
        for (idx, attribute) in new_candidates.iter().enumerate() {

            let child_discrepancy = match use_discrepancy {
                false => { None }
                _ => { Some(current_discrepancy.unwrap() + idx as u64)}
            };

            if use_discrepancy && child_discrepancy.unwrap() > max_discrepancy.unwrap() {
                break;
            }
            let items: Vec<Item> = vec![(*attribute, false), (*attribute, true)];
            let _first_item_sup = its_op.intersection_cover(&items[0]); // Here current is supposed to be updated


            let mut child_item_set = current_itemset.clone();
            child_item_set.push(items[0]);
            child_item_set.sort_unstable();


            let mut first_node_data = DL85::retrieve_cache_emplacement_for_current_its(&mut cache,  &mut its_op, &items[0], depth, current_discrepancy); // Error computation // cache_ref, item_ref, depth


            let data = DL85::recursion(cache, its_op, child_item_set.clone(), *attribute, new_candidates.clone(), child_upper_bound, depth + 1, max_depth, use_discrepancy, child_discrepancy, max_discrepancy, min_support, max_error, first_node_data, instant, time_limit, use_info_gain, reload_cache);

            cache = data.0;
            its_op = data.1;
            first_node_data = data.2;
            first_node_data.is_explored = true;
            if time_limit > 0. {
                if data.3.elapsed().as_secs() as f64 > time_limit {
                    out_of_time = true;
                }
            }
            if out_of_time {
                parent_node_data.is_explored = false;
                first_node_data.is_explored = false;
                cache.update(&child_item_set, first_node_data);
                break;
            }

            cache.update(&child_item_set, first_node_data);

            let first_split_error = first_node_data.node_error;
            its_op.backtrack();

            if first_node_data.node_error < upper_bound {
                let _second_item_sup = its_op.intersection_cover(&items[1]);

                let mut child_item_set = current_itemset.clone();
                child_item_set.push(items[1]);
                let mut second_node_data = DL85::retrieve_cache_emplacement_for_current_its(&mut cache, &mut its_op, &items[1], depth, current_discrepancy); // Error computation // cache_ref, item_ref, depth


                let remaining_ub = child_upper_bound - first_split_error;
                child_item_set.sort_unstable();


                let data = DL85::recursion(cache, its_op, child_item_set.clone(), *attribute, new_candidates.clone(), remaining_ub, depth + 1, max_depth, use_discrepancy, child_discrepancy, max_discrepancy, min_support, max_error, second_node_data, instant, time_limit, use_info_gain, reload_cache);

                cache = data.0;
                its_op = data.1;
                second_node_data = data.2;
                second_node_data.is_explored = true;
                if time_limit > 0. {
                    if data.3.elapsed().as_secs() as f64 > time_limit {
                        out_of_time = true;
                    }
                }
                if out_of_time {
                    parent_node_data.is_explored = false;
                    second_node_data.is_explored = false;
                    cache.update(&child_item_set, second_node_data);
                    break;
                }
                cache.update(&child_item_set, second_node_data);
                let second_split_error = second_node_data.node_error;
                its_op.backtrack();


                let feature_error = first_split_error + second_split_error;

                if feature_error < child_upper_bound {
                    parent_node_data.node_error = feature_error;
                    parent_node_data.test = *attribute;
                    child_upper_bound = feature_error;

                    cache.update(&current_itemset, parent_node_data);
                }
            } else {
                if time_limit > 0. {
                    if data.3.elapsed().as_secs() as f64 > time_limit {
                        out_of_time = true;
                    }
                }
                if out_of_time {
                    parent_node_data.is_explored = false;
                    break;
                }

                continue;
            }
        }
        // if out_of_time {
        //     cache.set_node_exploration_status(&current_itemset, false);
        // }
        //cache.set_node_exploration_status(&current_itemset, true);
        cache.is_done = true;
        (cache, its_op, parent_node_data, instant)
    }




    fn sort_by_information_gain<T: ItemsetBitvector>(mut its_op: T, mut candidates: Vec<Attribute>) -> (T, Vec<Attribute>) {
        let actual_classes_cover = its_op.classes_cover();
        let mut candidate_sort_by_ig = vec![];
        for attribute in &candidates {
            let attribute_classes_cover = its_op.temp_classes_cover(&(*attribute, false));
            let info_gain = T::information_gain(&actual_classes_cover, attribute_classes_cover, its_op.get_nclasses());
            candidate_sort_by_ig.push((attribute, info_gain));
        }
        candidate_sort_by_ig.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        candidates = candidate_sort_by_ig.iter().map(|(a, _)| **a).collect::<Vec<Attribute>>();
        (its_op, candidates)
    }


    fn check_if_stop_condition_reached(mut node: Node, upper_bond: f64, min_support: u64, current_support: u64, depth: u64, max_depth: u64, out_of_time: bool, reload_cache: bool, current_discrepancy:Option<u64>, max_discrepancy: Option<u64>) -> (bool, Node) { // TODO: Here we check if the node already exists. If not we create new one and return his address

        // unsafe {
        //     CAME_HERE += 1;
        // }



        if out_of_time {

            node.node_error = node.leaf_error;
            node.is_explored = false;
            return (true, node);
        }




        if reload_cache {
            if current_discrepancy.is_some() {
                if (current_discrepancy.unwrap() > max_discrepancy.unwrap()) && node.is_explored{
                    node.current_discrepancy = max_discrepancy;
                    return (true, node);
                }
            }
            else {
                if node.is_explored {
                    return (true, node)
            }
                node.node_error = <f64>::MAX;
            }

        }


        if current_discrepancy.is_some(){
            if (current_discrepancy.unwrap() > max_discrepancy.unwrap()) && node.is_explored{
                return (true, node);
            }
        }

        if depth == max_depth || current_support < (2 * min_support) as u64 {
            node.node_error = node.leaf_error;
            node.is_leaf = true;
            node.is_explored = true;
            return (true, node);
        }
        if upper_bond <= node.lower_bound {
            node.node_error = node.leaf_error;
            node.is_explored = true;
            return (true, node);
        }

        if node.leaf_error.approx_eq(0., F64Margin { ulps: 2, epsilon: 0.0 }) {
            node.node_error = node.leaf_error;
            node.is_leaf = true;
            node.is_explored = true;
            return (true, node);
        }
        (false, node)
    }


    fn retrieve_cache_emplacement_for_current_its<T: ItemsetBitvector>(cache_ref: &'a mut Trie, its_op: &mut T, item: &Item, depth: u64, current_discrepancy: Option<u64>) -> Node { //TODO:  Here we do the creation of the new cache emplacement and compute the error
        let mut its = its_op.get_current();
        its.sort_unstable();
        let mut node = cache_ref.insert(&its);


        if node.is_new {
            let error = its_op.leaf_misclassication_error();
            node.data = Node::new(item.0, depth);
            node.data.leaf_error = error.0 as f64;
            node.data.max_class = error.1;
            node.is_new = false; // Weird
            if current_discrepancy.is_some(){
                node.data.current_discrepancy = current_discrepancy;
            }
        }
        node.data
    }

    fn retrieve_next_sucessors<T: ItemsetBitvector>(candidates: &Vec<Attribute>, last_attribute: Attribute, its_op: &mut T, min_support: u64) -> Vec<Attribute> {
        let mut next_candidates = vec![];
        let current_support = its_op.support();

        for candidate in candidates {
            if *candidate == last_attribute {
                continue;
            }
            let left_sup = its_op.temp_intersection(&(*candidate, false));
            let right_sup = current_support - left_sup;
            if left_sup >= min_support as usize && right_sup >= min_support as usize {
                next_candidates.push(*candidate)
            }
        }
        next_candidates
    }


    // fn get_candidates_support(&mut self, candidates: &Vec<usize>) -> Vec<(usize, usize)> {
    //     let mut all_supports = vec![];
    //     for candidate in candidates {
    //         let items = vec![(*candidate, true), (*candidate, false)];
    //         let mut c_supports = vec![];
    //         for it in items {
    //             c_supports.push(self.its_op.union_cover(&it));
    //             self.its_op.backtrack();
    //         }
    //         all_supports.push((c_supports[0], c_supports[1]));
    //     }
    //     all_supports
    // }
}

