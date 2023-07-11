from typing import List
from pathlib import Path
import itertools
import textwrap
import hashlib
import time
import sys
import os

from docutils.nodes import Node
from docutils.parsers.rst import Directive, directives
from docutils.statemachine import StringList
from docutils import nodes

from sphinx.application import Sphinx
from sphinx.environment import BuildEnvironment
from sphinx.util.nodes import nested_parse_with_titles
from sphinx.util import logging
import sphinx.addnodes

from . import backend, domains, copied
from .errors import DocleafError

__version__ = "0.0.0"

logger = logging.getLogger(__name__)


class GitHubLinkResolver:
    """
    This works on the assumption that we can insert our own data into the 'names' field of desc_signature nodes in
    domain entries. That assumption might fail or clash with other things so we try to be careful when extracting the
    data from 'names' and fail early and quietly.
    """

    def __init__(self, *, root, user, repo, tag=None, branch=None, revision=None):
        self.root = Path(root).resolve()
        self.user = user
        self.repo = repo
        self.tag = tag
        self.branch = branch
        self.revision = revision

    def __call__(self, domain, info):
        if domain not in ["c", "cpp"]:
            return None

        names = info.get("names")
        if not names:
            return None

        entries = names.rsplit(":", 1)
        if len(entries) != 2:
            return None

        path = Path(entries[0]).resolve()
        relative = path.relative_to(self.root)

        try:
            line = int(entries[1])
        except ValueError:
            # Unable to get line number - exit quietly
            return None

        reference = self.tag or self.branch or self.revision
        if not reference:
            return None

        return f"https://github.com/{self.user}/{self.repo}/blob/{reference}/{relative}#L{line}"


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
            "field_list": as_list(nodes.field_list),
            "field": as_list(nodes.field),
            "field_name": as_list(nodes.field_name),
            "field_body": as_list(nodes.field_body),
            "image": as_list(nodes.image),
            "index": as_list(sphinx.addnodes.index),
            "inline": as_list(nodes.inline),
            "list_item": as_list(nodes.list_item),
            "literal": as_list(nodes.literal),
            "literal_block": as_list(nodes.literal_block),
            "literal_strong": as_list(sphinx.addnodes.literal_strong),
            "note": as_list(nodes.note),
            "only": as_list(sphinx.addnodes.only),
            "paragraph": as_list(nodes.paragraph),
            "raw": as_list(nodes.raw),
            "internal_reference": self.build_internal_reference,
            "external_reference": as_list(nodes.reference),
            "restructured_text_block": self.build_restructured_text_block,
            "restructured_text_inline": self.build_restructured_text_inline,
            "row": as_list(nodes.row),
            "rubric": as_list(nodes.rubric),
            "strong": as_list(nodes.strong),
            "see_also": as_list(sphinx.addnodes.seealso),
            "table": as_list(nodes.table),
            "tbody": as_list(nodes.tbody),
            "tgroup": as_list(nodes.tgroup),
            "thead": as_list(nodes.thead),
            "warning": as_list(nodes.warning),
            # Special
            "target": as_list(self.build_target),
            "domain_entry": self.build_domain_entry,
        }

    def get_builder(self, node_type):
        builder = self.lookup[node_type]
        return builder

    def build_target(self, *children, **attributes):
        target = nodes.target(*children, **attributes)
        self.state.document.note_explicit_target(target)
        return target

    def build_domain_entry(self, *children, **attributes):
        return domains.render_domain_entry(
            attributes["domain"],
            attributes["type"],
            attributes["declaration"],
            attributes.get("location"),
            self.build_target(**attributes["target"]),
            self.directive_arguments,
            children,
        )

    def build_internal_reference(self, *children, **attributes):
        reference = sphinx.addnodes.pending_xref(
            "",
            *children,
            reftype="ref",
            refdomain="std",
            refexplicit=True,
            refid=attributes["refid"],
            reftarget=attributes["refid"],
        )
        return [reference]

    def build_restructured_text_block(self, *children, **attributes):
        text = textwrap.dedent(children[0])

        # Inspired by autodoc.py in Sphinx
        rst = StringList()
        for line in text.split("\n"):
            rst.append(line, "<docleaf>")

        # Parent node for the generated node subtree
        rst_node = nodes.paragraph()
        rst_node.document = self.state.document

        nested_parse_with_titles(self.state, rst, rst_node)

        # We render the block into a paragraph node but the rendred block will contain
        # its own paragraph nodes at the top level so we don't need to return our
        # paragraph node so we return its children (the top layer of rendered nodes) as
        # the output
        return rst_node.children

    def build_restructured_text_inline(self, *children, **attributes):
        text = children[0]

        # Inspired by autodoc.py in Sphinx
        rst = StringList()
        for line in text.split("\n"):
            rst.append(line, "<docleaf>")

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

    if node.call_as == "text-element":
        return node_builder("", "", *children, **node.attributes)
    elif node.call_as == "element":
        return node_builder("", *children, **node.attributes)
    elif node.call_as == "function":
        return node_builder(*children, **node.attributes)
    else:
        raise DocleafError("Call As not implemented: " + node.call_as)


class Project:
    def __init__(self, root, xml):
        self._root = root
        self._xml = xml

    def root(self):
        return self._root

    def xml(self):
        return self._xml

    def get(projects, name: str):
        # For each 'try' block we need to catch KeyError and TypeError (if project is a string) so we catch everything
        # as there isn't much else that could go wrong

        try:
            data = projects[name]
        except Exception:
            raise DocleafError(
                f"Unable to find a project called '{name}' defined in the docleaf_projects config variable"
            )

        try:
            root = data["root"]
        except Exception:
            raise DocleafError(
                f"Unable to find the 'root' entry in the data for '{name}' project defined in the docleaf_projects "
                "config variable"
            )

        try:
            xml = data["xml"]
        except Exception:
            raise DocleafError(
                f"Unable to find the 'xml' entry in the data for '{name}' project defined in the docleaf_projects "
                "config variable"
            )

        return Project(root, xml)


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


class BasicDoxygenDirective(BaseDirective):
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
        project_name = self.options.get("project", self.app.config.docleaf_default_project)
        project = Project.get(self.app.config.docleaf_projects, project_name)
        skip_settings = get_skip_settings(self.app, self.options)

        build_dir = Path(self.app.doctreedir).parent / "docleaf"
        build_dir.mkdir(parents=True, exist_ok=True)

        context = backend.Context(
            project.root(),
            str(build_dir),
            skip_settings,
            self.app.config.docleaf_domain_by_extension,
            self.app.config.docleaf_mermaid_command,
        )

        tracked_cache = backend.TrackedCache(self.cache)
        node_list = self.render_function(name, project.xml(), context, tracked_cache)
        update_sphinx_env_file_data(self.app.env, tracked_cache.xml_paths(), self.app.env.docname)

        node_builder = NodeManager(self.state, self.get_directive_args())
        return render_node_list(node_list, node_builder)


class ClassDirective(BasicDoxygenDirective):
    directive_name = "doxygen-class"
    render_function = backend.render_class


class StructDirective(BasicDoxygenDirective):
    directive_name = "doxygen-struct"
    render_function = backend.render_struct


class EnumDirective(BasicDoxygenDirective):
    directive_name = "doxygen-enum"
    render_function = backend.render_enum


class FunctionDirective(BasicDoxygenDirective):
    directive_name = "doxygen-function"
    render_function = backend.render_function


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
        project_name = self.options.get("project", self.app.config.docleaf_default_project)
        project = Project.get(self.app.config.docleaf_projects, project_name)

        build_dir = Path(self.app.doctreedir).parent / "docleaf"
        build_dir.mkdir(parents=True, exist_ok=True)

        skip_settings = get_skip_settings(self.app, self.options)
        content_only = "content-only" in self.options
        inner_group = "inner" in self.options
        context = backend.Context(
            project.root(),
            str(build_dir),
            skip_settings,
            self.app.config.docleaf_domain_by_extension,
            self.app.config.docleaf_mermaid_command,
        )

        tracked_cache = backend.TrackedCache(self.cache)
        node_list = backend.render_group(
            name,
            project.xml(),
            context,
            content_only,
            inner_group,
            tracked_cache,
        )
        update_sphinx_env_file_data(self.app.env, tracked_cache.xml_paths(), self.app.env.docname)

        node_builder = NodeManager(self.state, self.get_directive_args())
        return render_node_list(node_list, node_builder)


class InheritanceGraphDirective(BaseDirective):
    has_content = True
    required_arguments = 1
    optional_arguments = 0
    final_argument_whitespace = True
    option_spec = {
        "project": directives.unchanged,
    }

    def run(self) -> List[Node]:
        name = self.arguments[0]
        project_name = self.options.get("project", self.app.config.docleaf_default_project)
        project = Project.get(self.app.config.docleaf_projects, project_name)

        build_dir = Path(self.app.doctreedir).parent / "docleaf"
        build_dir.mkdir(parents=True, exist_ok=True)

        context = backend.Context(
            project.root(),
            str(build_dir),
            skip_settings,
            self.app.config.docleaf_domain_by_extension,
            self.app.config.docleaf_mermaid_command,
        )

        tracked_cache = backend.TrackedCache(self.cache)
        node_list = self.render_inheritance_graph(name, project.xml(), context, tracked_cache)
        update_sphinx_env_file_data(self.app.env, tracked_cache.xml_paths(), self.app.env.docname)

        node_builder = NodeManager(self.state, self.get_directive_args())
        return render_node_list(node_list, node_builder)


def get_skip_settings(app, options):
    """
    Get the option for the directive and fallback to the app option if not defined on the directive
    """
    skip_settings = options.get("skip", None)
    if skip_settings is None:
        skip_settings = app.config.docleaf_doxygen_skip
    else:
        skip_settings = skip_settings.split(",")
    return skip_settings


class XmlFileInfo:
    def __init__(self, hash, rst_files):
        self.hash = hash
        self.rst_files = rst_files


def hash_file(path):
    logger.debug(f"docleaf: hashing {path}")
    if sys.version_info >= (3, 11):
        with open(path, "rb") as f:
            digest = hashlib.file_digest(f, "sha1")
            return digest.hexdigest()
    else:
        contents = open(path, "rb").read()
        hash = hashlib.sha1()
        hash.update(contents)
        return hash.hexdigest()


def update_sphinx_env_file_data(env: BuildEnvironment, xml_paths: List[str], rst_file: str):
    """
    Update our file_data store to indicate which rst files are dependent on which xml files
    """
    if not hasattr(env, "docleaf_file_data"):
        env.docleaf_file_data = {}

    for path in xml_paths:
        if path in env.docleaf_file_data:
            env.docleaf_file_data[path].rst_files.add(rst_file)
        else:
            env.docleaf_file_data[path] = XmlFileInfo(hash_file(path), set([rst_file]))


def calculate_files_to_refresh(app: Sphinx, env, added, changed, removed):
    """
    Sphinx tells us which rst files have changed on disk and we can tell it (by returning a list of doc names) which
    documents need to be re-read and have their output re-generated.

    We calculate which files to refresh by storing the last build time and comparing the modified time of each xml file
    against the last build time and see if any are newer. From there we need to get from the xml files to the rst files
    that they impact and return those rst files to be refreshed.
    """
    logger.debug("docleaf: calculate_files_to_refresh")

    # Get the last build time before updating it
    last_build_time = getattr(app.env, "docleaf_last_build_time", None)

    # Store the current time as the build time
    app.env.docleaf_last_build_time = time.time()

    if not last_build_time:
        return []

    if not hasattr(app.env, "docleaf_file_data"):
        return []

    rst_files_to_refresh = set()

    for xml_file_path, info in app.env.docleaf_file_data.items():
        stat = os.stat(xml_file_path)
        if stat.st_mtime > last_build_time:
            hash = hash_file(xml_file_path)
            logger.debug(f"docleaf: stored hash for {xml_file_path} = {hash}. Calculated hash: {info.hash}")
            if hash != info.hash:
                # Store the new hash
                info.hash = hash
                # Tell Sphinx to refresh all rst files associated with this file
                rst_files_to_refresh.update(info.rst_files)

    logger.verbose(f"docleaf: calculate_files_to_refresh - Rebuild: {rst_files_to_refresh}")
    return rst_files_to_refresh


def purge_file_data(app, env, docname):
    """
    Clear any file data associated with 'docname' in accordance with the Sphinx API
    """
    logger.debug("docleaf: purge_file_data - %s", docname)
    if not hasattr(app.env, "docleaf_file_data"):
        return

    for xml_file_path, info in app.env.docleaf_file_data.items():
        info.rst_files.discard(docname)


class ExtensionContext:
    def __init__(self, app: Sphinx, cache):
        self.app = app
        self.cache = cache


def add_directive(context, name, Cls):
    Cls.app = context.app
    Cls.cache = context.cache
    context.app.add_directive(name, Cls)


def setup(app: Sphinx):
    cache = backend.FileCache()

    context = ExtensionContext(app, cache)

    add_directive(context, "doxygen-class", ClassDirective)
    add_directive(context, "doxygenclass", ClassDirective)  # Deprecated

    add_directive(context, "doxygen-enum", EnumDirective)
    add_directive(context, "doxygenenum", EnumDirective)  # Deprecated

    add_directive(context, "doxygen-function", FunctionDirective)
    add_directive(context, "doxygenfunction", FunctionDirective)  # Deprecated

    add_directive(context, "doxygen-group", GroupDirective)
    add_directive(context, "doxygengroup", GroupDirective)  # Deprecated

    add_directive(context, "doxygen-struct", StructDirective)
    add_directive(context, "doxygenstruct", StructDirective)  # Deprecated

    add_directive(context, "doxygen-inheritance-graph", InheritanceGraphDirective)

    # Args: name = str, default = Any, rebuild = bool | str, types = Any
    app.add_config_value("docleaf_projects", {}, "env")
    app.add_config_value("docleaf_default_project", None, "env")
    app.add_config_value("docleaf_doxygen_skip", [], "env")
    app.add_config_value("docleaf_domain_by_extension", {}, True)
    app.add_config_value("docleaf_mermaid_command", ["mmdc"], True)

    app.connect("env-get-outdated", calculate_files_to_refresh)
    app.connect("env-purge-doc", purge_file_data)

    return {"version": __version__, "parallel_read_safe": True, "parallel_write_safe": True}
