
# Docleaf

Docleaf smoothly integrates your technical and long-form documentation. It is a Sphinx extension which reads Doxygen
XML output and formats the information seamlessly with your user documentation.

## License

Docleaf is licensed under the [Parity Public License](./LICENSE.md). The Parity license allows permissive use of 
Docleaf to help document open source projects. If you have a closed source project that you would like to document with
Docleaf then you must purchase a commercial license. For further information please email
[support@docleaf.io](mailto:support@docleaf.io).

## Installation

Docleaf can be installed from [PyPI](https://pypi.org/project/docleaf/):

```
pip install docleaf
```

## Usage

Include `docleaf.doxygen` as an extension in your Sphinx `conf.py` file:

```python
extensions = ["docleaf.doxygen"]
```

Configure the extenion to know where the Doxygen XML output has been generated for your project and optionally set the
default project:

```python
docleaf_projects = {
  "my_project": "../doxygen/xml"
}
docleaf_default_project = "my_project"
```

The use the provided directives in your reStructuredText files:

```rst
.. doxygenstruct:: ExampleStruct
```

See below for available directives.

### Directives

Generate documentation for a C++ class.

```rst
.. doxygenclass:: ClassName
```

Generate documentation for a C or C++ struct.

```rst
.. doxygenstruct:: StructName
```

Generate documentation for a C or C++ function.

```rst
.. doxygenfunction:: function_name
```

Generate documentation for a C or C++ enum.

```rst
.. doxygenenum:: EnumName
```

Generate documentation for specific group as specified within your Doxygen set up and code comments.

```rst
.. doxygengroup:: group_name
```

### Settings

- `docleaf_projects` 

  A Python dictionary mapping each project name to the folder where its Doxygen XML output is stored.

- `docleaf_default_project`

  The default project to use when none is specified on the directive itself.
  
- `docleaf_domain_by_extension`

  A Python dictionary mapping from file extension to Sphinx domain. Docleaf uses Doxygen's language classifications
  where possible but for optimal control of how source files are classified it is useful to use this setting. For
  example:

  ```python
  docleaf_domain_by_extension = {"hpp": "cpp", "h": "c"}
  ```

  Will make sure that all files that end in `.hpp` will be considered as C++ files and processed using the C++ Sphinx
  domain whilst files that end in `.h` will be considered C files and processed with the C Sphinx domain.

- `docleaf_doxygen_skip`

  A list of instructions describing any parts of the Doxygen XML to skip when generating the output documentation.
  Supported entries are:

  - `members:all_caps` - Skips any function or variable members (as defined as a 'memberdef' by Doxygen) which have 
    names which are all capital letters and underscores. This is to allow users to filter our unprocessed C/C++ macros
    if desirable.
  - `xml-nodes:<node name>` - Skips reading and process of the given XML node and its children in the Doxygen XML 
    output. Support is limited to the `htmlonly` node.

## History

Docleaf is written and maintained by the creator of the [Breathe](https://github.com/breathe-doc/breathe) project.
It was created to resolve some of the performance and memory consumption issues with Breathe by re-writing the code
base to use Rust. The user experience is designed to match Breathe.
