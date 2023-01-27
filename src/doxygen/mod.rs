pub mod compound;
pub mod index;

use super::nodes::{Node, SignatureType};
use compound::{DescriptionType, DocParaType, SectionDef};

pub fn render_class_compound(compound: compound::Root) -> Vec<Node> {
    let compound_def = compound.compound_def;

    let mut content_nodes = Vec::new();
    content_nodes.append(&mut render_description(compound_def.detailed_description));
    content_nodes.append(
        &mut compound_def
            .section_defs
            .into_iter()
            .map(render_section_def)
            .collect(),
    );
    let content = Node::DescContent(content_nodes);

    vec![Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(vec![
                Node::DescSignatureKeyword("class".to_string()),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(
                    compound_def.compound_name,
                ))),
            ])],
        )],
        Box::new(content),
    )]
}

pub fn render_section_def(_section_def: SectionDef) -> Node {
    Node::Container(Vec::new())
}

pub fn render_description(description: compound::Description) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in description.content {
        match entry {
            DescriptionType::Para(content) => nodes.push(Node::Paragraph(render_para(content))),
            DescriptionType::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    nodes
}

pub fn render_para(content: Vec<DocParaType>) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in content {
        match entry {
            DocParaType::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    nodes
}
