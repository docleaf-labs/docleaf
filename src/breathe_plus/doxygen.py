from typing import List
import textwrap
import itertools

from docutils.nodes import Node
from docutils.parsers.rst.directives import unchanged_required, unchanged, flag
from docutils.parsers.rst.states import Text
from docutils.parsers.rst import Directive, directives
from docutils.statemachine import StringList
from docutils import nodes

from sphinx.application import Sphinx
from sphinx.util.nodes import nested_parse_with_titles
from sphinx.domains import c
import sphinx.addnodes

from . import backend, domains, copied

__version__ = "0.0.0"


def as_list(node):
    def wrapper(*children, **attributes):
        return [node(*children, **attributes)]

    return wrapper


class NodeManager:
    def __init__(self, state, directive_arguments):
        self.state = state
        self.directive_arguments = directive_arguments
        self.lookup = {
            "bullet_list": as_list(nodes.bullet_list),
            "container": as_list(nodes.container),
            "colspec": as_list(nodes.colspec),
            "desc": as_list(sphinx.addnodes.desc),
            "desc_content": as_list(sphinx.addnodes.desc_content),
            "desc_name": as_list(sphinx.addnodes.desc_name),
            "desc_parameter": as_list(sphinx.addnodes.desc_parameter),
            "desc_parameterlist": as_list(sphinx.addnodes.desc_parameterlist),
            "desc_sig_keyword": as_list(sphinx.addnodes.desc_sig_keyword),
            "desc_sig_name": as_list(sphinx.addnodes.desc_sig_name),
            "desc_sig_space": as_list(sphinx.addnodes.desc_sig_space),
            "desc_signature": as_list(sphinx.addnodes.desc_signature),
            "desc_signature_line": as_list(sphinx.addnodes.desc_signature_line),
            "emphasis": as_list(nodes.emphasis),
            "entry": as_list(nodes.entry),
            "enumerated_list": as_list(nodes.enumerated_list),
            "index": as_list(sphinx.addnodes.index),
            "inline": as_list(nodes.inline),
            "list_item": as_list(nodes.list_item),
            "literal": as_list(nodes.literal),
            "literal_block": as_list(nodes.literal_block),
            "literal_strong": as_list(sphinx.addnodes.literal_strong),
            "only": as_list(sphinx.addnodes.only),
            "paragraph": as_list(nodes.paragraph),
            "raw": as_list(nodes.raw),
            "reference": as_list(nodes.reference),
            "restructured_text_block": self.build_restructured_text_block,
            "restructured_text_inline": self.build_restructured_text_inline,
            "row": as_list(nodes.row),
            "rubric": as_list(nodes.rubric),
            "strong": as_list(nodes.strong),
            "table": as_list(nodes.table),
            "tbody": as_list(nodes.tbody),
            "tgroup": as_list(nodes.tgroup),
            "thead": as_list(nodes.thead),
            # Special
            "target": self.build_target,
            "domain_entry": self.build_domain_entry,
        }

    def get_builder(self, node_type):
        builder = self.lookup[node_type]
        return builder

    def build_target(self, key, *children, **attributes):
        target = nodes.target(key, *children, **attributes)
        self.state.document.note_explicit_target(target)
        return [target]

    def build_domain_entry(self, *children, **attributes):
        return domains.render_domain_entry(
            attributes["domain"],
            attributes["type"],
            attributes["declaration"],
            self.directive_arguments,
            children,
        )

    def build_restructured_text_block(self, *children, **attributes):
        text = textwrap.dedent(children[0])

        # Inspired by autodoc.py in Sphinx
        rst = StringList()
        for line in text.split("\n"):
            rst.append(line, "<breathe>")

        # Parent node for the generated node subtree
        rst_node = nodes.paragraph()
        rst_node.document = self.state.document

        nested_parse_with_titles(self.state, rst, rst_node)

        return [rst_node]

    def build_restructured_text_inline(self, *children, **attributes):
        text = children[0]

        # Inspired by autodoc.py in Sphinx
        rst = StringList()
        for line in text.split("\n"):
            rst.append(line, "<breathe>")

        rst_node = nodes.inline()
        rst_node.document = self.state.document

        copied.nested_inline_parse_with_titles(self.state, rst, rst_node)

        return [rst_node]


def render_node_list(node_list, node_manager):
    # Use nested comprehension to flatten nodes lists coming back from render_node
    return flatten(render_node(node, node_manager) for node in node_list)


def flatten(list_of_lists):
    return list(itertools.chain.from_iterable(list_of_lists))


def render_node(node, node_manager):
    if node.type == "text":
        return [nodes.Text(node.text)]

    node_builder = node_manager.get_builder(node.type)
    children = render_node_list(node.children, node_manager)

    if node.call_as == "source-text":
        return node_builder("", "", *children, **node.attributes)
    elif node.call_as == "source":
        return node_builder("", *children, **node.attributes)
    elif node.call_as == "args":
        return node_builder(*children, **node.attributes)
    else:
        raise Exception("Call As not implemented: " + node.call_as)


class BaseDirective(Directive):
    def get_directive_args(self) -> list:
        # Must match order in docutils.parsers.rst.Directive.__init__
        return [
            self.name,
            self.arguments,
            self.options,
            self.content,
            self.lineno,
            self.content_offset,
            self.block_text,
            self.state,
            self.state_machine,
        ]


class ClassDirective(BaseDirective):
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

        node_builder = NodeManager(self.state, self.get_directive_args())
        return render_node_list(node_list, node_builder)


class StructDirective(BaseDirective):
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

        node_builder = NodeManager(self.state, self.get_directive_args())
        return render_node_list(node_list, node_builder)


class EnumDirective(BaseDirective):
    has_content = True
    required_arguments = 1
    optional_arguments = 0
    final_argument_whitespace = True
    option_spec = {
        "project": directives.unchanged,
        "skip-xml-nodes": directives.unchanged,
    }

    def run(self) -> List[Node]:
        name = self.arguments[0]
        project = self.options["project"] or self.app.config.breathe_default_project
        path = self.app.config.breathe_projects[project]
        skip_xml_nodes = get_skip_xml_nodes(self.app, self.options)

        context = backend.Context(skip_xml_nodes)
        node_list = backend.render_enum(name, path, context, self.cache)

        node_builder = NodeManager(self.state, self.get_directive_args())
        return render_node_list(node_list, node_builder)


class FunctionDirective(BaseDirective):
    has_content = True
    required_arguments = 1
    optional_arguments = 0
    final_argument_whitespace = True
    option_spec = {
        "project": directives.unchanged,
        "skip-xml-nodes": directives.unchanged,
    }

    def run(self) -> List[Node]:
        name = self.arguments[0]
        project = self.options["project"] or self.app.config.breathe_default_project
        path = self.app.config.breathe_projects[project]
        skip_xml_nodes = get_skip_xml_nodes(self.app, self.options)

        context = backend.Context(skip_xml_nodes)
        node_list = backend.render_function(name, path, context, self.cache)

        node_builder = NodeManager(self.state, self.get_directive_args())
        return render_node_list(node_list, node_builder)


class GroupDirective(BaseDirective):
    has_content = True
    required_arguments = 1
    optional_arguments = 0
    final_argument_whitespace = True
    option_spec = {
        "project": directives.unchanged,
        "content-only": directives.flag,  # TODO: Implement
        "inner": directives.flag,  # TODO: Implement
        "skip-xml-nodes": directives.unchanged,
    }

    def run(self) -> List[Node]:
        name = self.arguments[0]
        project = self.options.get("project", self.app.config.breathe_default_project)
        path = self.app.config.breathe_projects[project]

        skip_xml_nodes = get_skip_xml_nodes(self.app, self.options)
        content_only = "content-only" in self.options
        inner_group = "inner" in self.options
        context = backend.Context(skip_xml_nodes)
        node_list = backend.render_group(name, path, context, content_only, inner_group, self.cache)

        node_builder = NodeManager(self.state, self.get_directive_args())
        return render_node_list(node_list, node_builder)


def get_skip_xml_nodes(app, options):
    """
    Get the option for the directive and fallback to the app option if not defined on the directive
    """
    skip_xml_nodes = options.get("skip-xml-nodes", None)
    if skip_xml_nodes is None:
        skip_xml_nodes = app.config.breathe_skip_doxygen_xml_nodes
    else:
        skip_xml_nodes = skip_xml_nodes.split(",")
    return skip_xml_nodes


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
    app.add_config_value("breathe_skip_doxygen_xml_nodes", [], "env")

    return {"version": __version__, "parallel_read_safe": True, "parallel_write_safe": True}
