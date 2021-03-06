use std::cmp::min;
use std::time::Duration;
use std::time::Instant;

use clokwerk::{Scheduler, TimeUnits};
use float_cmp::{ApproxEq, F64Margin};

use crate::cache::trie::Trie;
use crate::mining::itemset_bitvector_trait::ItemsetBitvector;
use crate::mining::types_def::{Attribute, Item};
use crate::node::node::Node;

static mut CURRENT_ERROR: f64 = 0.;
static mut ERRORS: Vec<f32> = vec![];

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

    pub fn run<T: ItemsetBitvector>(
        &mut self,
        min_support: u64,
        max_depth: u64,
        max_error: f64,
        time_limit: f64,
        error_save_time: i32,
        use_info_gain: bool,
        use_discrepancy: bool,
        discrepancy_limit: Option<usize>,
        recursion_limit: Option<usize>,
        reload_cache: bool,
        mut its_ops: T,
        mut cache: Trie,
    ) -> (Trie, T, Node, Instant, u128) {
        let init_distribution = its_ops.classes_cover();
        println!("Train distribution: {:?}", init_distribution);

        let mut scheduler = Scheduler::new(); // Scheduler for the error save time

        #[allow(unused_variables)]
        let thread_handle; // The thread handler to stop

        let mut candidates_list: Vec<Attribute> = Vec::new();

        if min_support == 1 {
            candidates_list = (0..self.nattributes).collect::<Vec<Attribute>>();
        // TODO: Information Gain ???
        } else {
            for i in 0..self.nattributes {
                if its_ops.temp_intersection(&(i, false)) >= min_support as usize
                    && its_ops.temp_intersection(&(i, true)) >= min_support as usize
                {
                    candidates_list.push(i);
                }
            }
        }
        println!("Number of itemsets: {:?}", candidates_list.len() * 2);

        if use_info_gain {
            let data = DL85::sort_by_information_gain(its_ops, candidates_list);
            its_ops = data.0;
            candidates_list = data.1;
        }

        if error_save_time >= 0 {
            unsafe {
                scheduler
                    .every((error_save_time as u32).seconds())
                    .run(move || {
                        let temp_error = CURRENT_ERROR;
                        ERRORS.push(temp_error as f32);
                    });
            };
            thread_handle = scheduler.watch_thread(Duration::from_millis(100));
        }

        let rlimit = match recursion_limit.is_some() {
            true => recursion_limit.unwrap(),
            _ => 0,
        };

        return if !use_discrepancy {
            let empty_itemset: Vec<Item> = vec![];
            let now = Instant::now();
            let data = DL85::recursion(
                cache,
                its_ops,
                empty_itemset,
                <usize>::MAX,
                candidates_list,
                max_error,
                0,
                max_depth,
                use_discrepancy,
                None,
                0,
                rlimit,
                min_support,
                max_error,
                Node::new(<usize>::MAX, 0),
                now,
                time_limit,
                use_info_gain,
                reload_cache,
            );
            cache = data.0;
            let final_duration = data.3.elapsed().as_millis();
            if final_duration as f64 >= time_limit * 1000f64 {
                cache.has_timeout = true;
            }
            println!("Duration:  {:?} milliseconds", final_duration);

            if error_save_time > 0 {
                //thread_handle.stop();
                unsafe {
                    println!("Errors for each {} seconds : {:?}", error_save_time, ERRORS);
                    // if let Err(e) = make_a_plot(ERRORS.clone()) {
                    //     println!("Writing error: {}", e.to_string());
                    // };
                }
            }
            let data = (cache, data.1, data.2, data.3, final_duration);
            data
        } else {
            let len = candidates_list.len() - 1;
            let mut max_discrepancy = len;
            max_discrepancy = DL85::get_true_discrepancy_limit(len, max_depth);

            if discrepancy_limit.is_some() {
                max_discrepancy = min(discrepancy_limit.unwrap(), max_discrepancy);
            }
            cache.max_discrepancy = Some(max_discrepancy);
            cache.discrepancy = Some(0);
            cache.root.data.current_discrepancy = Some(0);
            let mut init_node = Node::new(<usize>::MAX, 0);
            init_node.current_discrepancy = Some(0);
            // println!("Max discrepancy: {}", max_discrepancy); // TODO: Change max discrepancy handling
            let empty_itemset: Vec<Item> = vec![];
            let mut reload_cache = false;
            let mut now = Instant::now();
            let mut data = DL85::recursion(
                cache,
                its_ops,
                empty_itemset.clone(),
                <usize>::MAX,
                candidates_list.clone(),
                max_error,
                0,
                max_depth,
                use_discrepancy,
                Some(0),
                max_discrepancy as u64,
                rlimit,
                min_support,
                max_error,
                init_node,
                now,
                time_limit,
                use_info_gain,
                reload_cache,
            );

            reload_cache = true;
            let mut has_timeout = false;

            let mut discrepancy = 1;
            let mut is_last = false;
            // println!("Max Discrepancy : {}", max_discrepancy);
            while discrepancy <= max_discrepancy {
                // println!("Current Discrepancy : {}", discrepancy);
                cache = data.0;
                cache.discrepancy = Some(discrepancy);
                let new_parent_node = cache.root.data.clone();
                let current_error = cache.root.data.node_error;
                let new_upper_bound = match current_error < max_error {
                    true => current_error,
                    _ => max_error,
                };
                its_ops = data.1;
                its_ops.reset();
                now = data.3;
                data = DL85::recursion(
                    cache,
                    its_ops,
                    empty_itemset.clone(),
                    <usize>::MAX,
                    candidates_list.clone(),
                    new_upper_bound,
                    0,
                    max_depth,
                    use_discrepancy,
                    Some(discrepancy as u64),
                    max_discrepancy as u64,
                    rlimit,
                    min_support,
                    new_upper_bound,
                    new_parent_node,
                    now,
                    time_limit,
                    use_info_gain,
                    reload_cache,
                );
                if data.0.root.data.node_error.approx_eq(
                    0.,
                    F64Margin {
                        ulps: 2,
                        epsilon: 0.0,
                    },
                ) {
                    break;
                }
                if time_limit > 0. {
                    if data.3.elapsed().as_secs() as f64 > time_limit {
                        has_timeout = true;
                        println!("Finished at discrepancy: {}", discrepancy);
                        break;
                    }
                }

                if is_last {
                    break;
                }
                discrepancy = DL85::augment_discrepancy(discrepancy, 2);
                if discrepancy >= max_discrepancy {
                    discrepancy = max_discrepancy;
                    is_last = true;
                }
            }

            let final_duration = data.3.elapsed().as_millis();
            println!(
                "Duration:  {:?} milliseconds for discrepancy Search",
                final_duration
            );

            if error_save_time > 0 {
                //thread_handle.stop();
                unsafe {
                    println!("Errors for each {} seconds : {:?}", error_save_time, ERRORS);
                    // if let Err(e) = make_a_plot(ERRORS.clone()) {
                    //     println!("Writing error: {}", e.to_string());
                    // };
                }
            }
            cache = data.0;
            cache.has_timeout = has_timeout;
            let data = (cache, data.1, data.2, data.3, final_duration);
            data
        };
    }

    fn recursion<T: ItemsetBitvector>(
        mut cache: Trie,
        mut its_op: T,
        current_itemset: Vec<Item>,
        last_attribute: Attribute,
        next_candidates: Vec<Attribute>,
        upper_bound: f64,
        depth: u64,
        max_depth: u64,
        use_discrepancy: bool,
        mut max_discrepancy: Option<u64>,
        discrepancy_limit: u64,
        recursion_limit: usize,
        min_support: u64,
        max_error: f64,
        mut parent_node_data: Node,
        instant: Instant,
        time_limit: f64,
        use_info_gain: bool,
        reload_cache: bool,
    ) -> (Trie, T, Node, Instant) {
        unsafe {
            CURRENT_ERROR = cache.root.data.node_error;
        }
        if recursion_limit > 0 && cache.recursion_count >= recursion_limit {
            parent_node_data.is_explored = false;
            parent_node_data.node_error = parent_node_data.leaf_error;
            cache.update(&current_itemset, parent_node_data); // New
            return (cache, its_op, parent_node_data, instant);
        }

        cache.recursion_count += 1;
        let mut child_upper_bound = upper_bound;
        let _min_lb = <f64>::MAX;
        let time_bundle = DL85::check_time_out(instant, time_limit); // TODO: Use a function to check the out of time.
        let out_of_time = time_bundle.0;
        let instant = time_bundle.1;

        if use_discrepancy && out_of_time {
            parent_node_data.is_explored = false;
            parent_node_data.node_error = parent_node_data.leaf_error;
            cache.update(&current_itemset, parent_node_data); // New
            return (cache, its_op, parent_node_data, instant);
        }

        let current_support = its_op.support() as u64;
        let data = match use_discrepancy {
            false => DL85::check_if_stop_condition_reached(
                parent_node_data,
                upper_bound,
                min_support,
                current_support,
                depth,
                max_depth,
                out_of_time,
                reload_cache,
                None,
                discrepancy_limit,
            ),
            _ => DL85::check_if_stop_condition_reached(
                parent_node_data,
                upper_bound,
                min_support,
                current_support,
                depth,
                max_depth,
                out_of_time,
                reload_cache,
                max_discrepancy,
                discrepancy_limit,
            ),
        };

        if data.0 {
            cache.update(&current_itemset, data.1);
            return (cache, its_op, data.1, instant);
        }

        let mut new_candidates = DL85::retrieve_next_successors(
            &next_candidates,
            last_attribute,
            &mut its_op,
            min_support,
        );

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
        let mut real_disc_limit = <usize>::MAX;
        if use_discrepancy {
            real_disc_limit =
                DL85::get_true_discrepancy_limit(new_candidates.len(), max_depth - depth);
            max_discrepancy = Some(min(max_discrepancy.unwrap(), real_disc_limit as u64));
        }

        for (idx, attribute) in new_candidates.iter().enumerate() {
            if recursion_limit > 0 && cache.recursion_count >= recursion_limit {
                break;
            }

            if use_discrepancy && idx as u64 > max_discrepancy.unwrap() {
                break;
            }

            let child_discrepancy = match use_discrepancy {
                false => None,
                _ => Some(max_discrepancy.unwrap() - idx as u64),
            };

            let items: Vec<Item> = vec![(*attribute, false), (*attribute, true)];
            let _first_item_sup = its_op.intersection_cover(&items[0]); // Here current is supposed to be updated
            let mut child_item_set = current_itemset.clone();
            child_item_set.push(items[0]);
            child_item_set.sort_unstable();
            let mut first_node_data = DL85::retrieve_cache_emplacement_for_current_its(
                &mut cache,
                &mut its_op,
                &items[0],
                depth,
                max_discrepancy,
            ); // Error computation // cache_ref, item_ref, depth
            let data = DL85::recursion(
                cache,
                its_op,
                child_item_set.clone(),
                *attribute,
                new_candidates.clone(),
                child_upper_bound,
                depth + 1,
                max_depth,
                use_discrepancy,
                child_discrepancy,
                discrepancy_limit,
                recursion_limit,
                min_support,
                max_error,
                first_node_data,
                instant,
                time_limit,
                use_info_gain,
                reload_cache,
            );

            cache = data.0;
            its_op = data.1;
            first_node_data = data.2;
            first_node_data.is_explored = true;
            cache.update(&child_item_set, first_node_data);
            let first_split_error = first_node_data.node_error;
            its_op.backtrack();

            if first_node_data.node_error < child_upper_bound {
                let _second_item_sup = its_op.intersection_cover(&items[1]);
                let mut child_item_set = current_itemset.clone();
                child_item_set.push(items[1]);
                let mut second_node_data = DL85::retrieve_cache_emplacement_for_current_its(
                    &mut cache,
                    &mut its_op,
                    &items[1],
                    depth,
                    max_discrepancy,
                ); // Error computation // cache_ref, item_ref, depth
                let remaining_ub = child_upper_bound - first_split_error;
                child_item_set.sort_unstable();

                let data = DL85::recursion(
                    cache,
                    its_op,
                    child_item_set.clone(),
                    *attribute,
                    new_candidates.clone(),
                    remaining_ub,
                    depth + 1,
                    max_depth,
                    use_discrepancy,
                    child_discrepancy,
                    discrepancy_limit,
                    recursion_limit,
                    min_support,
                    max_error,
                    second_node_data,
                    instant,
                    time_limit,
                    use_info_gain,
                    reload_cache,
                );

                cache = data.0;
                its_op = data.1;
                second_node_data = data.2;
                second_node_data.is_explored = true;

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
                let time_bundle = DL85::check_time_out(instant, time_limit);
                let out_of_time = time_bundle.0;
                let instant = time_bundle.1;

                if use_discrepancy && out_of_time {
                    parent_node_data.is_explored = false;
                    parent_node_data.node_error = parent_node_data.leaf_error;
                    return (cache, its_op, parent_node_data, instant);
                }
                continue;
            }
        }

        if use_discrepancy
            && (parent_node_data.node_error.approx_eq(
                0.,
                F64Margin {
                    ulps: 2,
                    epsilon: 0.0,
                },
            ) || real_disc_limit as u64 == max_discrepancy.unwrap())
        {
            parent_node_data.current_discrepancy = Some(discrepancy_limit);
            cache.update(&current_itemset, parent_node_data);
        }

        cache.is_done = true;
        (cache, its_op, parent_node_data, instant)
    }

    fn sort_by_information_gain<T: ItemsetBitvector>(
        mut its_op: T,
        mut candidates: Vec<Attribute>,
    ) -> (T, Vec<Attribute>) {
        let actual_classes_cover = its_op.classes_cover();
        let mut candidate_sort_by_ig = vec![];

        for attribute in &candidates {
            let attribute_classes_cover = its_op.temp_classes_cover(&(*attribute, false));
            let info_gain = T::information_gain(
                &actual_classes_cover,
                attribute_classes_cover,
                its_op.get_nclasses(),
            );
            candidate_sort_by_ig.push((attribute, info_gain));
        }

        candidate_sort_by_ig.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        candidates = candidate_sort_by_ig
            .iter()
            .map(|(a, _)| **a)
            .collect::<Vec<Attribute>>();
        (its_op, candidates)
    }

    fn check_if_stop_condition_reached(
        mut node: Node,
        upper_bond: f64,
        min_support: u64,
        current_support: u64,
        depth: u64,
        max_depth: u64,
        out_of_time: bool,
        reload_cache: bool,
        discrepancy: Option<u64>,
        max_discrepancy: u64,
    ) -> (bool, Node) {
        // TODO: Here we check if the node already exists. If not we create new one and return his address
        if out_of_time {
            node.node_error = node.leaf_error;
            node.is_explored = false;
            return (true, node);
        }

        if reload_cache {
            if discrepancy.is_some() {
                if node.current_discrepancy.unwrap() >= max_discrepancy {
                    if upper_bond <= node.lower_bound {
                        return (true, node);
                    }
                }
                if node.node_error.approx_eq(
                    0.,
                    F64Margin {
                        ulps: 2,
                        epsilon: 0.0,
                    },
                ) {
                    node.lower_bound = upper_bond;
                    return (true, node);
                }
            } else {
                if node.is_explored {
                    node.lower_bound = upper_bond;
                    return (true, node);
                }
                node.node_error = <f64>::MAX;
            }
        }
        
        if depth == max_depth || current_support < (2 * min_support) as u64 {
            node.node_error = node.leaf_error;
            node.is_leaf = true;
            node.is_explored = true;
            node.lower_bound = upper_bond;
            return (true, node);
        }

        if upper_bond <= node.lower_bound {
            node.node_error = node.leaf_error;
            node.is_explored = true;
            return (true, node);
        }

        if node.leaf_error.approx_eq(
            0.,
            F64Margin {
                ulps: 2,
                epsilon: 0.0,
            },
        ) {
            node.lower_bound = upper_bond;
            node.node_error = node.leaf_error;
            node.is_leaf = true;
            node.is_explored = true;
            return (true, node);
        }
        (false, node)
    }

    fn retrieve_cache_emplacement_for_current_its<T: ItemsetBitvector>(
        cache_ref: &'a mut Trie,
        its_op: &mut T,
        item: &Item,
        depth: u64,
        current_discrepancy: Option<u64>,
    ) -> Node {
        //TODO:  Here we do the creation of the new cache emplacement and compute the error
        let mut its = its_op.get_current();
        its.sort_unstable();
        let mut node = cache_ref.insert(&its);
        if node.is_new {
            let error = its_op.leaf_misclassication_error();
            node.data = Node::new(item.0, depth);
            node.data.leaf_error = error.0 as f64;
            node.data.max_class = error.1;
            node.is_new = false; // Weird

            if current_discrepancy.is_some() {
                node.data.current_discrepancy = current_discrepancy;
            }
        }
        node.data
    }

    fn retrieve_next_successors<T: ItemsetBitvector>(
        candidates: &Vec<Attribute>,
        last_attribute: Attribute,
        its_op: &mut T,
        min_support: u64,
    ) -> Vec<Attribute> {
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

    fn check_time_out(instant: Instant, time_limit: f64) -> (bool, Instant) {
        return if time_limit > 0. {
            (instant.elapsed().as_secs() as f64 > time_limit, instant)
        } else {
            (false, instant)
        };
    }

    fn get_true_discrepancy_limit(nb_successors: usize, remaining_depth: u64) -> usize {
        let mut max_discrepancy = nb_successors;
        for i in 1..remaining_depth {
            max_discrepancy += nb_successors.saturating_sub(i as usize);
        }
        return max_discrepancy;
    }
    // Two scheme 1 : for incremental and 2 for exponential
    fn augment_discrepancy(actual: usize, scheme: u8) -> usize {
        let next = match scheme {
            1 => actual + 1,
            2 => actual * 2,
            _ => actual + 1,
        };
        return next;
    }
}
