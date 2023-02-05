pub mod compound;
pub mod index;

use super::nodes::{Node, SignatureType};
use compound::{DescriptionType, DocPara, DocSimpleSect};

pub fn render_class_compound(compound: compound::Root) -> Vec<Node> {
    let compound_def = compound.compound_def;

    let mut content_nodes = Vec::new();
    content_nodes.append(&mut render_description(compound_def.brief_description));
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

pub fn render_section_def(section_def: compound::SectionDef) -> Node {
    let mut content_nodes = vec![Node::Rubric(vec![Node::Text(section_def.kind)])];
    content_nodes.append(
        &mut section_def
            .member_defs
            .into_iter()
            .map(render_member_def)
            .collect(),
    );

    Node::Container(content_nodes)
}

pub fn render_member_def(member_def: compound::MemberDef) -> Node {
    let name = member_def.kind.name();
    let mut content_nodes = Vec::new();
    content_nodes.append(&mut render_description(member_def.brief_description));
    content_nodes.append(&mut render_description(member_def.detailed_description));

    let mut signature_line = vec![
        Node::DescSignatureKeyword(name),
        Node::DescSignatureSpace,
        Node::DescName(Box::new(Node::DescSignatureName(member_def.name))),
    ];

    match member_def.kind {
        compound::MemberDefKind::Enum { values } => {
            content_nodes.append(&mut values.into_iter().map(render_enum_value).collect());
        }
        compound::MemberDefKind::Function { params } => {
            let parameter_list_items = params
                .into_iter()
                .map(|param| {
                    let type_ = match param.type_ {
                        compound::LinkedText::Ref(ref_text) => {
                            Node::DescSignatureName(ref_text.text)
                        }
                        compound::LinkedText::Text(text_) => Node::DescSignatureName(text_),
                    };

                    Node::DescParameter(vec![
                        type_,
                        Node::DescSignatureSpace,
                        Node::DescSignatureName(param.declname),
                    ])
                })
                .collect();
            signature_line.push(Node::DescParameterList(parameter_list_items));
        }
        compound::MemberDefKind::Variable => {}
        compound::MemberDefKind::Unknown(_) => {}
    };

    let content = Node::DescContent(content_nodes);

    Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(signature_line)],
        )],
        Box::new(content),
    )
}

pub fn render_enum_value(enum_value: compound::EnumValue) -> Node {
    let mut content_nodes = Vec::new();
    content_nodes.append(&mut render_description(enum_value.brief_description));
    content_nodes.append(&mut render_description(enum_value.detailed_description));
    let content = Node::DescContent(content_nodes);
    Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(vec![Node::DescName(Box::new(
                Node::DescSignatureName(enum_value.name),
            ))])],
        )],
        Box::new(content),
    )
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

pub fn render_para(content: Vec<DocPara>) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in content {
        match entry {
            // TODO: Render list
            DocPara::ParameterList(_) => {}
            // TODO: Handle title & paragraph block
            DocPara::SimpleSect(DocSimpleSect { paras }) => nodes.extend(render_para(paras)),
            DocPara::Ref(ref_text) => nodes.push(Node::Text(ref_text.text)),
            DocPara::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    nodes
}
