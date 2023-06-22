# Happy to switch this to a Makefile if that is more approachable

build:
	cd rust && cargo lcheck
	maturin develop
	make -C examples/sphinx clean
	make -C examples/sphinx pseudoxml
	make -C examples/sphinx html

wheel:
	rm -f rust/target/wheels/*
	maturin build --release
