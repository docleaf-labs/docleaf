#!/bin/bash -e

# Currently using this instead of 'maturin develop' as we can copy the output
# into the breathe module to have it work as a submodule. I'm not sure how to
# achieve that with maturin.

cargo build
cp target/debug/libbackend.so breathe/backend.cpython-38-x86_64-linux-gnu.so
