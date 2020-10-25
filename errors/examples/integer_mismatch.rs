#![allow(dead_code, unused_variables)]
//
// This is an example of Rust's strict type checking.
// Even for basic types like u8 and u32, the compiler won't automatically
// assume it knows what you intend.
// `b as u32` would work, as would u32::from(b)
// As of Rust 1.47.0, this emits two errors (mismatched types + cannot add u8 to u32)
// I would prefer examples that emit only one error, but I don't know how to
// achieve that here.

fn math(a: u32, b: u8) {
    let x: u32 = a + b;
}
