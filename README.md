
# Breathe Pro

This is an exploratory project for looking at what a new version of Breathe might look like if it is primarily written
in Rust. This is to improve performance and reduce the memory usage.

It is also an experiment in licensing as to make the project more sustainable the idea is to make it is source 
available (rather than open source) and free for open source projects but to require a paid license for non-open source
 work.

## Run

Requires a Rust & Cargo installation.

- https://www.rust-lang.org/tools/install

```
./build.sh
make -C documentation html
firefox documentation/build/html/index.html
```
