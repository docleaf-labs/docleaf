# Happy to switch this to a Makefile if that is more approachable

build:
	maturin develop
	make -C examples/sphinx clean
	make -C examples/sphinx pseudoxml
	make -C examples/sphinx html

wheel:
	maturin build --release
