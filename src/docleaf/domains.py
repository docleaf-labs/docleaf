import logging

from sphinx.domains import cpp, c
from docutils import nodes

from . import copied
from .errors import DocleafError

null_handler = lambda finder: finder


def strip_desc_addname(finder):
    # We pass qualified values to the domain directives but we don't always want Sphinx to show the qualified part
    # in the output as normally it is shown but the nesting of the entities so we strip it out which involes removing
    # the 'desc_addname' node in the output
    finder.declarator.children = [
        node for node in finder.declarator.children if node.tagname != "desc_addname"
    ]


cpp_domain = {
    "class": (cpp.CPPClassObject, "class", null_handler),
    "enum": (cpp.CPPEnumObject, "enum", null_handler),
    "enumerator": (cpp.CPPEnumeratorObject, "enumerator", strip_desc_addname),
    "function": (cpp.CPPFunctionObject, "function", strip_desc_addname),
    "member": (cpp.CPPMemberObject, "member", strip_desc_addname),
    "struct": (cpp.CPPClassObject, "struct", null_handler),
}

c_domain = {
    "define": (c.CMacroObject, "macro", null_handler),
    "enum": (c.CEnumObject, "enum", null_handler),
    "enumerator": (c.CEnumeratorObject, "enumerator", strip_desc_addname),
    "function": (c.CFunctionObject, "function", null_handler),
    "member": (c.CMemberObject, "member", strip_desc_addname),
    "struct": (c.CStructObject, "struct", null_handler),
    "typedef": (c.CTypeObject, "type", null_handler),
    "union": (c.CUnionObject, "union", strip_desc_addname),
}

domains = {"cpp": cpp_domain, "c": c_domain}


def render_domain_entry(
    domain_name: str, type: str, declaration: str, target, directive_args: list, content: list
):
    # print("render_domain_entry", domain_name, type, declaration)
    try:
        domain = domains[domain_name]
    except KeyError:
        raise DocleafError(f"Unsupported domain: {domain_name}")

    try:
        (Directive, domain_specific_type, handler) = domain[type]
    except KeyError:
        raise DocleafError(f'Unsupported type "{type}" on domain "{domain_name}"')

    directive_name = f"{domain_name}:{domain_specific_type}"

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
