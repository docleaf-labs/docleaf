from typing import List

from docutils.parsers.rst import Directive, directives
from docutils import nodes
from docutils.nodes import Node
from docutils.parsers.rst.directives import unchanged_required, unchanged, flag
from sphinx.application import Sphinx
import sphinx.addnodes

from . import backend

__version__ = "0.0.0"


class NodeManager:
    def __init__(self, document):
        self.document = document
        self.lookup = {
            "bullet_list": nodes.bullet_list,
            "container": nodes.container,
            "desc": sphinx.addnodes.desc,
            "desc_content": sphinx.addnodes.desc_content,
            "desc_name": sphinx.addnodes.desc_name,
            "desc_parameter": sphinx.addnodes.desc_parameter,
            "desc_parameterlist": sphinx.addnodes.desc_parameterlist,
            "desc_sig_keyword": sphinx.addnodes.desc_sig_keyword,
            "desc_sig_name": sphinx.addnodes.desc_sig_name,
            "desc_sig_space": sphinx.addnodes.desc_sig_space,
            "desc_signature": sphinx.addnodes.desc_signature,
            "desc_signature_line": sphinx.addnodes.desc_signature_line,
            "emphasis": nodes.emphasis,
            "enumerated_list": nodes.enumerated_list,
            "index": sphinx.addnodes.index,
            "inline": nodes.inline,
            "list_item": nodes.list_item,
            "literal": nodes.literal,
            "literal_block": nodes.literal_block,
            "literal_strong": sphinx.addnodes.literal_strong,
            "paragraph": nodes.paragraph,
            "reference": nodes.reference,
            "rubric": nodes.rubric,
            "strong": nodes.strong,
            "target": self.build_target,
        }

    def get_builder(self, node_type):
        return self.lookup[node_type]

    def build_target(self, key, *children, **attributes):
        # self.document.note_explicit_target(target)
        return nodes.target(key, *children, **attributes)


def render_node_list(node_list, node_manager):
    return [render_node(node, node_manager) for node in node_list]


def render_node(node, node_manager):
    if node.type == "text":
        return nodes.Text(node.text)

    node_builder = node_manager.get_builder(node.type)
    children = render_node_list(node.children, node_manager)

    if node.call_as == "source-text":
        return node_builder("", "", *children, **node.attributes)
    elif node.call_as == "source":
        return node_builder("", *children, **node.attributes)
    elif node.call_as == "args":
        return node_builder(*children, **node.attributes)
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
        node_list = backend.render_class(name, path, self.cache)
        document = self.state.document

        node_builder = NodeManager(document)
        return render_node_list(node_list, node_builder)


class StructDirective(Directive):
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
        node_list = backend.render_struct(name, path, self.cache)
        document = self.state.document

        node_builder = NodeManager(document)
        return render_node_list(node_list, node_builder)


class EnumDirective(Directive):
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
        node_list = backend.render_enum(name, path, self.cache)
        document = self.state.document

        node_builder = NodeManager(document)
        return render_node_list(node_list, node_builder)


class FunctionDirective(Directive):
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
        node_list = backend.render_function(name, path, self.cache)
        document = self.state.document

        node_builder = NodeManager(document)
        return render_node_list(node_list, node_builder)


class GroupDirective(Directive):
    has_content = True
    required_arguments = 1
    optional_arguments = 0
    final_argument_whitespace = True
    option_spec = {
        "project": directives.unchanged,
        "content-only": directives.flag,  # TODO: Implement
        "inner": directives.flag,         # TODO: Implement
    }

    def run(self) -> List[Node]:
        name = self.arguments[0]
        project = self.options.get("project", self.app.config.breathe_default_project)
        path = self.app.config.breathe_projects[project]
        node_list = backend.render_group(name, path, self.cache)
        document = self.state.document

        node_builder = NodeManager(document)
        return render_node_list(node_list, node_builder)


class Context:
    def __init__(self, app: Sphinx, cache):
        self.app = app
        self.cache = cache


def add_directive(context, name, Cls):
    Cls.app = context.app
    Cls.cache = context.cache
    context.app.add_directive(name, Cls)


def setup(app: Sphinx):
    cache = backend.Cache()

    context = Context(app, cache)

    add_directive(context, "doxygenclass", ClassDirective)
    add_directive(context, "doxygenstruct", StructDirective)
    add_directive(context, "doxygenfunction", FunctionDirective)
    add_directive(context, "doxygenenum", EnumDirective)
    add_directive(context, "doxygengroup", GroupDirective)

    app.add_config_value("breathe_projects", {}, "env")
    app.add_config_value("breathe_default_project", None, "env")

    return {"version": __version__}
