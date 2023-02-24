
# Breathe Plus

This is an exploratory project for what a new version of Breathe might look like if it is primarily written in Rust.
This is to improve performance and reduce the memory usage.

It is also an experiment in licensing as to make the project more sustainable the idea is to make it is source 
available (rather than open source) and free for open source projects but to require a paid license for non-open source
work.

## Run

Requires a Rust & Cargo installation.

- https://www.rust-lang.org/tools/install

And Poetry for Python package management.

- https://python-poetry.org/docs/#installation

```
poetry install
poetry shell
./build.sh
make -C documentation html
firefox documentation/build/html/index.html
```

## License

Licensed under the [Parity Public License](./LICENSE.md)
