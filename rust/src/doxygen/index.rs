pub mod elements;

use anyhow::anyhow;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

use crate::xml;

use elements::*;

pub fn parse_file(index_xml_path: &std::path::Path) -> anyhow::Result<DoxygenType> {
    tracing::info!("Reading {}", index_xml_path.display());
    let xml = std::fs::read_to_string(index_xml_path)?;
    parse(&xml)
}

pub fn parse(xml: &str) -> anyhow::Result<DoxygenType> {
    let mut reader = Reader::from_str(xml);

    let mut compound = Vec::new();

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(tag)) => {
                if let b"compound" = tag.name().as_ref() {
                    compound.push(parse_compound(&mut reader, tag)?);
                }
            }
            _ => (),
        }
    }

    Ok(DoxygenType { compound })
}

fn parse_compound(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<CompoundType> {
    let ref_id = xml::get_attribute_string(b"refid", &start_tag)?;
    let kind = xml::get_attribute_enum::<CompoundKind>(b"kind", &start_tag)?;

    let mut name = String::new();
    let mut member = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"member" => {
                    member.push(parse_member(reader, tag)?);
                }
                _ => return Err(anyhow!("unrecognised element in compound")),
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"compound" {
                    return Ok(CompoundType {
                        ref_id,
                        name,
                        kind,
                        member,
                    });
                }
            }
            Ok(Event::Text(_)) => {}
            _ => return Err(anyhow!("Expected Event::Start")),
        }
    }
}

fn parse_member(
    reader: &mut Reader<&[u8]>,
    start_tag: BytesStart<'_>,
) -> anyhow::Result<MemberType> {
    let ref_id = xml::get_attribute_string(b"refid", &start_tag)?;
    let kind = xml::get_attribute_enum::<MemberKind>(b"kind", &start_tag)?;
    let mut name = String::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                element => {
                    return Err(anyhow!(
                        "unrecognised element in member: {}",
                        String::from_utf8_lossy(element)
                    ))
                }
            },
            Ok(Event::Text(_)) => {}
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"member" {
                    return Ok(MemberType { ref_id, kind, name });
                }
            }

            _ => return Err(anyhow!("Expected Event::Start")),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(
            r#"<?xml version='1.0' encoding='UTF-8' standalone='no'?>
<doxygenindex xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="index.xsd" version="1.9.5" xml:lang="en-US">
  <compound refid="class_nutshell" kind="class"><name>Nutshell</name>
    <member refid="class_nutshell_1ae42034231cf912d095d57cbeed6cda79" kind="enum"><name>Tool</name></member>
  </compound>
  <compound refid="nutshell_8h" kind="file"><name>nutshell.h</name>
  </compound>
</doxygenindex>"#,
        );

        assert_eq!(
            result.unwrap(),
            DoxygenType {
                compound: vec![
                    CompoundType {
                        ref_id: "class_nutshell".to_string(),
                        name: "Nutshell".to_string(),
                        kind: CompoundKind::Class,
                        member: vec![MemberType {
                            ref_id: "class_nutshell_1ae42034231cf912d095d57cbeed6cda79".to_string(),
                            name: "Tool".to_string(),
                            kind: MemberKind::Enum,
                        }]
                    },
                    CompoundType {
                        ref_id: "nutshell_8h".to_string(),
                        name: "nutshell.h".to_string(),
                        kind: CompoundKind::File,
                        member: vec![]
                    }
                ]
            }
        );
    }
}
