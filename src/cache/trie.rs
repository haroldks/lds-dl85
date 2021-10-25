use std::fmt;
use std::fmt::Formatter;

use crate::mining::types_def::Item;
use crate::node::node::Node;

#[derive(Debug)]
pub struct TrieEdges {
    pub edges: Vec<TrieNode>,
}


impl TrieEdges {
    pub fn new<'a>() -> TrieEdges {
        TrieEdges { edges: vec![] }
    }
}

impl fmt::Display for TrieEdges {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{  ");
        for i in &self.edges {
            write!(f, "\t{}", i);
        }
        write!(f, " }}");
        Ok(())
    }
}


#[derive(Debug)]
pub struct TrieNode {
    pub item: Item,
    pub data: Node,
    pub sub_trie: TrieEdges,
    pub is_new: bool,
}

impl fmt::Display for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ Item:  ({}, {}),  is_new:  {}  NodeData :  {},  Edges:  {}}}", self.item.0, self.item.1, self.is_new, self.data, self.sub_trie)
    }
}


impl TrieNode {
    pub fn new(item: Item) -> TrieNode {
        TrieNode { item, sub_trie: TrieEdges::new(), data: Node::new(item.0, 0), is_new: true }
    }
}


pub struct Trie {
    pub root: TrieNode,
    pub cachesize: u64,
    pub is_done: bool,
}

#[allow(dead_code)]
impl Trie {
    pub fn new() -> Trie {
        Trie { root: TrieNode::new((usize::MAX, false)), cachesize: 0, is_done: false }
    }

    pub fn get(&mut self, key: &Vec<Item>) -> Option<&mut TrieNode> {
        let mut node = &mut self.root;

        for item in key.iter().enumerate() {
            let sub_trie = &mut node.sub_trie;
            let next = sub_trie.edges.iter_mut().find(|x| (**x).item == *item.1);
            if next.is_none() {
                return None;
            } else {
                node = next.unwrap();
            }
        }
        Option::from(node)
    }

    pub fn insert(&mut self, key: &Vec<Item>) -> &mut TrieNode {
        let mut node = &mut self.root;

        for item in key.iter().enumerate() {
            let sub_trie = &mut node.sub_trie;
            let next = sub_trie.edges.iter().position(|x| x.item == *item.1);
            if next.is_none() {
                self.cachesize += 1;
                let len = sub_trie.edges.len() + 1;
                let new_node = TrieNode::new(*item.1);
                sub_trie.edges.push(new_node);

                node = &mut sub_trie.edges[len - 1];
            } else {
                node = &mut sub_trie.edges[next.unwrap()];
            }
        }
        node
    }

    pub fn update(&mut self, itemset: &Vec<Item>, node_data: Node) -> bool {
        let node = self.get(itemset);
        let node_ref = node.unwrap();
        node_ref.is_new = false;
        node_ref.data = node_data;
        true
    }

    pub fn set_node_exploration_status(&mut self, itemset: &Vec<Item>, status: bool) -> bool {
        let node = self.get(itemset);
        let node_ref = node.unwrap();
        node_ref.data.is_explored = status;
        true
    }

    pub fn gen_final_tree(&mut self) {
        if !self.is_done {
            println!("The Cache is not fully loaded. This method should be called at the end of the search");
        }
    }
}
