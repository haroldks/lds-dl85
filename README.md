### DL8.5 Implementation on Rust


#### TODO Road Map

- [x] Runnable Implementation
- [x] Move tree generation methods out of the main file
- [x] Implement a predictor and a metric computer
- [x] Implement early stop with timeout
  - [x] Implement lazy stop using a timer (Will not directly stop but break each time)
  - [x] Check if it can be done using a thread
- [x] Implement error reading at fixed time
- [x] Move parameter from constructor to the run algo
- [x] Change to use environment parameters
- [x] Implement Generic type and common trait for its for the DL85 struct
- [X] Clean DL8.5 and move its_op and data outside it
- [X] Implement warm start
- [X] Sort successor by information gain or other
- [X] Implement Limited Discrepancy Search
- [X] Create a main file to use from command line
- [X] Use clap crate to allow default configuration for the algorithm
- [X] Fix Useless tree split when the error is done on the same class in each split
- [X] Test all implementations to see the fastest (Long is the fastest)
  - [ ] Remove the others
- [ ] Check Memory usage with C++ implementation
- [ ] Fix warnings
- [ ] Replace Vec to slices for the ItsOps implementations ? (Not necessary)
- [ ] Implement lower bound to reduce the space search
- [ ] Implement the valid words and limit trick to reduce computation
- [ ] Implement Depth 2 optimization
- [ ]Reduce memory using by removing clone calls

- [ ] Visualization of the Tree
- [ ] Unit Test implementation

### Topics

- [ ] Using DL85 With missing data
- [ ] Sliding Windows
- [ ] Knowledge distillation
