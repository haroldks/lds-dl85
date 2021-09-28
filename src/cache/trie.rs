use crate::mining::types_def::Item;
use crate::node::node::Node;
use std::fmt;
use std::fmt::Formatter;
#[derive(Debug)]
pub struct TrieEdge  {
    pub item : Item,
    pub sub_trie : TrieNode,
    //data: &'a Node <'a>
}


impl TrieEdge {
    pub fn new<'a>(item: Item) -> TrieEdge{
        TrieEdge { item, sub_trie: TrieNode::new()}
    }
    pub fn get_sub_trie<'a>(&mut self) -> &mut Vec<TrieEdge> {
        &mut self.sub_trie.edges
    }
    pub fn get_related_node<'a>(&mut self) -> &mut TrieNode {
        &mut self.sub_trie
    }
}

impl fmt::Display for TrieEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       return write!(f,
           "{{item: ({}, {}), associated_tree:{}   }}", self.item.0, self.item.1, self.sub_trie
       )
    }
}



#[derive(Debug)]
pub struct TrieNode {
    pub edges : Vec<TrieEdge>
}

impl fmt::Display for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{edges:");
        for i in &self.edges{
            write!(f, "\t{}", i);
        }
        write!(f, " }}");
        Ok(())
    }
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {edges : vec![]}
    }

}


pub struct Trie {

    pub root : TrieNode,
    pub cachesize: u64

}


impl Trie {

    pub fn new() -> Trie {
        Trie { root: TrieNode::new(), cachesize: 0}
    }

    pub fn get(&mut self, key : &Vec<Item>) -> Option<&mut TrieNode> {
        let mut node = &mut self.root;

        for item in key.iter().enumerate() {
            let mut edges= &mut node.edges;
            let mut next  = edges.iter_mut().find(|x| (**x).item == *item.1);
            if next.is_none(){
                return None;
            }
            else {
                let n_ref  = next.unwrap();
                node =  &mut n_ref.sub_trie;
            }
        }
        Option::from(node)

    }

    pub fn insert(&mut self, key : &Vec<Item>) -> &mut TrieNode {
        let mut node = &mut self.root;

        for item in key.iter().enumerate() {

            let mut edges= &mut node.edges;
            let mut next = edges.iter().position(|x| x.item == *item.1);
            if   next.is_none(){
                self.cachesize += 1;
                let len = edges.len() + 1;
                let mut new_edge = TrieEdge::new(*item.1);
                edges.push(new_edge);

                let m = &mut edges[len - 1];
                node = &mut m.sub_trie;

            }
            else {
                let mut edge_ref = &mut edges[next.unwrap()];
                node = &mut edge_ref.sub_trie;
            }
        }
        node
    }

    pub fn update(itemset : &Vec<Item>){
        unimplemented!()
    }

}
