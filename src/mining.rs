use bit_vec::BitVec;

pub type Transactions = Vec<BitVec>;
pub type Attribute = u32;
pub type Item = (Attribute, bool);
//pub type Itemset = Vec<Item>; // I can use a structure to have all data stored in it quickly like support, freq and so one when each function is called


pub struct Itemset{

    its: Vec<Item>,
    support: Option<f32>,
    frequency: Option<f32>

}

impl Itemset{

    // pub fn new(its: Vec<Item>, support: Option<f32>, frequency: Option<f32>) -> Itemset {
    //     Itemset(its, support, frequency)
    // }

    pub fn cover(&self){
        unimplemented!("Return a view of all the transactions associated with the actual itemset.");
    }

    pub fn union_cover(&self, second_its: &Itemset, transactions: &Transactions){
        unimplemented!("Union of two itemsets. Should return the cover of the new Itemset. All the\
        transactions associatied to the new itemset");
    }

    pub fn support(&self){

        unimplemented!("Return the support of the actual itemset.");

    }

    pub fn freq(&self){
        unimplemented!("Return the freq of the actual itemset.");
    }

    pub fn compute_infos(&self,  transactions: &Transactions){
        unimplemented!("Compute all for an itemset on a set of transactions.")
    }

}


