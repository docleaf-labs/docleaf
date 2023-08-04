import os

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
docleaf_projects = {}

# Populate the projects from the 'code' folder
for entry in os.listdir("../../code"):
    docleaf_projects[entry] = {
        "root": f"../code/{entry}/",
        "xml": f"../code/{entry}/xml/",
    }

docleaf_default_project = "nutshell"
docleaf_domain_by_extension = {"hpp": "cpp", "h": "c"}

docleaf_skip_doxygen_xml_nodes = []
docleaf_doxygen_skip = ["members:all_caps"]

linkcode_resolve = docleaf.doxygen.GitHubLinkResolver(
    root="../../../", user="docleaf-labs", repo="docleaf", branch="main"
)
