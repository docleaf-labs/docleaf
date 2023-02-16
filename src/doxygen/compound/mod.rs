pub mod elements;

use anyhow::anyhow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use vec1::Vec1;

use crate::doxygen::compound::elements::*;
use crate::xml;

pub fn parse_file(compound_xml_path: &std::path::Path) -> anyhow::Result<DoxygenType> {
    let xml = std::fs::read_to_string(compound_xml_path)?;
    parse(&xml)
}

pub fn parse(xml: &str) -> anyhow::Result<DoxygenType> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),

            Ok(Event::Eof) => {}

            Ok(Event::Start(tag)) => {
                if let b"compounddef" = tag.name().as_ref() {
                    let compound_def = parse_compound_def(&mut reader, tag)?;
                    return Ok(DoxygenType {
                        compound_def: Some(compound_def),
                    });
                }
            }

            _ => (),
        }
    }
}

fn parse_compound_def(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<CompoundDefType> {
    let id = xml::get_attribute_string(b"id", &start_tag)?;
    let mut compound_name = String::new();
    let mut brief_description = None;
    let mut detailed_description = None;
    let mut section_defs = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"compoundname" => {
                    compound_name = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = Some(parse_description(reader, tag)?);
                }
                b"detaileddescription" => {
                    detailed_description = Some(parse_description(reader, tag)?);
                }
                b"sectiondef" => {
                    section_defs.push(parse_section_def(reader, tag)?);
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"compounddef" {
                    return Ok(CompoundDefType {
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
) -> anyhow::Result<SectionDefType> {
    let kind = xml::get_attribute_enum::<DoxSectionKind>(b"kind", &tag)?;
    let mut member_defs = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"memberdef" => member_defs.push(parse_member_def(reader, tag)?),
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing sectiondef: {tag_name:?}"
                    ))
                }
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"sectiondef" {
                    return Ok(SectionDefType {
                        kind,
                        member_defs: Vec1::try_from_vec(member_defs)?,
                    });
                }
            }
            _ => {}
        }
    }
}

fn parse_member_def(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<MemberDefType> {
    let id = xml::get_attribute_string(b"id", &start_tag)?;
    let kind = xml::get_attribute_enum::<DoxMemberKind>(b"kind", &start_tag)?;

    let mut name = String::new();
    let mut brief_description = None;
    let mut detailed_description = None;
    let mut values = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = Some(parse_description(reader, tag)?);
                }
                b"detaileddescription" => {
                    detailed_description = Some(parse_description(reader, tag)?);
                }
                b"enumvalue" => {
                    values.push(parse_enum_value(reader, tag)?);
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"memberdef" {
                    return Ok(MemberDefType {
                        id,
                        name,
                        kind,
                        brief_description,
                        detailed_description,
                    });
                }
            }
            _ => {}
        }
    }
}

fn parse_param(reader: &mut Reader<&[u8]>, _tag: BytesStart<'_>) -> anyhow::Result<ParamType> {
    let mut type_ = None;
    let mut declname = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"type" => {
                    type_ = Some(parse_linked_text(reader, tag)?);
                }
                b"declname" => {
                    declname = Some(xml::parse_text(reader)?);
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"param" {
                    return Ok(ParamType { type_, declname });
                }
            }
            _ => {}
        }
    }
}

fn parse_enum_value(
    reader: &mut Reader<&[u8]>,
    _tag: BytesStart<'_>,
) -> anyhow::Result<EnumValueType> {
    let mut name = String::new();
    let mut initializer = None;
    let mut brief_description = None;
    let mut detailed_description = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"initializer" => {
                    initializer = Some(parse_linked_text(reader, tag)?);
                }
                b"briefdescription" => {
                    brief_description = Some(parse_description(reader, tag)?);
                }
                b"detaileddescription" => {
                    detailed_description = Some(parse_description(reader, tag)?);
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"enumvalue" {
                    return Ok(EnumValueType {
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

pub fn parse_description(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<DescriptionType> {
    let mut content = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"para" => content.push(DescriptionTypeItem::Para(parse_para(reader, tag)?)),
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing description: {:?}",
                        String::from_utf8_lossy(tag_name),
                    ))
                }
            },
            Ok(Event::Text(text)) => content.push(DescriptionTypeItem::Text(
                String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
            )),
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(DescriptionType { content });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

fn parse_linked_text(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<LinkedTextType> {
    let mut content = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"ref" => {
                    let ref_id = xml::get_attribute_string(b"refid", &tag)?;
                    content.push(LinkedTextTypeItem::Ref(RefTextType {
                        ref_id,
                        content: xml::parse_text(reader)?,
                    }))
                }
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing linked text: {tag_name:?}"
                    ))
                }
            },
            Ok(Event::Text(text)) => content.push(LinkedTextTypeItem::Text(
                String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
            )),
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(LinkedTextType { content });
                }
            }
            event => {
                return Err(anyhow!(
                    "unexpected event when parsing linked text: {:?}",
                    event
                ))
            }
        }
    }
}

pub fn parse_para(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<DocParaType> {
    let mut content = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"ref" => {
                    let ref_id = xml::get_attribute_string(b"refid", &tag)?;
                    content.push(DocParaTypeItem::DocCmdGroup(DocCmdGroup::DocTitleCmdGroup(
                        DocTitleCmdGroup::Ref(DocRefTextType {
                            ref_id,
                            content: Vec::new(),
                        }),
                    )))
                }
                b"parameterlist" => content.push(DocParaTypeItem::DocCmdGroup(
                    DocCmdGroup::ParameterList(parse_parameter_list(reader, tag)?),
                )),
                b"simplesect" => content.push(DocParaTypeItem::DocCmdGroup(
                    DocCmdGroup::Simplesect(parse_simple_sect(reader, tag)?),
                )),
                tag_name => {
                    return Err(anyhow!(
                        "unexpected tag when parsing para: {:?}",
                        String::from_utf8_lossy(tag_name),
                    ))
                }
            },
            Ok(Event::Text(text)) => content.push(DocParaTypeItem::Text(
                String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
            )),
            Ok(Event::End(tag)) => {
                if tag.name() == start_tag.name() {
                    return Ok(DocParaType { content });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

pub fn parse_simple_sect(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<DocSimpleSectType> {
    let mut para = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"para" => para.push(parse_para(reader, tag)?),
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
                    return Ok(DocSimpleSectType {
                        para: Vec1::try_from_vec(para)?,
                    });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

pub fn parse_parameter_list(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<DocParamListType> {
    let mut parameter_item = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"parameteritem" => parameter_item.push(parse_parameter_item(reader, tag)?),
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
                    return Ok(DocParamListType { parameter_item });
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
    let mut parameter_description = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"parameternamelist" => {
                    parameter_name_list.push(parse_parameter_name_list(reader, tag)?);
                }
                b"parameterdescription" => {
                    parameter_description = Some(parse_description(reader, tag)?);
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
            DoxygenType {
                compound_def: CompoundDefType {
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
