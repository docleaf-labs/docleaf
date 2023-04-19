from pathlib import Path
import sys
import os

# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "Breathe Plus"
copyright = "2023, Breathe Team"
author = "Breathe Team"
release = "0.0.0"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = ["breathe_plus.doxygen"]

templates_path = ["_templates"]
exclude_patterns = []

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "alabaster"
html_static_path = ["_static"]

# -- Options for Breathe
breathe_projects = {
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
    "text-formatting": "../../code/text-formatting/xml/",
    "urls": "../../code/urls/xml/",
    "verbatim": "../../code/verbatim/xml/",
    "xrefsect": "../../code/xrefsect/xml/",
}

breathe_default_project = "nutshell"

breathe_skip_doxygen_xml_nodes = []
