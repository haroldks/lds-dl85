### DL8.5 Implementation on Rust


#### TODO Road Map

- [x] Runnable Implementation
- [ ] Implement early stop with timeout
  - [x] Implement lazy stop using a timer (Will not directly stop but break each time)
  - [ ] Check if it can be done using a thread
- [ ] Sort successor by information gain or other
- [ ] Move tree generation methods from the main file
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
- [ ] Implement error reading at fixed time
  - [ ] Implement cache cloning in a file at fixed time in another thread
  - [ ] Use the saved thread and a test set to see the final error ?
- [ ] Implement warm start
- [ ] Create a main file to use from command line
- [ ] Visualization of the Tree

### Topics

- [ ] Using DL85 With missing data
- [ ] Sliding Windows
- [ ] Knowledge distillation
