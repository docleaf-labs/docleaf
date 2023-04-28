from sphinx.domains.cpp import CPPFunctionObject
from docutils import nodes


def render_domain_entry(name, type, declaration, directive_args):
    if name == "cpp" and type == "function":
        args = [type, [declaration]] + directive_args[2:]
        print(directive_args)
        print(args)
        directive = CPPFunctionObject(*args)
        return directive.run()

    else:
        return nodes.Text("domains:domain entry")
