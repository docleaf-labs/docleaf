import logging

from sphinx.domains import cpp
from docutils import nodes

from . import copied


def render_domain_entry(name, type, declaration, directive_args, content):
    # print("render_domain_entry", name, type, declaration)
    if name == "cpp" and type == "function":
        args = [type, [declaration]] + directive_args[2:]
        directive = cpp.CPPFunctionObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        finder.content.children = content

        return nodes

    if name == "cpp" and type == "class":
        args = [type, [declaration]] + directive_args[2:]
        directive = cpp.CPPClassObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        finder.content.children = content

        return nodes

    else:
        return nodes.Text("domains:domain entry")
