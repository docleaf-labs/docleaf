use crate::nodes::{Node, SignatureType};

use crate::doxygen::compound::elements as e;

pub fn render_compound(root: e::DoxygenType) -> Vec<Node> {
    let Some(compound_def) = root.compound_def else {
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

    let kind = render_compound_kind(compound_def.kind);

    vec![Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(vec![
                Node::Target { ids, names },
                Node::DescSignatureKeyword(vec![Node::Text(kind.to_string())]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(
                    compound_def.compound_name,
                ))),
            ])],
        )],
        Box::new(content),
    )]
}

fn render_compound_kind(kind: e::DoxCompoundKind) -> &'static str {
    match kind {
        e::DoxCompoundKind::Class => "class",
        e::DoxCompoundKind::Struct => "struct",
        e::DoxCompoundKind::Union => "union",
        e::DoxCompoundKind::Interface => "interface",
        e::DoxCompoundKind::Protocol => "protocol",
        e::DoxCompoundKind::Category => "category",
        e::DoxCompoundKind::Exception => "exception",
        e::DoxCompoundKind::Service => "service",
        e::DoxCompoundKind::Singleton => "singleton",
        e::DoxCompoundKind::Module => "module",
        e::DoxCompoundKind::Type => "type",
        e::DoxCompoundKind::File => "file",
        e::DoxCompoundKind::Namespace => "namespace",
        e::DoxCompoundKind::Group => "group",
        e::DoxCompoundKind::Page => "page",
        e::DoxCompoundKind::Example => "example",
        e::DoxCompoundKind::Dir => "dir",
        e::DoxCompoundKind::Concept => "concept",
    }
}

pub fn render_member(root: e::DoxygenType, member_ref_id: &str) -> Vec<Node> {
    let Some(compound_def) = root.compound_def else {
        return Vec::new();
    };

    let member_def = compound_def
        .section_defs
        .into_iter()
        .find_map(|section_def| {
            section_def
                .member_defs
                .into_iter()
                .find(|member_def| member_def.id == member_ref_id)
        });

    match member_def {
        Some(member_def) => {
            vec![render_member_def(member_def)]
        }
        None => {
            vec![]
        }
    }
}

fn render_section_def(section_def: e::SectionDefType) -> Node {
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
        e::DoxSectionKind::PublicType => "Public Types".to_string(),
        e::DoxSectionKind::PublicFunc => "Public Functions".to_string(),
        e::DoxSectionKind::PublicAttrib => "Public Attributes".to_string(),
        e::DoxSectionKind::PublicSlot => "PublicSlot".to_string(),
        e::DoxSectionKind::Signal => "Signal".to_string(),
        e::DoxSectionKind::DcopFunc => "DcopFunc".to_string(),
        e::DoxSectionKind::Property => "Property".to_string(),
        e::DoxSectionKind::Event => "Event".to_string(),
        e::DoxSectionKind::PublicStaticFunc => "PublicStaticFunc".to_string(),
        e::DoxSectionKind::PublicStaticAttrib => "Public Static Attributes".to_string(),
        e::DoxSectionKind::ProtectedType => "ProtectedType".to_string(),
        e::DoxSectionKind::ProtectedFunc => "ProtectedFunc".to_string(),
        e::DoxSectionKind::ProtectedAttrib => "Protected Attributes".to_string(),
        e::DoxSectionKind::ProtectedSlot => "ProtectedSlot".to_string(),
        e::DoxSectionKind::ProtectedStaticFunc => "ProtectedStaticFunc".to_string(),
        e::DoxSectionKind::ProtectedStaticAttrib => "ProtectedStatic Attributes".to_string(),
        e::DoxSectionKind::PackageType => "PackageType".to_string(),
        e::DoxSectionKind::PackageFunc => "PackageFunc".to_string(),
        e::DoxSectionKind::PackageAttrib => "Package Attributes".to_string(),
        e::DoxSectionKind::PackageStaticFunc => "PackageStaticFunc".to_string(),
        e::DoxSectionKind::PackageStaticAttrib => "PackageStatic Attributes".to_string(),
        e::DoxSectionKind::PrivateType => "PrivateType".to_string(),
        e::DoxSectionKind::PrivateFunc => "PrivateFunc".to_string(),
        e::DoxSectionKind::PrivateAttrib => "Private Attributes".to_string(),
        e::DoxSectionKind::PrivateSlot => "PrivateSlot".to_string(),
        e::DoxSectionKind::PrivateStaticFunc => "PrivateStaticFunc".to_string(),
        e::DoxSectionKind::PrivateStaticAttrib => "PrivateStatic Attributes".to_string(),
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

    let signature_line;

    match member_def.kind {
        e::DoxMemberKind::Enum => {
            signature_line = vec![
                Node::Target { ids, names },
                Node::DescSignatureKeyword(vec![Node::Text(name)]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(member_def.name))),
            ];

            content_nodes.append(
                &mut member_def
                    .enumvalue
                    .into_iter()
                    .map(render_enum_value)
                    .collect(),
            );
        }
        e::DoxMemberKind::Function => {
            let parameter_list_items = member_def
                .param
                .into_iter()
                .map(|param| {
                    let mut param_contents = Vec::new();

                    match (param.type_, param.declname) {
                        (Some(type_), Some(declname)) => {
                            param_contents.append(&mut render_linked_text_type(type_));
                            param_contents.push(Node::DescSignatureSpace);
                            param_contents.push(Node::DescSignatureName(declname));
                        }
                        (Some(type_), None) => {
                            param_contents.append(&mut render_linked_text_type(type_));
                        }
                        (None, Some(declname)) => {
                            param_contents.push(Node::DescSignatureName(declname));
                        }
                        (None, None) => {}
                    };

                    Node::DescParameter(param_contents)
                })
                .collect();

            match member_def.type_ {
                Some(type_) => {
                    signature_line = vec![
                        Node::Target { ids, names },
                        Node::DescSignatureKeyword(render_linked_text_type(type_)),
                        Node::DescSignatureSpace,
                        Node::DescName(Box::new(Node::DescSignatureName(member_def.name))),
                        Node::DescParameterList(parameter_list_items),
                    ];
                }
                None => {
                    signature_line = vec![
                        Node::Target { ids, names },
                        Node::DescName(Box::new(Node::DescSignatureName(member_def.name))),
                        Node::DescParameterList(parameter_list_items),
                    ];
                }
            }
        }
        _ => {
            signature_line = vec![
                Node::Target { ids, names },
                Node::DescSignatureKeyword(vec![Node::Text(name)]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(member_def.name))),
            ];
        }
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
        e::DoxMemberKind::Enum => "enum".to_string(),
        e::DoxMemberKind::Function => "function".to_string(),
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
                para,
            })) => nodes.append(
                &mut para
                    .into_iter()
                    .flat_map(|para| render_para(para))
                    .collect(),
            ),
            e::DocParaTypeItem::DocCmdGroup(element) => nodes.push(render_doc_cmd_group(element)),
            e::DocParaTypeItem::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    nodes
}

fn render_doc_cmd_group(element: e::DocCmdGroup) -> Node {
    match element {
        e::DocCmdGroup::DocTitleCmdGroup(element) => render_doc_title_cmd_group(element),
        // TODO: Change to panic
        _ => Node::Unknown,
    }
}

fn render_linked_text_type(linked_text_type: e::LinkedTextType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in linked_text_type.content {
        match entry {
            e::LinkedTextTypeItem::Ref(content) => nodes.push(render_ref_text_type(content)),
            e::LinkedTextTypeItem::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    nodes
}

fn render_ref_text_type(ref_text_type: e::RefTextType) -> Node {
    Node::Reference {
        internal: true,
        refid: ref_text_type.ref_id,
        children: vec![Node::DescSignatureName(ref_text_type.content)],
    }
}

fn render_doc_ref_text_type(doc_ref_text_type: e::DocRefTextType) -> Node {
    let mut nodes = Vec::new();

    for entry in doc_ref_text_type.content {
        match entry {
            e::DocRefTextTypeItem::DocTitleCmdGroup(content) => {
                nodes.push(render_doc_title_cmd_group(content))
            }
            e::DocRefTextTypeItem::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    Node::Reference {
        internal: true,
        refid: doc_ref_text_type.ref_id,
        children: nodes,
    }
}

fn render_doc_title_cmd_group(doc_title_cmd_group: e::DocTitleCmdGroup) -> Node {
    tracing::debug!("render_doc_title_cmd_group {doc_title_cmd_group:?}");
    match doc_title_cmd_group {
        e::DocTitleCmdGroup::Ref(element) => render_doc_ref_text_type(element),
        e::DocTitleCmdGroup::Bold(element) => Node::Bold(render_doc_markup_type(element)),
        e::DocTitleCmdGroup::Emphasis(element) => Node::Emphasis(render_doc_markup_type(element)),

        e::DocTitleCmdGroup::S(element)
        | e::DocTitleCmdGroup::Strike(element)
        | e::DocTitleCmdGroup::Underline(element)
        | e::DocTitleCmdGroup::Computeroutput(element)
        | e::DocTitleCmdGroup::Subscript(element)
        | e::DocTitleCmdGroup::Superscript(element)
        | e::DocTitleCmdGroup::Center(element)
        | e::DocTitleCmdGroup::Small(element)
        | e::DocTitleCmdGroup::Cite(element)
        | e::DocTitleCmdGroup::Del(element)
        | e::DocTitleCmdGroup::Ins(element)
        | e::DocTitleCmdGroup::Summary(element) => {
            tracing::warn!(
                "Unhandled inline doc_markup node: {element:?} in render_doc_title_cmd_group"
            );
            Node::UnknownInline(render_doc_markup_type(element))
        }
        element => {
            tracing::error!("No render handled for {element:?} in render_doc_title_cmd_group");
            Node::Unknown
        }
    }
}

fn render_doc_markup_type(element: e::DocMarkupType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in element.content {
        match entry {
            e::DocMarkupTypeItem::DocCmdGroup(content) => nodes.push(render_doc_cmd_group(content)),
            e::DocMarkupTypeItem::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    nodes
}
