use crate::mining::types_def::Item;

pub trait ItemsetBitvector {
    fn intersection_cover(&mut self, second_its: &Item) -> usize;

    fn temp_intersection(&mut self, second_its: &Item) -> usize;

    fn backtrack(&mut self);

    fn reset(&mut self);

    fn support(&mut self) -> usize;

    fn classes_cover(&mut self) -> Vec<usize>;

    fn top_class(&mut self) -> (usize, usize);

    fn leaf_misclassication_error(&mut self) -> (usize, usize);

    fn get_infos(&self) -> (usize, usize, usize);

    fn get_current(&self) -> Vec<Item>;

    fn get_nclasses(&self) -> usize;

    fn temp_classes_cover(&mut self, second_its: &Item) -> Vec<usize>;

    fn information_gain(actual_cover: &Vec<usize>, left_split_attribute_cover: Vec<usize>, nclasses: usize) -> f64 {
        let right_attribute_cover = actual_cover.iter().enumerate()
            .map(|(idx, val)| *val - left_split_attribute_cover[idx]).collect::<Vec<usize>>();

        let actual_size = actual_cover.iter().sum::<usize>();
        let left_split_size = left_split_attribute_cover.iter().sum::<usize>();
        let right_split_size = right_attribute_cover.iter().sum::<usize>();


        let left_weight = match actual_size {
            0 => { 0f64 }
            _ => { left_split_size as f64 / actual_size as f64 }
        };

        let right_weight = match actual_size {
            0 => { 0f64 }
            _ => { right_split_size as f64 / actual_size as f64 }
        };

        let mut left_split_entropy = 0f64;
        let mut right_split_entropy = 0f64;
        let mut parent_entropy = 0f64;
        for class in 0..nclasses {
            let p = match actual_size {
                0 => { 0f64 }
                _ => { actual_cover[class] as f64 / actual_size as f64 }
            };

            let mut log_val = 0f64;
            if p > 0. {
                log_val = p.log2();
            }
            parent_entropy += -p * log_val;

            let p = match left_split_size {
                0 => { 0f64 }
                _ => { left_split_attribute_cover[class] as f64 / left_split_size as f64 }
            };

            let mut log_val = 0f64;
            if p > 0. {
                log_val = p.log2();
            }
            left_split_entropy += -p * log_val;

            let p = match right_split_size {
                0 => { 0f64 }
                _ => { right_attribute_cover[class] as f64 / right_split_size as f64 }
            };

            let mut log_val = 0f64;
            if p > 0. {
                log_val = p.log2();
            }
            right_split_entropy += -p * log_val;
        }
        parent_entropy - (left_weight * left_split_entropy + right_weight * right_split_entropy)
    }

    fn max_search_tree_space(&self, nfeatures: u128, max_depth: u128) -> u128 {
        return if max_depth == 0 {
            0
        } else {
            let mut count = nfeatures * 2;
            let mut past_depth = count;
            for depth in 2..max_depth + 1 {
                if nfeatures.saturating_sub(depth - 1) == 0 {
                    break;
                }
                past_depth = past_depth * 2 * (nfeatures - depth + 1);
                count += past_depth;
            }
            count
        };


    }
}
