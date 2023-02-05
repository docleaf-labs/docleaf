use anyhow::anyhow;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use crate::xml;

#[derive(Debug, PartialEq)]
pub struct Root {
    pub compounds: Vec<Compound>,
}

#[derive(Debug, PartialEq)]
pub struct Compound {
    pub refid: String,
    pub name: String,
    pub kind: String,
    pub members: Vec<Member>,
}

#[derive(Debug, PartialEq)]
pub struct Member {
    refid: String,
    name: String,
}

pub fn parse_file(index_xml_path: &std::path::Path) -> anyhow::Result<Root> {
    let xml = std::fs::read_to_string(index_xml_path)?;
    parse(&xml)
}

pub fn parse(xml: &str) -> anyhow::Result<Root> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut txt = Vec::new();

    let mut compounds = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(tag)) => {
                if let b"compound" = tag.name().as_ref() {
                    let refid_attr = xml::get_attribute(b"refid", &tag)?;
                    let kind_attr = xml::get_attribute(b"kind", &tag)?;

                    let compound_contents = parse_compound(&mut reader)?;

                    compounds.push(Compound {
                        refid: String::from_utf8(refid_attr.value.into_owned())?,
                        name: compound_contents.name,
                        kind: String::from_utf8(kind_attr.value.into_owned())?,
                        members: compound_contents.members,
                    })
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
    }

    Ok(Root { compounds })
}

struct CompoundContents {
    name: String,
    members: Vec<Member>,
}

fn parse_compound(reader: &mut Reader<&[u8]>) -> anyhow::Result<CompoundContents> {
    let mut name = String::new();
    let mut members = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                b"name" => {
                    name = xml::parse_text(reader)?;
                }
                b"member" => {
                    let member_contents = parse_member(reader)?;
                    let refid_attr = xml::get_attribute(b"refid", &tag)?;
                    members.push(Member {
                        refid: String::from_utf8(refid_attr.value.into_owned())?,
                        name: member_contents.name,
                    });
                }
                _ => return Err(anyhow!("unrecognised element in compound")),
            },
            Ok(Event::End(tag)) => {
                if tag.local_name().as_ref() == b"compound" {
                    return Ok(CompoundContents { name, members });
                }
            }
            _ => return Err(anyhow!("Expected Event::Start")),
        }
    }
}

struct MemberContents {
    name: String,
}

fn parse_member(reader: &mut Reader<&[u8]>) -> anyhow::Result<MemberContents> {
    let name;
    match reader.read_event() {
        Ok(Event::Start(tag)) => match tag.name().as_ref() {
            b"name" => {
                name = xml::parse_text(reader)?;
            }
            _ => return Err(anyhow!("unrecognised element in member")),
        },
        _ => return Err(anyhow!("Expected Event::Start")),
    }

    Ok(MemberContents { name })
}

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_ne};

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
            Root {
                compounds: vec![
                    Compound {
                        refid: "class_nutshell".to_string(),
                        name: "Nutshell".to_string(),
                        kind: "class".to_string(),
                        members: vec![Member {
                            refid: "class_nutshell_1ae42034231cf912d095d57cbeed6cda79".to_string(),
                            name: "Tool".to_string(),
                        }]
                    },
                    Compound {
                        refid: "nutshell_8h".to_string(),
                        name: "nutshell.h".to_string(),
                        kind: "file".to_string(),
                        members: vec![]
                    }
                ]
            }
        );
    }
}
