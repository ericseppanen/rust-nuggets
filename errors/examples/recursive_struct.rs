#![allow(dead_code, unused_variables)]
//
// This is an example of a recursive struct that inlines its child node.
// It generates an error because the struct size would be infinite.
// The compiler will suggest the right fix: Box<Node>

struct Node {
    child: Node,
}
