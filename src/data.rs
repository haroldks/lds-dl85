use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use bit_vec::BitVec;
use std::collections::HashSet;


pub struct Data{
    pub filename: String,
    pub ntransactions: usize,
    pub nattributes : usize,
    pub nclasses:usize,
    pub data: Vec<BitVec>,
    pub target: Vec<BitVec>,
    // chunked: false // Update to use chunks
}


impl Data{

    pub fn new(filename: String) -> Result<Data, Error>{

        let input  = File::open(&filename)?; //Error Handling for missing filename

        let buffered = BufReader::new(input); // Buffer for the file

        // let first_line = buffered.lines().next().unwrap().unwrap();
        let data_lines: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();
        let nattributes= data_lines[0].split_ascii_whitespace().collect::<Vec<&str>>().len() - 1;

        // let lines: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();
        let mut lines = vec![];
        let mut target = vec![];

        for line in data_lines {
            let line = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let mut bv = BitVec::from_elem(nattributes, false);
            for (idx, l) in line.iter().enumerate(){
                match idx {

                    0 => {target.push(l.parse::<usize>().unwrap())},
                    _ => {bv.set(idx - 1, l == &"1")}

                }
            }
            lines.push(bv);

        }

        let ntransactions = lines.len();
        let nclasses = target.iter().collect::<HashSet<_>>().len();

        let mut targets_bv = vec![];

        for _ in 0..nclasses{
            targets_bv.push(BitVec::from_elem(ntransactions, false))
        }

        for (idx, class) in target.iter().enumerate(){
            targets_bv[*class].set(idx, true)
        }



    Ok(Data{filename, ntransactions, nattributes, nclasses, data: lines, target: targets_bv })

    }

}
