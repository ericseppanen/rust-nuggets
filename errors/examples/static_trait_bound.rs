#![allow(dead_code, unused_variables)]
//
// This is an example of a generic trait that requires a 'static trait bound.

use std::io::Read;
use std::thread;

pub fn read_in_background<T: Read + Send>(mut f: T) {
    thread::spawn(move || {
        let mut buf = Vec::<u8>::new();
        if let Ok(count) = f.read_to_end(&mut buf) {
            println!("read {} bytes from file.", count);
        }
    });
}
