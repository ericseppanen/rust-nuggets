#![allow(dead_code, unused_variables)]
//
// This is an example of a match statement with patterns that don't
// cover all possible inputs.

enum Color {
    Blue,
    Yellow,
    Red,
}

fn examine(color: Color) {
    match color {
        Color::Blue => {},
        Color::Yellow => {},
    }
}
