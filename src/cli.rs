use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[derive(Serialize, Deserialize)]
pub struct Cli {
    /// Sets a custom config file
    #[clap(short, long, value_name = "CONFIG_FILE")]
    pub(crate) config: Option<PathBuf>,

    /// Sets the dataset file
    #[clap(short, long, value_name = "DATASET_FILE")]
    pub(crate) input: Option<String>,

    /// If given set the output path for the generated tree
    #[clap(short, long, value_name = "JSON_TREE_FILE")]
    pub(crate) output: Option<String>,

    /// Sets the minimum support for the itemsets
    #[clap(short, long, value_name = "UNSIGNED_INTEGER")]
    pub(crate) support: Option<u64>,

    /// Sets the tree max depth
    #[clap(short, long, value_name = "UNSIGNED_INTEGER")]
    pub(crate) depth: Option<u64>,

    /// Sets the maximum error allowed
    #[clap(short, long, value_name = "UNSIGNED_FLOAT", default_value_t = <f64>::MAX)]
    #[serde(default = "default_error")]
    pub(crate) error: f64,

    /// Sets the tome between each log of error. The default value is for no log
    #[clap(short, long, value_name = "UNSIGNED_FLOAT", default_value_t = -1)]
    #[serde(default = "default_log_error")]
    pub(crate) log_error_time: i32,

    /// Sets the execution time limit in seconds
    #[clap(short, long, value_name = "UNSIGNED_FLOAT", default_value_t = 0.)]
    #[serde(default = "default_timeout")]
    pub(crate) timeout: f64,

    /// Allow the use of the information gain heuristic
    #[clap(short, long)]
    #[serde(default = "default_bool")]
    pub(crate) use_information_gain: bool,

    /// Allows the use of the discrepancy search
    #[clap(short, long)]
    #[serde(default = "default_bool")]
    pub(crate) allow_discrepancy: bool,

    #[clap(long)]
    #[serde(default = "default_option")]
    pub(crate) discrepancy_limit: Option<usize>,

    #[clap(long)]
    #[serde(default = "default_option")]
    pub(crate) recursion_limit: Option<usize>,
}

fn default_error() -> f64 {
    <f64>::MAX
}

fn default_log_error() -> i32 {
    -1
}

fn default_timeout() -> f64 {
    0.
}

fn default_bool() -> bool {
    false
}

fn default_option() -> Option<usize> {
    None
}
