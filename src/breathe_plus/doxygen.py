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

from . import backend, domains

__version__ = "0.0.0"


# Taken from the Breathe code base
class InlineText(Text):
    """
    Add a custom docutils class to allow parsing inline text. This is to be
    used inside a @verbatim/@endverbatim block but only the first line is
    consumed and a inline element is generated as the parent, instead of the
    paragraph used by Text.
    """

    patterns = {"inlinetext": r""}
    initial_transitions = [("inlinetext",)]

    def indent(self, match, context, next_state):
        """
        Avoid Text's indent from detecting space prefixed text and
        doing "funny" stuff; always rely on inlinetext for parsing.
        """
        return self.inlinetext(match, context, next_state)

    def eof(self, context):
        """
        Text.eof() inserts a paragraph, so override it to skip adding elements.
        """
        return []

    def inlinetext(self, match, context, next_state):
        """
        Called by the StateMachine when an inline element is found (which is
        any text when this class is added as the single transition.
        """
        startline = self.state_machine.abs_line_number() - 1
        msg = None
        try:
            block = self.state_machine.get_text_block()
        except UnexpectedIndentationError as err:
            block, src, srcline = err.args
            msg = self.reporter.error("Unexpected indentation.", source=src, line=srcline)
        lines = context + list(block)
        text, _ = self.inline_text(lines[0], startline)
        self.parent += text
        self.parent += msg
        return [], next_state, []


# Taken from the Breathe code base
def nested_inline_parse_with_titles(state, content, node) -> str:
    """
    This code is basically a customized nested_parse_with_titles from
    docutils, using the InlineText class on the statemachine.
    """
    surrounding_title_styles = state.memo.title_styles
    surrounding_section_level = state.memo.section_level
    state.memo.title_styles = []
    state.memo.section_level = 0
    try:
        return state.nested_parse(
            content,
            0,
            node,
            match_titles=1,
            state_machine_kwargs={
                "state_classes": (InlineText,),
                "initial_state": "InlineText",
            },
        )
    finally:
        state.memo.title_styles = surrounding_title_styles
        state.memo.section_level = surrounding_section_level


class NodeManager:
    def __init__(self, state, directive_arguments):
        self.state = state
        self.directive_arguments = directive_arguments
        self.lookup = {
            "bullet_list": nodes.bullet_list,
            "container": nodes.container,
            "colspec": nodes.colspec,
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
            "entry": nodes.entry,
            "enumerated_list": nodes.enumerated_list,
            "index": sphinx.addnodes.index,
            "inline": nodes.inline,
            "list_item": nodes.list_item,
            "literal": nodes.literal,
            "literal_block": nodes.literal_block,
            "literal_strong": sphinx.addnodes.literal_strong,
            "only": sphinx.addnodes.only,
            "paragraph": nodes.paragraph,
            "raw": nodes.raw,
            "reference": nodes.reference,
            "restructured_text_block": self.build_restructured_text_block,
            "restructured_text_inline": self.build_restructured_text_inline,
            "row": nodes.row,
            "rubric": nodes.rubric,
            "strong": nodes.strong,
            "table": nodes.table,
            "tbody": nodes.tbody,
            "tgroup": nodes.tgroup,
            "thead": nodes.thead,
            "target": self.build_target,
            # Special
            "domain_entry": self.build_domain_entry,
        }

    def get_builder(self, node_type):
        builder = self.lookup[node_type]
        return lambda *args, **attrs: [builder(*args, **attrs)]

    def build_target(self, key, *children, **attributes):
        # object = c.CFunctionObject()
        # object.run()

        target = nodes.target(key, *children, **attributes)
        self.state.document.note_explicit_target(target)
        return [target]

    def build_domain_entry(self, *children, **attributes):
        return domains.render_domain_entry(
            attributes["domain"],
            attributes["type"],
            attributes["declaration"],
            self.directive_arguments,
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

        nested_inline_parse_with_titles(self.state, rst, rst_node)

        return [rst_node]


def render_node_list(node_list, node_manager):
    # Use nested comprehension to flatten nodes lists coming back from render_node
    return flatten(render_node(node, node_manager) for node in node_list)


def flatten(list_of_lists):
    return itertools.chain.from_iterable(list_of_lists)


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

        result = render_node_list(node_list, node_builder)
        return result
        # return render_node_list(node_list, node_builder)


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
