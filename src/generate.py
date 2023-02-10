from collections import defaultdict
from pprint import pprint
import xml.etree.ElementTree as ET
import re
import sys


def main(args):
    target_xsd_file = args[0]
    target_rs_file = args[1]

    comment_lookup = generate_comment_lookup(target_rs_file)

    tree = ET.parse(target_xsd_file)
    root = tree.getroot()

    with open(target_rs_file, "w") as output:
        for child in root:
            if "name" in child.attrib:
                if child.tag == "{http://www.w3.org/2001/XMLSchema}complexType":
                    if "mixed" in child.attrib:
                        output_mixed(output, child)
                    else:
                        output_struct(output, child, comment_lookup)
                elif child.tag == "{http://www.w3.org/2001/XMLSchema}simpleType":
                    for grandchild in child:
                        if grandchild.tag == "{http://www.w3.org/2001/XMLSchema}restriction":
                            name = child.attrib["name"]
                            output_restriction(output, name, grandchild)


def generate_comment_lookup(filepath):
    """
    Parses the current rust file with regexes to track which fields in which structs have
    been commented out so we can put those comments in when re-creating the file. This means
    that we're free to comment fields during development and not have to deal with them
    being uncommented by the script if we need to regenerate anything.
    """
    
    lookup = defaultdict(dict)
    key = None

    struct_re = re.compile(r"pub struct (.*) \{")
    enum_re = re.compile(r"pub enum (.*) \{")
    field_re = re.compile(r"    pub (.*): .*")
    comment_field_re = re.compile(r"    // pub (.*): .*")
    
    with open(filepath) as f:
        for line in f:
            if match := struct_re.match(line):
                # Lower case entries to make them robust to future case changes
                key = match.group(1).lower()
            elif match := enum_re.match(line):
                key = None
            elif match := field_re.match(line):
                if key:
                    # Lower case entries to make them robust to future case changes
                    lookup[key][match.group(1).lower()] = False
            elif match := comment_field_re.match(line):
                if key:
                    # Lower case entries to make them robust to future case changes
                    lookup[key][match.group(1).lower()] = True

    return lookup


def output_struct(output, tag, comment_lookup):
    """
    Output a rust struct for a given xsd tag
    """

    name = convert_type_name(tag.attrib["name"], False)

    attribute_fields = []
    element_fields = []
    for child in tag:
        if child.tag == "{http://www.w3.org/2001/XMLSchema}attribute":
            if "name" in child.attrib:
                field_name = convert_field_name(child.attrib["name"])
                field_type = convert_type_name(child.attrib["type"], True)

                # Robust comment lookup by using fallbacks
                comment = "// " if comment_lookup.get(name.lower(), {}).get(field_name.lower(), True) else ""

                attribute_fields.append(f"  {comment}pub {field_name}: {field_type}")

        if child.tag == "{http://www.w3.org/2001/XMLSchema}sequence":
            for element in child:
                if "name" in element.attrib:
                    field_name = convert_field_name(element.attrib["name"])
                    # Some elements don't have a 'type' attribute, I'm not sure if "String" is a reasonable fallback
                    # but we'll try it until proven wrong
                    field_type = convert_type_name(element.attrib["type"], True) if "type" in  element.attrib else "String"

                    min_occurs = (
                        int(element.attrib["minOccurs"]) if "minOccurs" in element.attrib else 1
                    )

                    max_occurs = element.attrib["maxOccurs"] if "maxOccurs" in element.attrib else 1

                    # Robust comment lookup by using fallbacks
                    comment = "// " if comment_lookup.get(name.lower(), {}).get(field_name.lower(), True) else ""

                    if min_occurs == 0 and max_occurs in [1, "1"]:
                        element_fields.append(f"    {comment}pub {field_name}: Option<{field_type}>")
                    elif min_occurs == 0 and max_occurs == "unbounded":
                        element_fields.append(f"    {comment}pub {field_name}: Vec<{field_type}>")
                    elif min_occurs == 1 and max_occurs == "unbounded":
                        element_fields.append(f"    {comment}pub {field_name}: vec1::Vec1<{field_type}>")
                    elif min_occurs == 1 and max_occurs in [1, "1"]:
                        element_fields.append(f"    {comment}pub {field_name}: {field_type}")
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
""",
    file=output
    )


def output_mixed(output, tag):
    """
    Output a rust enum for a mixed complex type
    """

    name = convert_type_name(tag.attrib["name"], False)

    sequence = None
    for child in tag:
        if child.tag == "{http://www.w3.org/2001/XMLSchema}sequence":
            sequence = child
            break

    if sequence is None:
        return

    entries = []
    for child in sequence:
        print(tag)
        print(sequence)
        print(child)
        if "name" in child.attrib and "type" in child.attrib:
            entry_name = child.attrib["name"]
            entry_type = child.attrib["type"]
            entry_name = convert_type_name(entry_name, False)
            entry_type = convert_type_name(entry_type, False)
            entries.append(f"{entry_name}({entry_type})")

    entries.append("Text(String)")

    entries = ",\n    ".join(entries)


    print(
        f"""
#[derive(Debug, PartialEq)]
pub enum {name} {{
    {entries}
}}
""",
        file=output
    )

def output_restriction(output, name, tag):
    """
    Output a rust enum for a given xsd tag
    """

    # Handle special cases which are patterns rather than enums
    if name in ["DoxVersionNumber", "DoxCharRange"]:
        print(f"type {name} = String;", file=output)
        return

    # Skip weird type until we need it
    if name in ["DoxOlType", "DoxBool"]:
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
#[derive(Debug, strum::EnumString, PartialEq)]
pub enum {name} {{
    {entries}
}}
""",
        file=output
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


def convert_type_name(name, as_field_type):
    if name in type_lookup:
        return type_lookup[name]

    name = capitalize(name)
    name = name.replace("def", "Def")
    name = name.replace("type", "Type")
    name = name.replace("kind", "Kind")
    name = name.replace("class", "Class")
    name = name.replace("value", "Value")
    name = name.replace("param", "Param")
    name = name.replace("list", "List")
    name = name.replace("contents", "Contents")
    name = name.replace("ofC", "OfC")

    return name


field_lookup = {
    "compounddef": "compound_def",
    "compoundname": "compound_name",
    "sectiondef": "section_defs",
    "memberdef": "member_defs",
    "briefdescription": "brief_description",
    "detaileddescription": "detailed_description",
}

def convert_field_name(name):
    keywords = ["ref", "type", "static", "const", "final", "abstract"]
    if name in keywords:
        return f"{name}_"

    name = field_lookup.get(name.lower(), name)
    return name


if __name__ == "__main__":
    main(sys.argv[1:])
