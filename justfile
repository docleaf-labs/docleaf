# Happy to switch this to a Makefile if that is more approachable

build:
	maturin develop
	make -C documentation clean
	make -C documentation html
	make -C documentation pseudoxml

compound-elements:
	git checkout rust/src/doxygen/compound/elements.rs
	python3 rust/generate.py examples/nutshell/xml/compound.xsd rust/src/doxygen/compound/elements.rs
	rustfmt rust/src/doxygen/compound/elements.rs
	# cargo lbuild