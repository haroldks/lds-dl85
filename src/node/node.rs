use crate::mining::types_def::Attribute;

struct NodeData {
    leaf_error : f64,
    error : f64,


}

pub struct Node<'a> {

    test: Attribute,
    data: &'a NodeData,
    left: &'a Node <'a>,
    right: &'a Node <'a>,

}


