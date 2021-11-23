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


    fn gcd(&self, a: u64, b: u64) -> u64
    {
        if b == 0
        {
            return a;
        }
        return self.gcd(b, a % b);
    }

    fn find_n_c_r(&self, objects: u64, subset: u64) -> u64
    {
        let mut n: u64 = objects;
        let mut r: u64 = subset;
        let mut p = 1;
        let mut k = 1;
        if (n - r) < r
        {
            r = n - r;
        }
        if r != 0
        {
            while r >= 1
            {
                p = p * n;
                k = k * r;
                n = n - 1;
                r = r - 1;
                let d = self.gcd(p, k);
                p = p / d;
                k = k / d;
            }
        } else {
            p = 1;
        }
        p
    }

    fn max_cache_nodes(&self, nfeatures: u64, mut depth: u64) -> u64 {
        depth += 1;
        let mut count = 0;
        for d in 0..depth+1{
            count += self.find_n_c_r(nfeatures, d) * 2u64.pow(d as u32)
        }
        count

    }

}
