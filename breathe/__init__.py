from typing import List

from docutils.parsers.rst import Directive, directives
from docutils import nodes
from docutils.nodes import Node
from docutils.parsers.rst.directives import unchanged_required, unchanged, flag
from sphinx.application import Sphinx
import sphinx.addnodes

from . import backend

__version__ = "0.0.0"

node_lookup = {
    "container": nodes.container,
    "desc": sphinx.addnodes.desc,
    "desc_name": sphinx.addnodes.desc_name,
    "desc_content": sphinx.addnodes.desc_content,
    "desc_parameter": sphinx.addnodes.desc_parameter,
    "desc_parameterlist": sphinx.addnodes.desc_parameterlist,
    "desc_signature": sphinx.addnodes.desc_signature,
    "desc_signature_line": sphinx.addnodes.desc_signature_line,
    "desc_sig_keyword": sphinx.addnodes.desc_sig_keyword,
    "desc_sig_space": sphinx.addnodes.desc_sig_space,
    "desc_sig_name": sphinx.addnodes.desc_sig_name,
    "paragraph": nodes.paragraph,
    "index": sphinx.addnodes.index,
    "rubric": nodes.rubric,
}


def render_node_list(node_list):
    return [render_node(node) for node in node_list]


def render_node(node):
    if node.type == "text":
        return nodes.Text(node.text)

    node_builder = node_lookup[node.type]
    children = render_node_list(node.children)

    if node.call_as == "source-text":
        return node_builder("", "", *children)
    elif node.call_as == "source":
        return node_builder("", *children)
    elif node.call_as == "args":
        return node_builder(*children)
    else:
        raise Exception("Call As not implemented" + node.call_as)


class ClassDirective(Directive):
    has_content = True
    required_arguments = 1
    optional_arguments = 0
    final_argument_whitespace = True
    option_spec = {
        "project": directives.unchanged,
    }

    def run(self) -> List[Node]:
        name = self.arguments[0]
        project = self.options["project"] or self.app.config.breathe_default_project
        path = self.app.config.breathe_projects[project]
        node_list = backend.render_class(name, path)
        return render_node_list(node_list)


def setup(app: Sphinx):
    ClassDirective.app = app
    app.add_directive("doxygenclass", ClassDirective)
    app.add_config_value("breathe_projects", {}, "env")
    app.add_config_value("breathe_default_project", None, "env")
    return {"version": __version__}
