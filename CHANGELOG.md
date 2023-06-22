# Docleaf Changelog

## Version 0.8.0 - 2023-06-22

### Fixed 

- HTML entities are escaped in embedded restructured text blocks which allows restructured texted hyperlinks to be
  processed correctly.

## Version 0.7.0 - 2023-06-22

### Fixed 

- Fixed handling of args string values for member variables. Docleaf uses the args string if it is detected as part
  of a function pointer member variable otherwise it does not attempt to pass it on to the Sphinx C or C++ domain as
  it might be long and complex and fail the parsing in the domain code.
- Fixed namespacing indicator in text rendering of member variable types in C when passing to Sphinx domains.

## Version 0.6.0 - 2023-06-20

### Fixed 

- Construction of various docutils nodes, including `literal`, which derive from docutils `TextElement` base class.

## Version 0.5.0 - 2023-06-18

### Fixed

- Fixed Doxygen-based cross-references between restructured text source files.

## Version 0.4.0 - 2023-06-14

### Added

- Support for `sphinx.ext.linkcode` extension with the `docleaf.doxygen.GitHubLinkResolver` helper.
- Additional `pyproject.toml` metadata.

## Version 0.3.0 - 2023-06-10

Alpha release of software for Linux, Windows and MacOS.
