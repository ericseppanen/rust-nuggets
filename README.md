# Rust Nuggets

This is a collection of small bits of Rust code.  Each file demonstrates one concept as clearly as possible.

In the `errors/examples` directory, you will find Rust examples that won't compile, and produce clean error messages to demonstrate a particular concept: mutability, lifetimes, etc. To see one example by itself, run e.g. `cargo check --example immutable_ref`

You can see an HTML version of the error examples [here](https://raw.githack.com/ericseppanen/rust-nuggets/master/html/errors.html).

### What's this for?

These examples are meant to be used to teach people about Rust.  Specifically, if you need a quick presentation slide demonstrating one concept, but don't want to get tripped up by including other things (e.g. generic values, ? operator, traits) that you haven't introduced yet.

In the case of the compiler errors, they're meant to emit error messages that can easily be pasted into a slide, and make sense even if you don't show the source code.

### Contributing

If you have a new example, please follow the existing style:
- A comment at the top of each example explaining what's being shown.
- All code should be free of unnecessary warnings.
- All code should be formatted using the default rustfmt style.

Each new error example should be in a standalone file, with a corresponding target in `Cargo.toml`.

### License

All examples are in the public domain.
Use or modify them any way you like.  No credit required.
