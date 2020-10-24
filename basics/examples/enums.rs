#![allow(dead_code)]
//
// This file contains useful examples of enums.
//

// An example of an enum used without associated data.

enum Color {
    Blue,
    Yellow,
    Red,
}

fn to_wavelength(color: Color) -> u32 {
    match color {
        Color::Blue => 450,
        Color::Yellow => 575,
        Color::Red => 700,
    }
}

// An enum that demonstrates different kinds of payloads.
// Note that one variant contains no associated data, another
// has encapsulated data, while another has some heap-allocated
// data.

enum Possible<T> {
    None,
    One(T),
    Many(Vec<T>),
}

fn inspect_container(c: &Possible<u8>) {
    match c {
        Possible::None => {}
        Possible::One(n) => println!("{}", n),
        Possible::Many(_) => println!("many!"),
    }
}
