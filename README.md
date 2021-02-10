Rust Protobuf Binding Generator
================================
[Prost](https://github.com/danburkert/prost) is the most commonly-used Rust Protobuf binding generator available. Unfortunately, it:

1. Doesn't come with a `protoc` plugin, which means that it can't be used directly (it must be used within a Rust CLI) and
2. Doesn't generate bindings for Protobuf `service`s

To fix the second issue we use `tonic-build`, which is a wrapper around `prost-build` to generate traits, and to fix the second issue we have this repo which is a tiny CLI wrapped around `tonic-build`.

Usage
-----
Install the CLI to your local machine:
```
cargo install --path .
```
