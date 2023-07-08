import docleaf.doxygen

# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "Docleaf"
copyright = "2023, Docleaf Team"
author = "Docleaf Team"
release = "0.0.0"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "docleaf.doxygen",
    "sphinx.ext.linkcode",
]

templates_path = ["_templates"]
exclude_patterns = []

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "alabaster"
html_static_path = ["_static"]

# -- Options for docleaf
docleaf_projects = {
    "class_methods": {
        "root": "../code/class_methods/",
        "xml": "../code/class_methods/xml/",
    },
    "classes": {
        "root": "../code/classes/",
        "xml": "../code/classes/xml/",
    },
    "enums": {"root": "../code/enums/", "xml": "../code/enums/xml/"},
    "enums-c": {"root": "../code/enums-c/", "xml": "../code/enums-c/xml/"},
    "functions": {"root": "../code/functions/", "xml": "../code/functions/xml/"},
    "groups": {"root": "../code/groups/", "xml": "../code/groups/xml/"},
    "groups-c": {"root": "../code/groups-c/", "xml": "../code/groups-c/xml/"},
    "html-only": {"root": "../code/html-only/", "xml": "../code/html-only/xml/"},
    "lists": {"root": "../code/lists/", "xml": "../code/lists/xml/"},
    "notes": {"root": "../code/notes/", "xml": "../code/notes/xml/"},
    "nutshell": {"root": "../code/nutshell/", "xml": "../code/nutshell/xml/"},
    "paragraphs": {"root": "../code/paragraphs/", "xml": "../code/paragraphs/xml/"},
    "preformatted": {
        "root": "../code/preformatted/",
        "xml": "../code/preformatted/xml/",
    },
    "program-listings": {
        "root": "../code/program-listings/",
        "xml": "../code/program-listings/xml/",
    },
    "references": {"root": "../code/references/", "xml": "../code/references/xml/"},
    "structs": {"root": "../code/structs/", "xml": "../code/structs/xml/"},
    "structs-c": {"root": "../code/structs-c/", "xml": "../code/structs-c/xml/"},
    "tables": {"root": "../code/tables/", "xml": "../code/tables/xml/"},
    "text-formatting": {
        "root": "../code/text-formatting/",
        "xml": "../code/text-formatting/xml/",
    },
    "urls": {"root": "../code/urls/", "xml": "../code/urls/xml/"},
    "verbatim": {"root": "../code/verbatim/", "xml": "../code/verbatim/xml/"},
    "xrefsect": {"root": "../code/xrefsect/", "xml": "../code/xrefsect/xml/"},
}

docleaf_default_project = "nutshell"
docleaf_domain_by_extension = {"hpp": "cpp", "h": "c"}

docleaf_skip_doxygen_xml_nodes = []
docleaf_doxygen_skip = ["members:all_caps"]

linkcode_resolve = docleaf.doxygen.GitHubLinkResolver(
    root="../../../", user="docleaf-labs", repo="docleaf", branch="main"
)
