use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub struct DataLong {
    pub filename: String,
    pub ntransactions: usize,
    pub nattributes: usize,
    pub nclasses: usize,
    pub data: Vec<Vec<u64>>,
    pub target: Vec<Vec<u64>>,
}

#[allow(dead_code)]
impl DataLong {
    // TODO: add comments for readability
    pub fn new(filename: String) -> Result<DataLong, Error> {
        let input = File::open(&filename)?; //Error Handling for missing filename

        let buffered = BufReader::new(input); // Buffer for the file

        let data_lines: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();

        DataLong::data_to_long(data_lines, filename)
    }


    fn data_to_long(mut data: Vec<String>, filename: String) -> Result<DataLong, Error> {
        let nattributes = data[0].split_ascii_whitespace().collect::<Vec<&str>>().len() - 1;
        let ntransactions = data.len();

        let mut nchunks = 1;
        if ntransactions > 64 {
            nchunks = match ntransactions % 64 {
                0 => { ntransactions / 64 }
                _ => { (ntransactions / 64) + 1 }
            };
        }
        let mut inputs = vec![vec![0u64; nchunks]; nattributes];
        let mut target = vec![];

        data.reverse();

        let mut actual_chunk = nchunks - 1;
        let mut counter = 0;


        for line in data.iter() {
            if counter == 64 {
                actual_chunk -= 1;
                counter = 0;
            }
            let line = line.split_ascii_whitespace().collect::<Vec<&str>>();
            for (j, l) in line.iter().enumerate() {
                match j {
                    0 => { target.push(l.parse::<usize>().unwrap()) }
                    _ => {
                        if l == &"1" {
                            inputs[(j - 1)][actual_chunk] = DataLong::bit_to_one(inputs[(j - 1)][actual_chunk], counter as u64)
                        }
                    }
                }
            }
            counter += 1;
        }


        let mut nclasses = target.iter().collect::<HashSet<_>>().len();

        if nclasses < 2 {
            nclasses = 2;
        }

        let mut targets_bv = vec![];

        for _ in 0..nclasses {
            targets_bv.push(vec![0u64; nchunks])
        }
        let tg_len = target.len();
        let mut counter = 0;
        let mut actual_chunk = nchunks - 1;
        for i in 0..tg_len {
            if counter == 64 {
                actual_chunk -= 1;
                counter = 0;
            }
            let class = target[i];
            targets_bv[class][actual_chunk] = DataLong::bit_to_one(targets_bv[class][actual_chunk], counter as u64);
            counter += 1;
        }
        // for (idx, class) in target.iter().enumerate() {
        //     targets_bv[*class][idx / 64] = DataLong::bit_to_one(targets_bv[*class][idx / 64], 63 - (idx % 64) as u64);
        //     if class == 1{
        //         if idx/64 == 0:
        //     }
        // }


        Ok(DataLong { filename, ntransactions, nattributes, nclasses, data: inputs, target: targets_bv })
    }

    fn bit_to_one(original: u64, bit: u64) -> u64 {
        let mask = 1u64 << bit;
        original | mask
    }
}
