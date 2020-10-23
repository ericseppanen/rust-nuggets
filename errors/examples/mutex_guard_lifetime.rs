// This is an example of lifetimes in the context of Mutex guards.
// A Mutex guard is used to create a reference to some inner element.
// That reference tries to outlive the guard, resulting in a compiler error.

use std::sync::Mutex;

struct Thing {
    x: u8,
}

fn stuff() {
    let thing = Thing { x: 7 };
    let wrap = Mutex::new(thing);
    let x_ref: &u8;
    {
        let guard = wrap.lock().unwrap();
        x_ref = &guard.x;
    }
    println!("x was {}", x_ref);
}
