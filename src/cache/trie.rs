use crate::mining::types_def::Item;
use crate::node::node::Node;
use std::fmt;
use std::fmt::Formatter;
#[derive(Debug)]
pub struct TrieEdges  {
   pub edges : Vec<TrieNode>
}


impl TrieEdges {
    pub fn new<'a>() -> TrieEdges{
        TrieEdges { edges: vec![], }
    }

    pub fn get_related_node<'a>(&mut self) -> &mut Vec<TrieNode> {
        &mut self.edges
    }
}

impl fmt::Display for TrieEdges {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{  ");
        for i in &self.edges{
            write!(f, "\t{}", i);
        }
        write!(f, " }}");
        Ok(())

    }
}



#[derive(Debug)]
pub struct TrieNode {
    pub item : Item,
    pub data: Node,
    pub sub_trie : TrieEdges,
    pub is_new: bool
}

impl fmt::Display for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ Item:  ({}, {}),  NodeData :  {},  Edges:  {}}}", self.item.0, self.item.1, self.data, self.sub_trie)

    }
}
//
 impl TrieNode {
    pub fn new(item: Item, ) -> TrieNode {
        TrieNode { item, sub_trie: TrieEdges::new(), data: Node::new(item, 0), is_new: true }
    }
//     pub fn get_sub_trie<'a>(&mut self) -> &mut TrieEdges {
//         &mut self.sub_trie
//     }
//
 }


pub struct Trie {

    pub root : TrieNode,
    pub cachesize: u64

}


impl Trie {

    pub fn new() -> Trie {
        Trie { root: TrieNode::new((usize::MAX, false)), cachesize: 0}
    }

    pub fn get(&mut self, key : &Vec<Item>) -> Option<&mut TrieNode> {
        let mut node = &mut self.root;

        for item in key.iter().enumerate() {
            let mut sub_trie= &mut node.sub_trie;
            let mut next  = sub_trie.edges.iter_mut().find(|x| (**x).item == *item.1);
            if next.is_none(){
                return None;
            }
            else {
                 node  = next.unwrap();
            }
        }
        Option::from(node)

    }

    pub fn insert(&mut self, key : &Vec<Item>) -> &mut TrieNode {
        let mut node = &mut self.root;

        for item in key.iter().enumerate() {

            let mut sub_trie= &mut node.sub_trie;
            let mut next = sub_trie.edges.iter().position(|x| x.item == *item.1);
            if  next.is_none(){
                self.cachesize += 1;
                let len = sub_trie.edges.len() + 1;
                let mut new_node = TrieNode::new(*item.1);
                sub_trie.edges.push(new_node);

                node = &mut sub_trie.edges[len - 1];

            }
            else {
                node = &mut sub_trie.edges[next.unwrap()];

            }
        }
        node
    }

    pub fn update(itemset : &Vec<Item>){
        unimplemented!()
    }

}
