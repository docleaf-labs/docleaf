# Happy to switch this to a Makefile if that is more approachable

build:
	maturin develop
	make -C documentation clean
	make -C documentation html
	make -C documentation pseudoxml

compound-elements:
	git checkout src/doxygen/compound/elements.rs
	python3 src/generate.py examples/nutshell/xml/compound.xsd src/doxygen/compound/elements.rs
	rustfmt src/doxygen/compound/elements.rs
	# cargo lbuild