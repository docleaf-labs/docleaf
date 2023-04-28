import logging

from sphinx.domains.cpp import CPPFunctionObject
from docutils import nodes


def render_domain_entry(name, type, declaration, directive_args):
    # print("render_domain_entry", name, type, declaration)
    if name == "cpp" and type == "function":
        args = [type, [declaration]] + directive_args[2:]
        directive = CPPFunctionObject(*args)
        return directive.run()

    else:
        return nodes.Text("domains:domain entry")
