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

        set_children(finder.content, content)
        finder.declarator.children.insert(0, target)

        return nodes

    elif name == "cpp" and type == "class":
        args = [directive_name, [declaration]] + directive_args[2:]
        directive = cpp.CPPClassObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        set_children(finder.content, content)
        finder.declarator.children.insert(0, target)

        return nodes

    elif name == "cpp" and type == "struct":
        args = [directive_name, [declaration]] + directive_args[2:]
        directive = cpp.CPPClassObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        set_children(finder.content, content)
        finder.declarator.children.insert(0, target)

        return nodes

    elif name == "cpp" and type == "enum":
        args = [directive_name, [declaration]] + directive_args[2:]
        directive = cpp.CPPEnumObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        set_children(finder.content, content)
        finder.declarator.children.insert(0, target)

        return nodes

    elif name == "cpp" and type == "enumerator":
        args = [directive_name, [declaration]] + directive_args[2:]
        directive = cpp.CPPEnumeratorObject(*args)

        nodes = directive.run()

        rst_node = nodes[1]
        finder = copied.NodeFinder(rst_node.document)
        rst_node.walk(finder)

        set_children(finder.content, content)
        finder.declarator.children.insert(0, target)

        # We pass EnumName::EnumeratorName to the CPPEnumeratorObject directive but we don't want to have the
        # "EnumName::" part in the output so we find the 'desc_addname' that Sphinx uses for it and remove it
        finder.declarator.children = [node for node in finder.declarator.children if node.tagname != "desc_addname"]

        return nodes

    else:
        raise Exception(f"Unsupported domain name ({name}) and type ({type})")


def set_children(node, children):
    """
    The children have to be informed of the parent and that happens in the parents helper methods like 'append' and
    'extend' so we can't just replace 'node.children' with 'children'. Instead we empty the children list and then
    add the children so they are set up properly.
    """
    node.children.clear()
    node.extend(children)
