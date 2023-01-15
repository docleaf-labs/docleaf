use anyhow::anyhow;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use crate::xml;

#[derive(Debug, PartialEq)]
pub struct Root {
    pub compound_def: CompoundDef,
}

#[derive(Debug, PartialEq)]
pub struct CompoundDef {
    pub compound_name: String,
    pub brief_description: Description,
    pub detailed_description: Description,
}

#[derive(Debug, Default, PartialEq)]
pub struct Description {
    content: Vec<DescriptionType>,
}

#[derive(Debug, PartialEq)]
pub enum DescriptionType {
    Para(Vec<DocParaType>),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum DocParaType {
    Text(String),
}

pub fn parse_file(compound_xml_path: &std::path::Path) -> anyhow::Result<Root> {
    let xml = std::fs::read_to_string(compound_xml_path)?;
    parse(&xml)
}

pub fn parse(xml: &str) -> anyhow::Result<Root> {
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut txt = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => {}

            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"compounddef" => {
                    let compound_def_contents = parse_compound_def(&mut reader)?;

                    return Ok(Root {
                        compound_def: CompoundDef {
                            compound_name: compound_def_contents.name,
                            brief_description: compound_def_contents.brief_description,
                            detailed_description: compound_def_contents.detailed_description,
                        },
                    });
                }
                _ => {}
            },
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
    }
}

struct CompoundDefContents {
    name: String,
    brief_description: Description,
    detailed_description: Description,
}

fn parse_compound_def(reader: &mut Reader<&[u8]>) -> anyhow::Result<CompoundDefContents> {
    let mut name = String::new();
    let mut brief_description = Description::default();
    let mut detailed_description = Description::default();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"compoundname" => {
                    name = xml::parse_text(reader)?;
                }
                b"briefdescription" => {
                    brief_description = parse_description(reader, b"briefdescription")?;
                }
                b"detaileddescription" => {
                    detailed_description = parse_description(reader, b"detaileddescription")?;
                }
                _ => {}
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"compounddef" {
                    return Ok(CompoundDefContents {
                        name,
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
    tag_name: &[u8],
) -> anyhow::Result<Description> {
    let mut content = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"para" => content.push(DescriptionType::Para(parse_para(reader)?)),
                _ => {}
            },
            Ok(Event::Text(text)) => content.push(DescriptionType::Text(
                String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
            )),
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == tag_name {
                    return Ok(Description { content });
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
        }
    }
}

pub fn parse_para(reader: &mut Reader<&[u8]>) -> anyhow::Result<Vec<DocParaType>> {
    let mut content = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                _ => {}
            },
            Ok(Event::Text(text)) => content.push(DocParaType::Text(
                String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))?,
            )),
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"para" {
                    return Ok(content);
                }
            }
            event => return Err(anyhow!("unexpected event: {:?}", event)),
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
                    compound_name: "Nutshell".to_string(),
                    brief_description: Description::default(),
                    detailed_description: Description::default(),
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
