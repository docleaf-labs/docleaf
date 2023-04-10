use crate::nodes::{Node, SignatureType};

use crate::doxygen::compound::generated as e;

pub fn render_compound(root: e::DoxygenType) -> Vec<Node> {
    let Some(compound_def) = root.compounddef else {
        return Vec::new();
    };

    let mut content_nodes = Vec::new();

    if let Some(description) = compound_def.briefdescription {
        content_nodes.append(&mut render_description(description));
    }

    if let Some(description) = compound_def.detaileddescription {
        content_nodes.append(&mut render_description(description));
    }

    content_nodes.append(
        &mut compound_def
            .sectiondef
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
                Node::DescName(Box::new(Node::DescSignatureName(compound_def.compoundname))),
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
    let Some(compound_def) = root.compounddef else {
        return Vec::new();
    };

    let member_def = compound_def.sectiondef.into_iter().find_map(|section_def| {
        section_def
            .memberdef
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

fn render_section_def(section_def: e::SectiondefType) -> Node {
    let mut content_nodes = vec![Node::Rubric(vec![Node::Text(section_title(
        &section_def.kind,
    ))])];
    content_nodes.append(
        &mut section_def
            .memberdef
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

pub fn render_member_def(member_def: e::MemberdefType) -> Node {
    let name = member_kind_name(&member_def.kind);
    let mut content_nodes = Vec::new();

    if let Some(description) = member_def.briefdescription {
        content_nodes.append(&mut render_description(description));
    }
    if let Some(description) = member_def.detaileddescription {
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

pub fn render_enum_value(enum_value: e::EnumvalueType) -> Node {
    let mut content_nodes = Vec::new();

    if let Some(description) = enum_value.briefdescription {
        content_nodes.append(&mut render_description(description));
    }
    if let Some(description) = enum_value.detaileddescription {
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

fn render_description(element: e::DescriptionType) -> Vec<Node> {
    element.para.into_iter().map(render_doc_para_type).collect()
}

fn extract_inner_description(nodes: Vec<Node>) -> Vec<Node> {
    tracing::debug!("extract_inner_description: {nodes:#?}");
    if nodes.len() == 1 {
        // Check without taking ownership
        match nodes.first() {
            Some(Node::Paragraph(_)) => {
                // Extract and take ownership
                if let Some(Node::Paragraph(inner)) = nodes.into_iter().next() {
                    return inner;
                } else {
                    // Can't happen
                    panic!("Should not occur - condition already checked")
                }
            }
            _ => nodes,
        }
    } else {
        nodes
    }
}

fn render_doc_para_type(element: e::DocParaType) -> Node {
    let mut nodes = Vec::new();

    for entry in element.content {
        match entry {
            e::DocParaTypeItem::DocCmdGroup(element) => nodes.push(render_doc_cmd_group(element)),
            e::DocParaTypeItem::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    Node::Paragraph(nodes)
}

fn render_doc_cmd_group(element: e::DocCmdGroup) -> Node {
    match element {
        e::DocCmdGroup::DocTitleCmdGroup(element) => render_doc_title_cmd_group(element),
        e::DocCmdGroup::Parameterlist(element) => {
            Node::BulletList(render_doc_param_list_type(element))
        }
        e::DocCmdGroup::Simplesect(element) => {
            Node::Container(render_doc_simple_sect_type(element))
        }
        e::DocCmdGroup::Itemizedlist(element) => render_doc_list_type(element),
        e::DocCmdGroup::Orderedlist(element) => render_doc_list_type(element),
        e::DocCmdGroup::Programlisting(element) => render_listing_type(element),
        // TODO: Change to panic
        _ => {
            tracing::error!("Unhandled DocCmdGroup node: {element:?} in render_doc_cmd_group");
            Node::Unknown
        }
    }
}

fn render_listing_type(element: e::ListingType) -> Node {
    let lines: Vec<Vec<Node>> = element
        .codeline
        .into_iter()
        .map(render_code_line_type)
        .collect();

    let nodes = itertools::intersperse(lines.into_iter(), vec![Node::Text("\n".to_string())])
        .flat_map(|vec| vec.into_iter())
        .collect();

    Node::LiteralBlock(nodes)
}

fn render_code_line_type(element: e::CodelineType) -> Vec<Node> {
    element
        .highlight
        .into_iter()
        .flat_map(render_highlight_type)
        .collect()
}

fn render_highlight_type(element: e::HighlightType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in element.content {
        match entry {
            e::HighlightTypeItem::Sp(content) => nodes.push(render_sp_type(content)),
            e::HighlightTypeItem::Ref(content) => nodes.push(render_ref_text_type(content)),
            e::HighlightTypeItem::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    nodes
}

fn render_sp_type(element: e::SpType) -> Node {
    Node::Text(" ".to_string())
}

fn render_doc_list_type(element: e::DocListType) -> Node {
    let items = element
        .listitem
        .into_iter()
        .map(render_doc_list_item_type)
        .collect();
    Node::EnumeratedList(items)
}

fn render_doc_list_item_type(element: e::DocListItemType) -> Node {
    let contents = element.para.into_iter().map(render_doc_para_type).collect();
    Node::ListItem(contents)
}

/// Incomplete - just renders the para blocks at the moment
fn render_doc_simple_sect_type(element: e::DocSimpleSectType) -> Vec<Node> {
    element.para.into_iter().map(render_doc_para_type).collect()
}

fn render_doc_param_list_type(element: e::DocParamListType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for item in element.parameteritem {
        let mut contents = render_doc_param_name_list(item.parameternamelist);
        contents.push(Node::Text(" - ".to_string()));

        let description = render_description(item.parameterdescription);
        let mut inner_description = extract_inner_description(description);
        contents.append(&mut inner_description);

        nodes.push(Node::ListItem(contents))
    }

    nodes
}

fn render_doc_param_name_list(element: e::DocParamNameList) -> Vec<Node> {
    element
        .parametername
        .into_iter()
        .flat_map(render_doc_param_name)
        .collect()
}

// TODO: Create macros or abstraction for this Ref + Text pattern
fn render_doc_param_name(element: e::DocParamName) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in element.content {
        match entry {
            e::DocParamNameItem::Ref(content) => nodes.push(render_ref_text_type(content)),
            e::DocParamNameItem::Text(text) => nodes.push(Node::Text(text)),
        }
    }

    nodes
}

// TODO: Create macros or abstraction for this Ref + Text pattern
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
        refid: ref_text_type.refid,
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
        refid: doc_ref_text_type.refid,
        children: nodes,
    }
}

fn render_doc_title_cmd_group(doc_title_cmd_group: e::DocTitleCmdGroup) -> Node {
    tracing::debug!("render_doc_title_cmd_group {doc_title_cmd_group:?}");
    match doc_title_cmd_group {
        e::DocTitleCmdGroup::Ref(element) => render_doc_ref_text_type(element),
        e::DocTitleCmdGroup::Bold(element) => Node::Strong(render_doc_markup_type(element)),
        e::DocTitleCmdGroup::Emphasis(element) => Node::Emphasis(render_doc_markup_type(element)),
        e::DocTitleCmdGroup::Computeroutput(element) => {
            Node::Literal(render_doc_markup_type(element))
        }
        // This might not be the correct way to handle it but there isn't a reStructuredText line break node
        e::DocTitleCmdGroup::Linebreak => Node::Text("\n".to_string()),

        e::DocTitleCmdGroup::S(element)
        | e::DocTitleCmdGroup::Strike(element)
        | e::DocTitleCmdGroup::Underline(element)
        | e::DocTitleCmdGroup::Subscript(element)
        | e::DocTitleCmdGroup::Superscript(element)
        | e::DocTitleCmdGroup::Center(element)
        | e::DocTitleCmdGroup::Small(element)
        | e::DocTitleCmdGroup::Cite(element)
        | e::DocTitleCmdGroup::Del(element)
        | e::DocTitleCmdGroup::Ins(element)
        | e::DocTitleCmdGroup::Summary(element) => {
            tracing::error!(
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
