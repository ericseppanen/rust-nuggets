#![allow(dead_code, unused_variables)]
//
// This file contains cleaned-up versions of types from the standard library.
//

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    None,
    Some(T),
}

enum Alignment {
    Left,
    Right,
    Center,
}

enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

// Examples that demonstrate the Result type
mod result_examples {
    use std::error::Error;
    use std::fs;

    // A placeholder for function calls I don't want to explain.
    fn do_something<T>(t: T) {}

    // A placeholder for your error handling library of choice.
    type SomeError = Box<dyn Error>;

    // My first example for people on how Result works.
    // I usually just paste the function body.
    fn first(filename: &str) {
        // fs::read returns Result<Vec<u8>, io::Error>
        let read_result = fs::read(filename);
        match read_result {
            Ok(file_data) => do_something(file_data),
            Err(e) => { /* ... */ }
        }
    }

    // Show how `first` can be improved using '?'
    // I usually just paste the function body.
    fn second(filename: &str) -> Result<(), SomeError> {
        // if Result::Err, return the Err.
        // read_result will contain Vec<u8>
        let file_data = fs::read(filename)?;
        do_something(file_data);

        Ok(())
    }
}
