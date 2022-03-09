use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use bit_vec::BitVec;

pub struct DataChunked {
    pub filename: String,
    pub ntransactions: usize,
    pub nattributes: usize,
    pub nclasses: usize,
    pub data: Vec<Vec<BitVec>>,
    pub target: Vec<Vec<BitVec>>,
}

#[allow(dead_code)]
impl DataChunked {
    // TODO: add comments for readability
    pub fn new(filename: String) -> Result<DataChunked, Error> {
        let input = File::open(&filename)?; //Error Handling for missing filename

        let buffered = BufReader::new(input); // Buffer for the file

        let data_lines: Vec<String> = buffered.lines().map(|x| x.unwrap()).collect();

        DataChunked::data_chunked(data_lines, filename)
    }

    fn data_chunked(data: Vec<String>, filename: String) -> Result<DataChunked, Error> {
        let nattributes = data[0]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .len()
            - 1;
        let ntransactions = data.len();

        let mut nchunks = 1;
        if ntransactions > 64 {
            nchunks = match ntransactions % 64 {
                0 => ntransactions / 64,
                _ => (ntransactions / 64) + 1,
            };
        }
        let mut inputs = vec![vec![BitVec::from_elem(64, false); nchunks]; nattributes];
        let mut target = vec![];

        for (i, line) in data.iter().enumerate() {
            let line = line.split_ascii_whitespace().collect::<Vec<&str>>();
            for (j, l) in line.iter().enumerate() {
                match j {
                    0 => target.push(l.parse::<usize>().unwrap()),
                    _ => inputs[(j - 1)][i / 64].set(i % 64, l == &"1"),
                }
            }
        }

        let mut nclasses = target.iter().collect::<HashSet<_>>().len();

        let mut targets_bv = vec![];

        if nclasses < 2 {
            nclasses = 2;
        }

        for _ in 0..nclasses {
            targets_bv.push(vec![BitVec::from_elem(64, false); nchunks])
        }

        for (idx, class) in target.iter().enumerate() {
            targets_bv[*class][idx / 64].set(idx % 64, true);
        }

        Ok(DataChunked {
            filename,
            ntransactions,
            nattributes,
            nclasses,
            data: inputs,
            target: targets_bv,
        })
    }
}
