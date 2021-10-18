use crate::mining::types_def::Item;

pub trait ItemsetBitvector {

    fn intersection_cover(&mut self, second_its: &Item) -> usize;

    fn temp_intersection(&mut self, second_its: &Item) -> usize;

    fn backtrack(&mut self);

    fn support(&mut self) -> usize;

    fn classes_cover(&mut self) -> Vec<usize>;

    fn top_class(&mut self) -> (usize, usize);

    fn leaf_misclassication_error(&mut self) -> (usize, usize);

    fn get_infos(&self) -> (usize, usize, usize);



}
