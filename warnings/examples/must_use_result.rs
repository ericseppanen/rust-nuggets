#![allow(dead_code, unused_variables)]

// This is an example of the warning message that appears if you don't
// use the Result that's the return value of a function.

use std::fs;

fn do_rename() {
    fs::rename("old_file", "new_file");
}
