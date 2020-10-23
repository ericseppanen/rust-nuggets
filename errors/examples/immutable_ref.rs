// This is an example of trying to modify something behind an
// immutable reference.
//
// Note: this could have been `fn observe(x: Vec<u8>)` but then the error
// message refers to `*x`, which seems like an unnecessary thing to have
// to explain.

struct Thing {
    x: Vec<u8>,
}

fn observe(thing: &Thing) {
    thing.x[0] = 1;
}
