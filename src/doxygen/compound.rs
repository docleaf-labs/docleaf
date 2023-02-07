use anyhow::anyhow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

use crate::xml;

#[derive(Debug, PartialEq)]
pub struct Root {
    pub compound_def: CompoundDef,
}

#[derive(Debug, PartialEq)]
pub struct CompoundDef {
    pub id: String,
    pub compound_name: String,
    pub brief_description: Description,
    pub detailed_description: Description,
    pub section_defs: Vec<SectionDef>,
}

#[derive(Debug, PartialEq)]
pub struct SectionDef {
    pub kind: String,
    pub member_defs: Vec<MemberDef>,
}

#[derive(Debug, PartialEq)]
pub struct MemberDef {
    pub id: String,
    pub name: String,
    pub kind: MemberDefKind,
    pub brief_description: Description,
    pub detailed_description: Description,
}

#[derive(Debug, PartialEq)]
pub enum MemberDefKind {
    Enum { values: Vec<EnumValue> },
    Function { params: Vec<Param> },
    Variable,
    Unknown(String),
}

impl MemberDefKind {
    pub fn name(&self) -> String {
        match self {
            Self::Enum { .. } => String::from("enum"),
            Self::Function { .. } => String::from("function"),
            Self::Variable => String::from("variable"),
            Self::Unknown(name) => name.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub type_: LinkedText,
    pub declname: String,
}

#[derive(Debug, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub initializer: String,
    pub brief_description: Description,
    pub detailed_description: Description,
}

#[derive(Debug, Default, PartialEq)]
pub struct Description {
    pub content: Vec<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub enum DescriptionType {
    Para(Vec<DocPara>),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum DocPara {
    ParameterList(DocParamList),
    SimpleSect(DocSimpleSect),
    Ref(RefText),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocParamList {
    // kind: DoxParamListKind,
    parameter_items: Vec<DocParamListItem>,
}

#[derive(Debug, PartialEq)]
pub struct DocParamListItem {
    parameter_name_list: Vec<DocParamNameList>,
    parameter_description: Description,
}

#[derive(Debug, PartialEq)]
pub struct DocParamNameList {
    parameter_type: Option<DocParamType>,
    parameter_name: Option<DocParamName>,
}

#[derive(Debug, PartialEq)]
pub enum DocParamType {
    Ref(RefText),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum DocParamName {
    Ref(RefText),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct DocSimpleSect {
    // title: String,
    pub paras: Vec<DocPara>,
}

#[derive(Debug, PartialEq)]
pub enum DoxParamListKind {
    Param,
    Retval,
    Exception,
    TemplateParam,
}

#[derive(Debug, PartialEq)]
pub enum LinkedText {
    Ref(RefText),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct RefText {
    pub id: String,
    pub text: String,
}

pub fn parse_file(compound_xml_path: &std::path::Path) -> anyhow::Result<Root> {
    let xml = std::fs::read_to_string(compound_xml_path)?;
    parse(&xml)
}

pub fn parse(xml: &str) -> anyhow::Result<Root> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),

            Ok(Event::Eof) => {}

            Ok(Event::Start(tag)) => {
                if let b"compounddef" = tag.name().as_ref() {
                    let compound_def = parse_compound_def(&mut reader, tag)?;
                    return Ok(Root { compound_def });
                }
            }

            _ => (),
        }
    }
}

fn parse_compound_def(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<CompoundDef> {
    let id = xml::get_attribute_string(b"id", &start_tag)?;
    let mut compound_name = String::new();
    let mut brief_description = Description::default();
    let mut detailed_description = Description::default();
    let mut section_defs = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"compoundname" => {
                    compound_name = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = parse_description(reader, tag)?;
                }
                b"detaileddescription" => {
                    detailed_description = parse_description(reader, tag)?;
                }
                b"sectiondef" => {
                    section_defs.push(parse_section_def(reader, tag)?);
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"compounddef" {
                    return Ok(CompoundDef {
                        id,
                        compound_name,
                        brief_description,
                        detailed_description,
                        section_defs,
                    });
                }
            }
            _ => {}
        }
    }
}

fn parse_section_def(
    reader: &mut Reader<&[u8]>,
    tag: BytesStart<'_>,
) -> anyhow::Result<SectionDef> {
    let kind = xml::get_attribute_string(b"kind", &tag)?;
    let mut member_defs = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"memberdef" => {
                    let kind = xml::get_attribute_string(b"kind", &tag)?;
                    match kind.as_str() {
                        "enum" => {
                            member_defs.push(parse_enum_member_def(reader, tag)?);
                        }
                        "function" => {
                            member_defs.push(parse_function_member_def(reader, tag)?);
                        }
                        "variable" => {
                            member_defs.push(parse_variable_member_def(reader, tag)?);
                        }
                        _ => {
                            member_defs.push(parse_unknown_member_def(reader, tag, &kind)?);
                        }
                    }
                }
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing sectiondef: {tag_name:?}"
                    ))
                }
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"sectiondef" {
                    return Ok(SectionDef { kind, member_defs });
                }
            }
            _ => {}
        }
    }
}

fn parse_enum_member_def(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<MemberDef> {
    let id = xml::get_attribute_string(b"id", &start_tag)?;
    let mut name = String::new();
    let mut brief_description = Description::default();
    let mut detailed_description = Description::default();
    let mut values = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = parse_description(reader, tag)?;
                }
                b"detaileddescription" => {
                    detailed_description = parse_description(reader, tag)?;
                }
                b"enumvalue" => {
                    values.push(parse_enum_value(reader, tag)?);
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"memberdef" {
                    return Ok(MemberDef {
                        id,
                        name,
                        brief_description,
                        detailed_description,
                        kind: MemberDefKind::Enum { values },
                    });
                }
            }
            _ => {}
        }
    }
}

fn parse_function_member_def(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<MemberDef> {
    let id = xml::get_attribute_string(b"id", &start_tag)?;
    let mut name = String::new();
    let mut brief_description = Description::default();
    let mut detailed_description = Description::default();
    let mut params = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = parse_description(reader, tag)?;
                }
                b"detaileddescription" => {
                    detailed_description = parse_description(reader, tag)?;
                }
                b"param" => {
                    params.push(parse_param(reader, tag)?);
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"memberdef" {
                    return Ok(MemberDef {
                        id,
                        name,
                        brief_description,
                        detailed_description,
                        kind: MemberDefKind::Function { params },
                    });
                }
            }
            _ => {}
        }
    }
}

fn parse_param(reader: &mut Reader<&[u8]>, _tag: BytesStart<'_>) -> anyhow::Result<Param> {
    let mut type_ = None;
    let mut declname = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"type" => {
                    type_ = Some(parse_linked_text(reader, tag)?);
                }
                b"declname" => {
                    declname = xml::parse_text(reader)?;
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"param" {
                    return type_
                        .map(|type_| Param { type_, declname })
                        .ok_or_else(|| anyhow::anyhow!("Failed to find type for param"));
                }
            }
            _ => {}
        }
    }
}

fn parse_variable_member_def(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<MemberDef> {
    let id = xml::get_attribute_string(b"id", &start_tag)?;
    let mut name = String::new();
    let mut brief_description = Description::default();
    let mut detailed_description = Description::default();
    let mut values = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = parse_description(reader, tag)?;
                }
                b"detaileddescription" => {
                    detailed_description = parse_description(reader, tag)?;
                }
                b"enumvalue" => {
                    values.push(parse_enum_value(reader, tag)?);
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"memberdef" {
                    return Ok(MemberDef {
                        id,
                        name,
                        brief_description,
                        detailed_description,
                        kind: MemberDefKind::Variable,
                    });
                }
            }
            _ => {}
        }
    }
}

fn parse_enum_value(reader: &mut Reader<&[u8]>, _tag: BytesStart<'_>) -> anyhow::Result<EnumValue> {
    let mut name = String::new();
    let mut initializer = String::new();
    let mut brief_description = Description::default();
    let mut detailed_description = Description::default();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"initializer" => {
                    initializer = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = parse_description(reader, tag)?;
                }
                b"detaileddescription" => {
                    detailed_description = parse_description(reader, tag)?;
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"enumvalue" {
                    return Ok(EnumValue {
                        name,
                        initializer,
                        brief_description,
                        detailed_description,
                    });
                }
            }
            _ => {}
        }
    }
}

fn parse_unknown_member_def(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
    kind: &str,
) -> anyhow::Result<MemberDef> {
    let id = xml::get_attribute_string(b"id", &start_tag)?;
    let mut name = String::new();
    let mut brief_description = Description::default();
    let mut detailed_description = Description::default();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = parse_description(reader, tag)?;
                }
                b"detaileddescription" => {
                    detailed_description = parse_description(reader, tag)?;
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"memberdef" {
                    return Ok(MemberDef {
                        id,
                        name,
                        brief_description,
                        detailed_description,
                        kind: MemberDefKind::Unknown(kind.to_string()),
                    });
                }
            }
            _ => {}
        }
    }
}

pub fn parse_description(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<Description> {
    let mut content = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"para" => content.push(DescriptionType::Para(parse_para(reader, tag)?)),
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing description: {:?}",
                        String::from_utf8_lossy(tag_name),
                    ))
                }
            },
            Ok(Event::Text(text)) => content.push(DescriptionType::Text(
                String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
            )),
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(Description { content });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

fn parse_linked_text(
    reader: &mut Reader<&[u8]>,
    _tag: BytesStart<'_>,
) -> anyhow::Result<LinkedText> {
    match reader.read_event() {
        Ok(Event::Start(tag)) => match tag.name().as_ref() {
            b"ref" => {
                let id = xml::get_attribute_string(b"refid", &tag)?;
                Ok(LinkedText::Ref(RefText {
                    id,
                    text: xml::parse_text(reader)?,
                }))
            }
            tag_name => Err(anyhow!(
                "unexpected tag when parsing linked text: {tag_name:?}"
            )),
        },
        Ok(Event::Text(text)) => Ok(LinkedText::Text(
            String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
        )),
        event => {
            return Err(anyhow!(
                "unexpected event when parsing linked text: {:?}",
                event
            ))
        }
    }
}

pub fn parse_para(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<Vec<DocPara>> {
    let mut content = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"ref" => {
                    let id = xml::get_attribute_string(b"refid", &tag)?;
                    content.push(DocPara::Ref(RefText {
                        id,
                        text: xml::parse_text(reader)?,
                    }))
                }
                b"parameterlist" => {
                    content.push(DocPara::ParameterList(parse_parameter_list(reader, tag)?))
                }
                b"simplesect" => content.push(DocPara::SimpleSect(parse_simple_sect(reader, tag)?)),
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing para: {:?}",
                        String::from_utf8_lossy(tag_name),
                    ))
                }
            },
            Ok(Event::Text(text)) => content.push(DocPara::Text(
                String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
            )),
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(content);
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

pub fn parse_simple_sect(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<DocSimpleSect> {
    let mut paras = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"para" => paras.extend(parse_para(reader, tag)?),
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing {}: {:?}",
                        String::from_utf8_lossy(start_tag.name().into_inner()),
                        String::from_utf8_lossy(tag_name),
                    ))
                }
            },
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(DocSimpleSect { paras });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

pub fn parse_parameter_list(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<DocParamList> {
    let mut parameter_items = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"parameteritem" => parameter_items.push(parse_parameter_item(reader, tag)?),
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing {}: {:?}",
                        String::from_utf8_lossy(start_tag.name().into_inner()),
                        String::from_utf8_lossy(tag_name),
                    ))
                }
            },
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(DocParamList { parameter_items });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

pub fn parse_parameter_item(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<DocParamListItem> {
    let mut parameter_name_list = Vec::new();
    let mut parameter_description = Description::default();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"parameternamelist" => {
                    parameter_name_list.push(parse_parameter_name_list(reader, tag)?);
                }
                b"parameterdescription" => {
                    parameter_description = parse_description(reader, tag)?;
                }
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing {}: {:?}",
                        String::from_utf8_lossy(start_tag.name().into_inner()),
                        String::from_utf8_lossy(tag_name),
                    ))
                }
            },
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(DocParamListItem {
                        parameter_name_list,
                        parameter_description,
                    });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

pub fn parse_parameter_name_list(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<DocParamNameList> {
    let mut parameter_type = None;
    let mut parameter_name = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"parametertype" => {
                    parameter_type = Some(parse_parameter_type(reader, tag)?);
                }
                b"parametername" => {
                    parameter_name = Some(parse_parameter_name(reader, tag)?);
                }
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing {}: {:?}",
                        String::from_utf8_lossy(start_tag.name().into_inner()),
                        String::from_utf8_lossy(tag_name),
                    ))
                }
            },
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(DocParamNameList {
                        parameter_type,
                        parameter_name,
                    });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

fn parse_parameter_type(
    reader: &mut Reader<&[u8]>,
    _start_tag: BytesStart<'_>,
) -> anyhow::Result<DocParamType> {
    match reader.read_event() {
        Ok(Event::Start(tag)) => match tag.name().as_ref() {
            b"ref" => {
                let id = xml::get_attribute_string(b"refid", &tag)?;
                Ok(DocParamType::Ref(RefText {
                    id,
                    text: xml::parse_text(reader)?,
                }))
            }
            tag_name => Err(anyhow!(
                "unexpected tag when parsing linked text: {tag_name:?}"
            )),
        },
        Ok(Event::Text(text)) => Ok(DocParamType::Text(
            String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
        )),
        event => {
            return Err(anyhow!(
                "unexpected event when parsing linked text: {:?}",
                event
            ))
        }
    }
}

fn parse_parameter_name(
    reader: &mut Reader<&[u8]>,
    _start_tag: BytesStart<'_>,
) -> anyhow::Result<DocParamName> {
    match reader.read_event() {
        Ok(Event::Start(tag)) => match tag.name().as_ref() {
            b"ref" => {
                let id = xml::get_attribute_string(b"refid", &tag)?;
                Ok(DocParamName::Ref(RefText {
                    id,
                    text: xml::parse_text(reader)?,
                }))
            }
            tag_name => Err(anyhow!(
                "unexpected tag when parsing linked text: {tag_name:?}"
            )),
        },
        Ok(Event::Text(text)) => Ok(DocParamName::Text(
            String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
        )),
        event => {
            return Err(anyhow!(
                "unexpected event when parsing linked text: {:?}",
                event
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn test_parse_basic() {
        let result = parse(
            r#"
            <?xml version='1.0' encoding='UTF-8' standalone='no'?>
            <doxygen xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="compound.xsd" version="1.9.5" xml:lang="en-US">
              <compounddef id="class_nutshell" kind="class" language="C++" prot="public">
                <compoundname>Nutshell</compoundname>
              </compounddef>
            </doxygen>
            "#,
        );

        assert_eq!(
            result.unwrap(),
            Root {
                compound_def: CompoundDef {
                    id: "class_nutshell".to_string(),
                    compound_name: "Nutshell".to_string(),
                    brief_description: Description::default(),
                    detailed_description: Description::default(),
                    section_defs: Vec::new(),
                },
            }
        );
    }

    #[test]
    fn test_parse_nutshell() {
        let result = parse(include_str!("compound/class_nutshell.xml"));
        insta::assert_debug_snapshot!(result.unwrap());
    }
}
