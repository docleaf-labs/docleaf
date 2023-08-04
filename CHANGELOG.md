# Docleaf Changelog

## Version 0.8.3 - 2023-08-04

### Fixed

- Fixed handling of `includes` XML tags that do not have a `refid` attribute. This occurs
  for `includes` that point to files that Doxygen is not aware of or hasn't generated an
  entry for.

## Version 0.8.2 - 2023-07-13

### Added

- Added third-party licenses file to the distributed Python wheels to include the licenses for the Rust crates used
  in the Rust part of the this project.

### Fixed

- Added `objtype` and `domain` attributes to all directly created `desc` nodes to resolve
  [#3](https://github.com/docleaf-labs/docleaf/issues/3). The `domain` value falls back to `cpp` if no explicit domain
  is detected from the Doxygen XML files or a setting like `docleaf_domain_by_extension`.

## Version 0.8.1 - 2023-07-03

### Changed

- Changed the Sphinx dependency to have no upper bound on the version number. It might not work with all future
  versions of Sphinx but we shouldn't be unnecessarily restrictive.

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
