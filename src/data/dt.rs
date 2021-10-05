use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use bit_vec::BitVec;

pub struct Data {
    pub filename: String,
    pub ntransactions: usize,
    pub nattributes: usize,
    pub nclasses: usize,
    pub data: Vec<BitVec>,
    pub target: Vec<BitVec>,
    chunked: bool // Update to use chunks
}


impl Data {
    // TODO: add comments for readability
    pub fn new(filename: String) -> Result<Data, Error> {
        let input = File::open(&filename)?; //Error Handling for missing filename

        let buffered = BufReader::new(input); // Buffer for the file

        let data_lines: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();

        Data::data_as_it(data_lines, filename)
    }

    fn data_as_it(data:Vec<String>, filename: String) -> Result<Data, Error> {
        let nattributes = data[0].split_ascii_whitespace().collect::<Vec<&str>>().len() - 1;
        let ntransactions = data.len();
        // let lines: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();
        let mut inputs = vec![BitVec::from_elem(ntransactions, false); nattributes];
        let mut target = vec![];

        for (i, line) in data.iter().enumerate() {
            let line = line.split_ascii_whitespace().collect::<Vec<&str>>();
            for (j, l) in line.iter().enumerate() {
                match j {
                    0 => { target.push(l.parse::<usize>().unwrap()) }
                    _ => { inputs[j - 1].set(i, l == &"1") }
                }
            }
        }

        let nclasses = target.iter().collect::<HashSet<_>>().len();

        let mut targets_bv = vec![];

        for _ in 0..nclasses {
            targets_bv.push(BitVec::from_elem(ntransactions, false))
        }

        for (idx, class) in target.iter().enumerate() {
            targets_bv[*class].set(idx, true)
        }

        Ok(Data { filename, ntransactions, nattributes, nclasses, data: inputs, target: targets_bv, chunked: false })

    }

}
