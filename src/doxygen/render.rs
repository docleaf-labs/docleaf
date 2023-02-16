use crate::nodes::{Node, SignatureType};

use crate::doxygen::compound::elements as e;

pub fn render_class_compound(compound: e::DoxygenType) -> Vec<Node> {
    let Some(compound_def) = compound.compound_def else {
        return Vec::new();
    };

    let mut content_nodes = Vec::new();

    if let Some(description) = compound_def.brief_description {
        content_nodes.append(&mut render_description(description));
    }
    if let Some(description) = compound_def.detailed_description {
        content_nodes.append(&mut render_description(description));
    }
    content_nodes.append(
        &mut compound_def
            .section_defs
            .into_iter()
            .map(render_section_def)
            .collect(),
    );
    let content = Node::DescContent(content_nodes);

    let ids = compound_def.id.clone();
    let names = compound_def.id;

    vec![Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(vec![
                Node::Target { ids, names },
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

pub fn render_section_def(section_def: e::SectionDefType) -> Node {
    let mut content_nodes = vec![Node::Rubric(vec![Node::Text(section_title(
        &section_def.kind,
    ))])];
    content_nodes.append(
        &mut section_def
            .member_defs
            .into_iter()
            .map(render_member_def)
            .collect(),
    );

    Node::Container(content_nodes)
}

fn section_title(section_kind: &e::DoxSectionKind) -> String {
    match section_kind {
        e::DoxSectionKind::UserDefined => "UserDefined".to_string(),
        e::DoxSectionKind::PublicType => "PublicType".to_string(),
        e::DoxSectionKind::PublicFunc => "PublicFunc".to_string(),
        e::DoxSectionKind::PublicAttrib => "PublicAttrib".to_string(),
        e::DoxSectionKind::PublicSlot => "PublicSlot".to_string(),
        e::DoxSectionKind::Signal => "Signal".to_string(),
        e::DoxSectionKind::DcopFunc => "DcopFunc".to_string(),
        e::DoxSectionKind::Property => "Property".to_string(),
        e::DoxSectionKind::Event => "Event".to_string(),
        e::DoxSectionKind::PublicStaticFunc => "PublicStaticFunc".to_string(),
        e::DoxSectionKind::PublicStaticAttrib => "PublicStaticAttrib".to_string(),
        e::DoxSectionKind::ProtectedType => "ProtectedType".to_string(),
        e::DoxSectionKind::ProtectedFunc => "ProtectedFunc".to_string(),
        e::DoxSectionKind::ProtectedAttrib => "ProtectedAttrib".to_string(),
        e::DoxSectionKind::ProtectedSlot => "ProtectedSlot".to_string(),
        e::DoxSectionKind::ProtectedStaticFunc => "ProtectedStaticFunc".to_string(),
        e::DoxSectionKind::ProtectedStaticAttrib => "ProtectedStaticAttrib".to_string(),
        e::DoxSectionKind::PackageType => "PackageType".to_string(),
        e::DoxSectionKind::PackageFunc => "PackageFunc".to_string(),
        e::DoxSectionKind::PackageAttrib => "PackageAttrib".to_string(),
        e::DoxSectionKind::PackageStaticFunc => "PackageStaticFunc".to_string(),
        e::DoxSectionKind::PackageStaticAttrib => "PackageStaticAttrib".to_string(),
        e::DoxSectionKind::PrivateType => "PrivateType".to_string(),
        e::DoxSectionKind::PrivateFunc => "PrivateFunc".to_string(),
        e::DoxSectionKind::PrivateAttrib => "PrivateAttrib".to_string(),
        e::DoxSectionKind::PrivateSlot => "PrivateSlot".to_string(),
        e::DoxSectionKind::PrivateStaticFunc => "PrivateStaticFunc".to_string(),
        e::DoxSectionKind::PrivateStaticAttrib => "PrivateStaticAttrib".to_string(),
        e::DoxSectionKind::Friend => "Friend".to_string(),
        e::DoxSectionKind::Related => "Related".to_string(),
        e::DoxSectionKind::Define => "Define".to_string(),
        e::DoxSectionKind::Prototype => "Prototype".to_string(),
        e::DoxSectionKind::Typedef => "Typedef".to_string(),
        e::DoxSectionKind::Enum => "Enum".to_string(),
        e::DoxSectionKind::Func => "Func".to_string(),
        e::DoxSectionKind::Var => "Var".to_string(),
    }
}

pub fn render_member_def(member_def: e::MemberDefType) -> Node {
    let name = member_kind_name(&member_def.kind);
    let mut content_nodes = Vec::new();

    if let Some(description) = member_def.brief_description {
        content_nodes.append(&mut render_description(description));
    }
    if let Some(description) = member_def.detailed_description {
        content_nodes.append(&mut render_description(description));
    }

    let ids = member_def.id.clone();
    let names = member_def.id;

    let mut signature_line = vec![
        Node::Target { ids, names },
        Node::DescSignatureKeyword(name),
        Node::DescSignatureSpace,
        Node::DescName(Box::new(Node::DescSignatureName(member_def.name))),
    ];

    match member_def.kind {
        /*
            DoxMemberKind::Enum => {
                content_nodes.append(
                    &mut member_def
                        .values
                        .into_iter()
                        .map(render_enum_value)
                        .collect(),
                );
            }
            DoxMemberKind::Function => {
                let parameter_list_items = member_def
                    .params
                    .into_iter()
                    .map(|param| {
                        let type_ = match param.type_ {
                            LinkedTextType::Ref(ref_text) => Node::Reference {
                                internal: true,
                                refid: ref_text.id,
                                children: vec![Node::DescSignatureName(ref_text.text)],
                            },
                            LinkedTextType::Text(text_) => Node::DescSignatureName(text_),
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
            DoxMemberKind::Variable => {}
        */
        _ => {}
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

fn member_kind_name(member_kind: &e::DoxMemberKind) -> String {
    match member_kind {
        e::DoxMemberKind::Define => "Define".to_string(),
        e::DoxMemberKind::Property => "Property".to_string(),
        e::DoxMemberKind::Event => "Event".to_string(),
        e::DoxMemberKind::Variable => "Variable".to_string(),
        e::DoxMemberKind::Typedef => "Typedef".to_string(),
        e::DoxMemberKind::Enum => "Enum".to_string(),
        e::DoxMemberKind::Function => "Function".to_string(),
        e::DoxMemberKind::Signal => "Signal".to_string(),
        e::DoxMemberKind::Prototype => "Prototype".to_string(),
        e::DoxMemberKind::Friend => "Friend".to_string(),
        e::DoxMemberKind::Dcop => "Dcop".to_string(),
        e::DoxMemberKind::Slot => "Slot".to_string(),
        e::DoxMemberKind::Interface => "Interface".to_string(),
        e::DoxMemberKind::Service => "Service".to_string(),
    }
}

pub fn render_enum_value(enum_value: e::EnumValueType) -> Node {
    let mut content_nodes = Vec::new();

    if let Some(description) = enum_value.brief_description {
        content_nodes.append(&mut render_description(description));
    }
    if let Some(description) = enum_value.detailed_description {
        content_nodes.append(&mut render_description(description));
    }

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

pub fn render_description(description: e::DescriptionType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in description.content {
        match entry {
            e::DescriptionTypeItem::Para(content) => {
                nodes.push(Node::Paragraph(render_para(content)))
            }
            e::DescriptionTypeItem::Text(text) => nodes.push(Node::Text(text)),
            _ => {}
        }
    }

    nodes
}

pub fn render_para(element: e::DocParaType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in element.content {
        match entry {
            // TODO: Render list
            e::DocParaTypeItem::DocCmdGroup(e::DocCmdGroup::ParameterList(_)) => {}
            // TODO: Handle title & paragraph block
            e::DocParaTypeItem::DocCmdGroup(e::DocCmdGroup::Simplesect(e::DocSimpleSectType {
                para: _,
            })) => {} // nodes.extend(render_para(para)),
            e::DocParaTypeItem::DocCmdGroup(e::DocCmdGroup::DocTitleCmdGroup(
                e::DocTitleCmdGroup::Ref(_ref_text),
            )) => nodes.push(Node::Text(
                /*ref_text.content*/ "ref text content".to_string(),
            )),
            e::DocParaTypeItem::Text(text) => nodes.push(Node::Text(text)),
            _ => {}
        }
    }

    nodes
}
