# Exercise 5: Pointers and Structures in Rust

In `linkedlist.rs`, complete the function `has_cycle()` to implement the following algorithm for checking if a Linked List has a cycle:

1. Start with two pointers at the head of the list, `fast_ptr` and `slow_ptr`
2. Move `fast_ptr` forward by 2 nodes, if possible. If not (when there is a null pointer), that means we have reached the end of the list and the list is acyclic.
3. Move `slow_ptr` forward by 1 node.
4. If `fast_ptr` and `slow_ptr` ever point to the same node, then the list is cyclic. Otherwise, go back to step 2.

## Running your code

Execute the following commands to run the tests for your code.

```/bin/bash
$ rustc linkedlist
$ ./linkedlist
```
