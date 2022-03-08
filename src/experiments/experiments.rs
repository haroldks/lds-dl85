use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};
use serde_json::to_writer;

use crate::{DataLong, DL85, ItemsetBitvector, ItemsetOpsLong, Trie};

#[derive(Serialize, Deserialize, Clone)]
pub struct TestConfig {
    pub min_support: u64,
    pub max_depth: u64,
    pub max_error: f64,
    pub timeouts: Option<Vec<f64>>,
    #[serde(skip)]
    pub output_folders: [(bool, String); 2],
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    // Comparison error of normal run with discrepancy run on all dataset
    name: String,
    timeout: Vec<f64>,
    dl85_errors: Vec<f64>,
    lds_errors: Vec<f64>,
    pub test_config: Option<TestConfig>,
}

impl Test {
    pub fn new() -> Test {
        Test {
            name: "".parse().unwrap(),
            timeout: vec![],
            dl85_errors: vec![],
            lds_errors: vec![],
            test_config: None,
        }
    }

    pub fn to_json(&self, filename: String) -> Result<(), Error> {
        if let Err(e) = to_writer(&File::create(filename)?, &self) {
            println!("Error While creating the file: {}", e.to_string());
        };
        Ok(())
    }

    pub fn run(&mut self, conf: TestConfig) -> Result<(), Error> {
        self.test_config = Some(conf.clone());

        let timeouts = conf.timeouts.unwrap_or(vec![30., 60., 90.]);
        let output_folders: HashMap<bool, String> = HashMap::from_iter(conf.output_folders);

        for (_, val) in output_folders.iter() {
            let _folder_creation = fs::create_dir_all(val);
            let _folder_creation = match _folder_creation {
                Ok(_) => { println!("Created a directory at the path  {:?}", val) }
                Err(_) => {
                    if let Err(e) = fs::remove_dir_all(val) {
                        println!("Error while removing directories: {}", e.to_string());
                    };
                    if let Err(e) = fs::create_dir_all(val) {
                        println!("Error while creating directories: {}", e.to_string());
                    };
                    println!("Directory already exists at the path {:?}. Recreation", val);
                }
            };
        }


        for use_information_gain in [true, false] {
            let files = fs::read_dir("datasets").unwrap();
            for file in files {
                let file = file?;
                let path = file.path().to_str().unwrap().to_string();

                let filename: Vec<&str> = path.split("/").collect();
                let size = filename.len();
                let dataset_name = filename[size - 1];

                self.name = dataset_name.to_string();

                let mut out = output_folders.get(&use_information_gain).unwrap().clone();
                out.push_str(dataset_name);
                let size = out.len();
                let out = &out[..size - 3];
                let mut out = out.to_string();
                out.push_str("json");

                println!("Actual File: {:?}\n", path);

                for use_discrepancy in [false, true] {
                    println!("timeout {:?}", timeouts);
                    for timeout in &timeouts {
                        println!("Timeout\t:  {}", timeout);
                        println!("Using discrepancy\t:  {}\n", use_discrepancy);

                        let data = DataLong::new(path.clone()).unwrap();
                        let operator = ItemsetOpsLong::new(&data);
                        let mut algo = DL85::new(operator.get_infos());
                        let output = algo.run(conf.min_support, conf.max_depth, conf.max_error, *timeout, -1, use_information_gain, use_discrepancy, false, operator, Trie::new());

                        if use_discrepancy {
                            self.timeout.push(*timeout);
                            self.lds_errors.push(output.0.root.data.node_error);
                        } else {
                            self.dl85_errors.push(output.0.root.data.node_error);
                        }
                    }
                }
                println!("Saved Results to : {}", out);
                if let Err(e) = self.to_json(out) {
                    println!("Error while creating result file : {}", e);
                };
                self.dl85_errors = vec![];
                self.lds_errors = vec![];
                self.timeout = vec![];
            }
        }
        Ok(())
    }
}

//
// fn run_test() -> Result<(), Error> {
//
//     // Read File here and get data set as a list
//     let min_support = 1;
//     let max_depth = 9;
//     //let use_info_gain = true;
//
//     for info_gain in [true, false] {
//         let files = fs::read_dir("datasets").unwrap();
//
//         for file in files {
//             let file = file?;
//             let path = file.path().to_str().unwrap().to_string();
//             let path_clone = path.clone();
//             let filename: Vec<&str> = path_clone.split("/").collect();
//
//             let right_split = &filename[1];
//             let mut out = "output_d9_ig/".to_string();
//             if !info_gain {
//                 out = "output_d9_no_ig/".to_string();
//             }
//             out.push_str(right_split);
//             let size = out.len();
//             let out = &out[..size - 3];
//             let mut out = out.to_string();
//             out.push_str("json");
//
//
//             let mut timeout_vec = vec![];
//             let mut normal_run = vec![];
//             let mut discrepancy_run = vec![];
//             // let mut max_iterations = vec![];
//             // let mut normal_iterations = vec![];
//             // let mut discrepancy_iterations = vec![];
//
//             println!("Actual File: {:?}\n", path);
//             for use_discrepancy in [false, true] {
//
//                 for timeout in [30., 60., 90.] {
//                     println!("Timeout\t:  {}", timeout);
//                     println!("Using discrepancy\t:  {}\n", use_discrepancy);
//                     let data = DataLong::new(path.clone()).unwrap();
//                     let its_op = ItemsetOpsLong::new(&data);
//                     let mut algo = DL85::new(its_op.get_infos());
//                     let output = algo.run(min_support, max_depth, <f64>::MAX, timeout, -1, info_gain, use_discrepancy, false, its_op, Trie::new());
//                     if use_discrepancy {
//                         timeout_vec.push(timeout);
//
//                         discrepancy_run.push(output.0.root.data.node_error);
//                         // discrepancy_iterations.push(output.0.current_iterations);
//                     } else {
//                         // max_iterations.push(output.0.max_iterations);
//                         normal_run.push(output.0.root.data.node_error);
//                         // normal_iterations.push(output.0.current_iterations);
//                     }
//                 }
//             }
//             let infos = TimeoutComp::new(timeout_vec, normal_run, discrepancy_run);
//             println!("File : {}", out);
//             if let Err(e) = infos.to_json(out.to_string()) {
//                 println!("Error while creating json : {}", e);
//             };
//         }
//     }
//     Ok(())
// }
