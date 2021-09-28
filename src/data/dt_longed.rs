use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use bit_vec::BitVec;
use substring::Substring;

pub struct DataLong {
    pub filename: String,
    pub ntransactions: usize,
    pub nattributes: usize,
    pub nclasses: usize,
    pub data: Vec<Vec<u64>>,
    pub target: Vec<Vec<u64>>,
    chunked: bool // Update to use chunks
}


impl DataLong {
    // TODO: add comments for readability
    pub fn new(filename: String) -> Result<DataLong, Error> {
        let input = File::open(&filename)?; //Error Handling for missing filename

        let buffered = BufReader::new(input); // Buffer for the file

        let data_lines: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();

        DataLong::data_chuncked(data_lines, filename)
    }



    fn data_chuncked(data:Vec<String>, filename:String) -> Result<DataLong, Error> {
        let nattributes = data[0].split_ascii_whitespace().collect::<Vec<&str>>().len() - 1;
        let ntransactions = data.len();

        let mut nchunks = 1;
        let mut inputs = vec![];
        if ntransactions > 64 {
            nchunks = match ntransactions % 64 {
                0 => { ntransactions / 64 }
                _ => { (ntransactions / 64) + 1 }
            };
        }
        inputs = vec!["".to_string(); nattributes];
        let mut target = vec![];

        for (i, line) in data.iter().enumerate() {
            let line = line.split_ascii_whitespace().collect::<Vec<&str>>();
            for (j, l) in line.iter().enumerate() {
                match j {
                    0 => { target.push(l.parse::<usize>().unwrap()) }
                    _ => {
                        inputs[(j - 1)].extend(l.chars());
                    }
                }
            }
        }

        let mut final_inputs = vec![vec![]; nattributes];

        for att in 0..nattributes{
            let attrib_str  =  &mut inputs[att].as_str();

            for i in (0..ntransactions).rev().step_by(64){
                let j = (i).saturating_sub(63);
                let mut a =  attrib_str.substring(j, i+1);
                final_inputs[att].push(<u64>:: from_str_radix(&*a.chars().rev().collect::<String>(), 2).unwrap())

            }

        }



        let nclasses = target.iter().collect::<HashSet<_>>().len();

        let mut targets_bv = vec![];
        for class in 0..nclasses {
            targets_bv.push(target.iter().map(|x| ((*x == class) as usize).to_string()).collect::<String>());
        }
        let mut final_targets = vec![vec![]; nclasses];
        if nchunks >  1{

            for c in 0..nclasses {
                let class_str  =  &mut targets_bv[c].as_str();
                for i in (0..ntransactions).rev().step_by(64){
                    let j = (i).saturating_sub(63);
                    let mut a =  class_str.substring(j, i+1);
                    final_targets[c].push(<u64>:: from_str_radix(&*a.chars().rev().collect::<String>(), 2).unwrap())

                }
            }

        }


        // for (idx, class) in target.iter().enumerate() {
        //     targets_bv[*class][idx/64][idx%64] =  1;
        //
        // }

        Ok(DataLong { filename, ntransactions, nattributes, nclasses, data: final_inputs, target: final_targets, chunked: true })



    }



}
