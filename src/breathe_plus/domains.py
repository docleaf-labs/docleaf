import logging

from sphinx.domains import cpp
from docutils import nodes

from . import copied


def render_domain_entry(name: str, type: str, declaration: str, target, directive_args: list, content: list):
    directive_name = f"{name}:{type}"
    # print("render_domain_entry", name, type, declaration)
    if name == "cpp" and type == "function":
        args = [directive_name, [declaration]] + directive_args[2:]
        directive = cpp.CPPFunctionObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        finder.content.children = content
        finder.declarator.children.insert(0, target)

        return nodes

    if name == "cpp" and type == "class":
        args = [directive_name, [declaration]] + directive_args[2:]
        directive = cpp.CPPClassObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        finder.content.children = content
        finder.declarator.children.insert(0, target)

        return nodes

    if name == "cpp" and type == "enum":
        args = [directive_name, [declaration]] + directive_args[2:]
        directive = cpp.CPPEnumObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        finder.content.children = content
        finder.declarator.children.insert(0, target)

        return nodes

    else:
        return nodes.Text("domains:domain entry")
