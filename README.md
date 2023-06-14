<h1 align="center">
  Docleaf
</h1>

<p align="center">
   Your technical docs, beautifully integrated
</p>

Docleaf smoothly integrates your technical and long-form documentation. It is a Sphinx extension which reads Doxygen
XML output and formats the information seamlessly with your user documentation.

## License

Docleaf is licensed under the [Parity Public License](./LICENSE.md). The Parity license allows permissive use of 
Docleaf to help document open source projects. If you have a closed source project that you would like to document with
Docleaf then you must purchase a commercial license.

For further information please email: [support@docleaf.io](mailto:support@docleaf.io)

## Features

- Custom directives allowing you to target various parts of C and C++ code bases.
- Integration with Sphinx C and C++ domains to support easily linking to your generated output.
- Hash-based content checks, as well as timestamp checks, to minimize incremental build times after a Doxygen run.
- Integration with the `sphinx.ext.linkcode` extension to support links to source code locations in a GitHub
  repository.

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

Configure the extension to know where your source code is stored and the Doxygen XML output has been generated for
your project. Optionally set the default project:

```python
docleaf_projects = {
  "my_project": {
    "root": "../src",
    "xml": "../doxygen/xml"
  }

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

All directives take a `:project:` option to specify the project to use from your `conf.py` if you don't want to use
the default project.

### Settings

- `docleaf_projects` 

  A Python dictionary mapping each project name to the folders where its source code and Doxygen XML output are stored.

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

### Integration with `sphinx.ext.linkcode`

Docleaf can integrate with the `sphinx.ext.linkcode` extension in order to add `[source]` links next to various
supported entries in your documentation. Linking to GitHub based repositories is supported.

In order to use it, add the `sphinx.ext.linkcode` extension to the `extensions` list in your Sphinx `conf.py` and use
the `docleaf.doxygen.GitHubLinkResolver` with appropriate parameters for your repository.

```python
extensions = [
  "docleaf.doxygen",
  "sphinx.ext.linkcode",
  ]

linkcode_resolve = docleaf.doxygen.GitHubLinkResolver(
    root="../../../", user="docleaf-labs", repo="docleaf", branch="main"
)
```

Where:
- `root` is the relative path to the root of your repository.
- `user` is the user or organisation name for your GitHub repository.
- `repo` is the name of your GitHub repository.
- `tag` is the git tag that you would like the generated link URLs to target.
- `branch` is the git branch that you would like the generated link URLs to target.
- `commit` is the git commit SHA that you would like the generated link URLs to target.

Only one of `tag`, `branch` and `commit` is necessary.


## Performance

When doing a clean build of the Zephyr RTOS documentation suite, Docleaf is 2.1x faster than Breathe.

```
Benchmark: docleaf
  Time (mean ± σ):     180.383 s ±  3.213 s    [User: 448.242 s, System: 12.908 s]
  Range (min … max):   175.695 s … 185.187 s    10 runs
```

```
Benchmark: breathe
  Time (mean ± σ):     389.658 s ±  5.271 s    [User: 1839.366 s, System: 24.895 s]
  Range (min … max):   379.093 s … 394.315 s    10 runs
```

## History

Docleaf is written and maintained by the creator of the [Breathe](https://github.com/breathe-doc/breathe) project.
It was created to resolve some of the performance and memory consumption issues with Breathe by rewriting the code
base to use Rust. The user experience is designed to match and improve on Breathe.
