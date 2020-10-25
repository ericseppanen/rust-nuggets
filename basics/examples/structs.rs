#![allow(dead_code)]
//
// This file contains useful examples of structs.
//
// See standard.rs for some more examples from the Rust standard library.
//

// An example of a simple struct

struct KitchenSink {
    name: String,
    num: u64,
    enabled: bool,
}

// An example of a struct with child elements
// This is a good example of Rust zero-cost abstractions:
// Option<Box<T>> is the same size as a pointer.
// This struct is 24 bytes on a 64-bit architecture.

struct Node {
    value: u64,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}
