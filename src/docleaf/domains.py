import logging

from sphinx.domains import cpp, c
from docutils import nodes

from . import copied
from .errors import DocleafError

null_handler = lambda finder: finder


def enumerator_handler(finder):
    # We pass EnumName::EnumeratorName to the CPPEnumeratorObject directive but we don't want to have the
    # "EnumName::" part in the output so we find the 'desc_addname' that Sphinx uses for it and remove it
    finder.declarator.children = [
        node for node in finder.declarator.children if node.tagname != "desc_addname"
    ]


cpp_domain = {
    "class": (cpp.CPPClassObject, null_handler),
    "enum": (cpp.CPPEnumObject, null_handler),
    "enumerator": (cpp.CPPEnumeratorObject, enumerator_handler),
    "function": (cpp.CPPFunctionObject, null_handler),
    "struct": (cpp.CPPClassObject, null_handler),
}

c_domain = {
    "enum": (c.CEnumObject, null_handler),
    "enumerator": (c.CEnumeratorObject, enumerator_handler),
    "function": (c.CFunctionObject, null_handler),
    "struct": (c.CStructObject, null_handler),
}

domains = {"cpp": cpp_domain, "c": c_domain}


def render_domain_entry(
    name: str, type: str, declaration: str, target, directive_args: list, content: list
):
    # print("render_domain_entry", name, type, declaration)
    try:
        domain = domains[name]
    except KeyError:
        raise DocleafError(f"Unsupported domain: {name}")

    try:
        (Directive, handler) = domain[type]
    except KeyError:
        raise DocleafError(f'Unsupported type "{type}" on domain "{name}"')

    directive_name = f"{name}:{type}"

    args = [directive_name, [declaration]] + directive_args[2:]
    directive = Directive(*args)

    nodes = directive.run()

    rst_node = nodes[1]
    finder = copied.NodeFinder(rst_node.document)
    rst_node.walk(finder)

    set_children(finder.content, content)
    finder.declarator.children.insert(0, target)

    handler(finder)

    return nodes


def set_children(node, children):
    """
    The children have to be informed of the parent and that happens in the parents helper methods like 'append' and
    'extend' so we can't just replace 'node.children' with 'children'. Instead we empty the children list and then
    add the children so they are set up properly.
    """
    node.children.clear()
    node.extend(children)
