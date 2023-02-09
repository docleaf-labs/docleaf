import xml.etree.ElementTree as ET
import re
import sys


def main():
    tree = ET.parse("examples/nutshell/xml/compound.xsd")
    root = tree.getroot()

    for child in root:
        if "name" in child.attrib:
            if child.tag == "{http://www.w3.org/2001/XMLSchema}complexType":
                output_struct(child)
            elif child.tag == "{http://www.w3.org/2001/XMLSchema}simpleType":
                for grandchild in child:
                    if grandchild.tag == "{http://www.w3.org/2001/XMLSchema}restriction":
                        name = child.attrib["name"]
                        output_restriction(name, grandchild)


def output_struct(tag):
    name = convert_type_name(tag.attrib["name"], False)

    attribute_fields = []
    element_fields = []
    for child in tag:
        if child.tag == "{http://www.w3.org/2001/XMLSchema}attribute":
            if "name" in child.attrib:
                field_name = convert_field_name(child.attrib["name"])
                field_type = convert_type_name(child.attrib["type"], True)

                attribute_fields.append(f"  {field_name}: {field_type}")

        if child.tag == "{http://www.w3.org/2001/XMLSchema}sequence":
            for element in child:
                if "name" in element.attrib and "type" in element.attrib:
                    field_name = convert_field_name(element.attrib["name"])
                    field_type = convert_type_name(element.attrib["type"], True)

                    min_occurs = (
                        int(element.attrib["minOccurs"]) if "minOccurs" in element.attrib else 1
                    )

                    max_occurs = element.attrib["maxOccurs"] if "maxOccurs" in element.attrib else 1

                    if min_occurs == 0 and max_occurs in [1, "1"]:
                        element_fields.append(f"    {field_name}: Option<{field_type}>")
                    elif min_occurs == 0 and max_occurs == "unbounded":
                        element_fields.append(f"    {field_name}: Vec<{field_type}>")
                    elif min_occurs == 1 and max_occurs == "unbounded":
                        element_fields.append(f"    {field_name}: vec1::Vec1<{field_type}>")
                    elif min_occurs == 1 and max_occurs in [1, "1"]:
                        element_fields.append(f"    {field_name}: {field_type}")
                    else:
                        raise Exception(f"min:{repr(min_occurs)} max:{repr(max_occurs)}")

    attribute_fields = ",\n".join(attribute_fields)
    attribute_fields = f"{attribute_fields}," if attribute_fields else attribute_fields
    element_fields = ",\n".join(element_fields)

    print(
        f"""
#[derive(Debug, PartialEq)]
pub struct {name} {{
// Attributes
{attribute_fields}
// Children
{element_fields}
}}
"""
    )


def output_restriction(name, tag):

    # Handle special cases which are patterns rather than enums
    if name in ["DoxVersionNumber", "DoxCharRange"]:
        print(f"type {name} = String;")
        return

    # Skip weird type until we need it
    if name == "DoxOlType":
        return

    name = convert_type_name(name, False)

    entries = []
    for child in tag:
        entry_name = child.attrib["value"]
        entry_name = convert_enum_name(entry_name)
        entries.append(entry_name)

    entries = ",\n    ".join(entries)

    print(
        f"""
#[derive(Debug, PartialEq)]
pub enum {name} {{
    {entries}
}}
"""
    )


def convert_enum_name(name):
    name = capitalize(name)
    # print(name, file=sys.stderr)
    while True:
        match = re.search("-[A-Za-z]", name)
        if match:
            span = match.span()
            # print(match.group(), file=sys.stderr)
            name = name[: span[0]] + match.group()[1:].upper() + name[span[1] :]
            # print(name, file=sys.stderr)
        else:
            break

    name = name.replace("#", "Sharp")
    name = name.replace("+", "Plus")

    return name


def capitalize(name):
    return name[0].upper() + name[1:]


type_lookup = {
    "xsd:string": "String",
    "xsd:integer": "i32",
    "DoxBool": "bool",
    "DoxOlType": "String",
}


box = ["TableofcontentsType"]


def convert_type_name(name, as_field_type):
    if name in type_lookup:
        return type_lookup[name]

    name = capitalize(name)
    name = name.replace("def", "Def")
    name = name.replace("type", "Type")
    name = name.replace("kind", "Kind")
    name = name.replace("class", "Class")

    if as_field_type and name in box:
        return f"Box<{name}>"

    return name


def convert_field_name(name):
    keywords = ["ref", "type", "static", "const", "final", "abstract"]
    if name in keywords:
        return f"{name}_"

    name = name.replace("ddef", "d_def")
    return name


if __name__ == "__main__":
    main()
