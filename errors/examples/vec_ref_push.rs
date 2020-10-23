// This is an example of conflicting mutable and immutable references.
// The code takes a reference to the first element in a Vec, and then
// tries to modify the Vec by pushing an additional element. The
// reference is used later, which is not possible because it requires
// an immutable reference and mutable reference to exist at the same time.
//
// This is a common error in C++.  Because the `push` could cause the
// Vec to be moved in memory, the earlier reference could be invalidated.

struct Thing {
    x: Vec<u8>,
}

fn change(thing: &mut Thing) {
    let y: &u8 = &thing.x[0];

    thing.x.push(1);

    println!("y is {}", y);
}
