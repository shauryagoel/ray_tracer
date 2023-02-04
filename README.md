# Rust Ray Tracer

This repository contains rust code for the book "Ray Tracer Challenge by Jamis Buck".

It is not meant to be an optimised version of the ray tracer. I followed this book to learn the Rust programming language. I have made sure to replicate the various logic and tests presented throughout the book. I may be wrong in some cases, but, I will try to keep the code well documented and easy to read.

## Project structure
- src/ : contains the code for various data types like tuple, vector, projectile etc and unit tests.
    - bin/ : contains end of the chapter exercises

## Testing
All the unit tests are present in the each of the source code files. You can run all the tests using the command-
```
cargo test
```

## Building the binaries
You have to build each binary in the src/bin directory separately.
```
cargo run --bin <binary file name without extension>
```

