from pathlib import Path
import sys
import os

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

extensions = ["docleaf.doxygen"]

templates_path = ["_templates"]
exclude_patterns = []

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "alabaster"
html_static_path = ["_static"]

# -- Options for docleaf
docleaf_projects = {
    "class_methods": "../../code/class_methods/xml/",
    "enums": "../../code/enums/xml/",
    "functions": "../../code/functions/xml/",
    "groups": "../../code/groups/xml/",
    "html-only": "../../code/html-only/xml/",
    "lists": "../../code/lists/xml/",
    "nutshell": "../../code/nutshell/xml/",
    "paragraphs": "../../code/paragraphs/xml/",
    "preformatted": "../../code/preformatted/xml/",
    "program-listings": "../../code/program-listings/xml/",
    "references": "../../code/references/xml/",
    "structs": "../../code/structs/xml/",
    "tables": "../../code/tables/xml/",
    "text-formatting": "../../code/text-formatting/xml/",
    "urls": "../../code/urls/xml/",
    "verbatim": "../../code/verbatim/xml/",
    "xrefsect": "../../code/xrefsect/xml/",
}

docleaf_default_project = "nutshell"
docleaf_domain_by_extension = { "hpp": "cpp", "h": "c" }

docleaf_skip_doxygen_xml_nodes = []
