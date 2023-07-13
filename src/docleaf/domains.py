from sphinx.domains import cpp, c

from . import copied
from .errors import DocleafError


def null_handler(finder, location):
    return


def strip_desc_addname(finder, location):
    # We pass qualified values to the domain directives but we don't always want Sphinx
    # to show the qualified part in the output as normally it is shown but the nesting
    # of the entities so we strip it out which involes removing the 'desc_addname' node
    # in the output
    finder.declarator.children = [node for node in finder.declarator.children if node.tagname != "desc_addname"]


def add_location_via_names(finder, location):
    if finder.signature and location:
        finder.signature["names"] = f"{location['path']}:{location['line']}"


def chain(*funcs):
    def inner(finder, location):
        for func in funcs:
            func(finder, location)

    return inner


cpp_domain = {
    "class": (cpp.CPPClassObject, "class", add_location_via_names),
    "enum": (cpp.CPPEnumObject, "enum", add_location_via_names),
    "enumerator": (
        cpp.CPPEnumeratorObject,
        "enumerator",
        chain(strip_desc_addname, add_location_via_names),
    ),
    "function": (
        cpp.CPPFunctionObject,
        "function",
        chain(strip_desc_addname, add_location_via_names),
    ),
    "member": (cpp.CPPMemberObject, "member", chain(strip_desc_addname, add_location_via_names)),
    "struct": (cpp.CPPClassObject, "struct", add_location_via_names),
}

c_domain = {
    "define": (c.CMacroObject, "macro", add_location_via_names),
    "enum": (c.CEnumObject, "enum", add_location_via_names),
    "enumerator": (
        c.CEnumeratorObject,
        "enumerator",
        chain(strip_desc_addname, add_location_via_names),
    ),
    "function": (c.CFunctionObject, "function", add_location_via_names),
    "member": (c.CMemberObject, "member", chain(strip_desc_addname, add_location_via_names)),
    "struct": (c.CStructObject, "struct", add_location_via_names),
    "typedef": (c.CTypeObject, "type", add_location_via_names),
    "union": (c.CUnionObject, "union", chain(strip_desc_addname, add_location_via_names)),
}

domains = {"cpp": cpp_domain, "c": c_domain}


def render_domain_entry(
    domain_name: str,
    type: str,
    declaration: str,
    location,
    target,
    directive_args: list,
    content: list,
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

    handler(finder, location)

    return nodes


def set_children(node, children):
    """
    The children have to be informed of the parent and that happens in the parents
    helper methods like 'append' and 'extend' so we can't just replace 'node.children'
    with 'children'. Instead we empty the children list and then add the children so
    they are set up properly.
    """
    node.children.clear()
    node.extend(children)
