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

extensions = ["breathe_plus"]

templates_path = ["_templates"]
exclude_patterns = []

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "alabaster"
html_static_path = ["_static"]

# -- Options for Breathe
breathe_projects = {
    "nutshell": "../../examples/nutshell/xml/",
    "class_methods": "../../examples/class_methods/xml/",
    "references": "../../examples/references/xml/",
    "paragraphs": "../../examples/paragraphs/xml/",
    "functions": "../../examples/functions/xml/",
    "structs": "../../examples/structs/xml/",
    "enums": "../../examples/enums/xml/",
    "groups": "../../examples/groups/xml/",
    "text-formatting": "../../examples/text-formatting/xml/",
}

breathe_default_project = "nutshell"
