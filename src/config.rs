use float_cmp::{ApproxEq, F64Margin};

pub struct Config {
    pub filename: String,
    pub min_support : u64,
    pub max_depth : u64,
    pub max_error : f64,
    pub time_limit : f64,
    pub error_save_time: i32
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 7 {
            return Err("You should give 6 arguments : 'filename' 'min_support' 'max_depth' 'max_error (0 when using infinity)' 'time_limit (0 when no time limit)' 'error_save_time (0 when no saving is allowed)' ");
        }

        let filename = args[1].clone();

        let min_support = args[2].clone().parse::<u64>().unwrap();
        if min_support <= 1 {
           return  Err("Invalid min support. Min support should be greater than 0.");
        }

        let max_depth = args[3].clone().parse::<u64>().unwrap();
        if max_depth <= 1 {
            return Err("Invalid max depth. Max Depth should be greater than 1.");
        }

        let mut max_error = args[4].clone().parse::<f64>().unwrap();
        if max_error.approx_eq(0., F64Margin {ulps: 2, epsilon: 0.0}) {
            println!("Max error lower or equal to zero. Using infinity as the upper bound.");
            max_error = <f64>::MAX;
        }

        let mut time_limit = args[5].clone().parse::<f64>().unwrap();
        if time_limit.approx_eq(0., F64Margin {ulps: 2, epsilon: 0.0}) {
            println!("Time lower or equal to zero. Using no time limit option.");
            time_limit = 0.;
        }

        let mut error_save_time = args[6].clone().parse::<i32>().unwrap();
        if error_save_time <= 0 {
            println!("Error save time lower or equal to zero. Using no save error time option.");
            error_save_time = -1;
        }
        Ok(Config { filename, min_support, max_depth, max_error, time_limit, error_save_time })
    }
}
