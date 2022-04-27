# Rust Bindings Demo

An example of using Rust bindings.

This typically involves some specialized use of Rust's type system.

A C module is contained in the `cmodule` directory.
`build.rs` compiles this C library and generates Rust bindings from the C header file.

Various Rust binaries in `src/bin/` call into this API.

Examples are presented as "tests" in binaries.

## Build

Build all the tests:

```sh
cargo build --tests
```

To check the tests without actually compiling:

```sh
cargo check --tests
```

## Run

Run all tests using:

```sh
cargo test
```

Run tests in a specific binary using:

```sh
cargo test --bin const_char
```

To see what binaries are available, look for Rust files in `src/bin/` or run:

```sh
cargo test --bin
```
