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
- [ ] Use clap crate to allow default configuration for the algorithm
- [ ] Implement Generic type and common trait for its for the DL85 struct
- [ ] Sort successor by information gain or other
- [ ] Fix Useless tree split when the error is done on the same class in each split
- [ ] Test all implementations to see the fastest (Chunked, Not chunked)
- [ ] Check Memory usage with C++ implementation
- [ ] Merge tree node when fake leaves is created ?
- [ ] Fix warnings
- [ ] Replace Vec to slices for the ItsOps implementations ?
- [ ] Implement lower bound to reduce the space search
- [ ] Implement the valid words and limit trick to reduce computation
- [ ] Allow DL8.5 main class to use generic types for dataset and its_op
- [ ] Clean DL8.5 and move its_op and data outside it
- [ ] Implement warm start
- [ ] Create a main file to use from command line
- [ ] Visualization of the Tree

### Topics

- [ ] Using DL85 With missing data
- [ ] Sliding Windows
- [ ] Knowledge distillation
