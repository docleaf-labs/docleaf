use crate::nodes::{Node, SignatureType};

use crate::doxygen::compound::generated as e;

/// Information and options for rendering
#[derive(Default)]
pub struct Context {
    /// A list of Doxygen xml nodes names to ignore when rendering. Limited support.
    pub skip_xml_nodes: Vec<String>,
}

pub fn render_compound(ctx: &Context, root: &e::DoxygenType) -> Vec<Node> {
    let Some(ref compound_def) = root.compounddef else {
        return Vec::new();
    };

    let mut content_nodes = Vec::new();

    if let Some(ref description) = compound_def.briefdescription {
        content_nodes.append(&mut render_description(ctx, description));
    }

    if let Some(ref description) = compound_def.detaileddescription {
        content_nodes.append(&mut render_description(ctx, description));
    }

    content_nodes.append(
        &mut compound_def
            .sectiondef
            .iter()
            .map(|section_def| render_section_def(ctx, section_def))
            .collect(),
    );

    let content = Node::DescContent(content_nodes);

    let ids = compound_def.id.clone();
    let names = compound_def.id.clone();

    let kind = render_compound_kind(ctx, &compound_def.kind);

    vec![Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(vec![
                Node::Target { ids, names },
                Node::DescSignatureKeyword(vec![Node::Text(kind.to_string())]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(
                    compound_def.compoundname.clone(),
                ))),
            ])],
        )],
        Box::new(content),
    )]
}

fn render_compound_kind(_ctx: &Context, kind: &e::DoxCompoundKind) -> &'static str {
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

pub fn render_member(ctx: &Context, root: &e::DoxygenType, member_ref_id: &str) -> Vec<Node> {
    let Some(ref compound_def) = root.compounddef else {
        return Vec::new();
    };

    let member_def = compound_def.sectiondef.iter().find_map(|section_def| {
        section_def
            .memberdef
            .iter()
            .find(|member_def| member_def.id == member_ref_id)
    });

    match member_def {
        Some(member_def) => {
            vec![render_member_def(ctx, &member_def)]
        }
        None => {
            vec![]
        }
    }
}

fn render_section_def(ctx: &Context, section_def: &e::SectiondefType) -> Node {
    let mut content_nodes = vec![Node::Rubric(vec![Node::Text(section_title(
        &section_def.kind,
    ))])];
    content_nodes.append(
        &mut section_def
            .memberdef
            .iter()
            .map(|element| render_member_def(ctx, &element))
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

pub fn render_member_def(ctx: &Context, member_def: &e::MemberdefType) -> Node {
    let name = member_kind_name(&member_def.kind);
    let mut content_nodes = Vec::new();

    if let Some(ref description) = member_def.briefdescription {
        content_nodes.append(&mut render_description(ctx, description));
    }
    if let Some(ref description) = member_def.detaileddescription {
        content_nodes.append(&mut render_description(ctx, description));
    }

    let ids = member_def.id.clone();
    let names = member_def.id.clone();

    let signature_line;

    match member_def.kind {
        e::DoxMemberKind::Enum => {
            signature_line = vec![
                Node::Target { ids, names },
                Node::DescSignatureKeyword(vec![Node::Text(name)]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
            ];

            content_nodes.append(
                &mut member_def
                    .enumvalue
                    .iter()
                    .map(|element| render_enum_value(ctx, &element))
                    .collect(),
            );
        }
        e::DoxMemberKind::Function => {
            let parameter_list_items = member_def
                .param
                .iter()
                .map(|param| {
                    let mut param_contents = Vec::new();

                    match (&param.type_, &param.declname) {
                        (Some(ref type_), Some(ref declname)) => {
                            param_contents.append(&mut render_linked_text_type(ctx, type_));
                            param_contents.push(Node::DescSignatureSpace);
                            param_contents.push(Node::DescSignatureName(declname.clone()));
                        }
                        (Some(ref type_), None) => {
                            param_contents.append(&mut render_linked_text_type(ctx, type_));
                        }
                        (None, Some(ref declname)) => {
                            param_contents.push(Node::DescSignatureName(declname.clone()));
                        }
                        (None, None) => {}
                    };

                    Node::DescParameter(param_contents)
                })
                .collect();

            match member_def.type_ {
                Some(ref type_) => {
                    signature_line = vec![
                        Node::Target { ids, names },
                        Node::DescSignatureKeyword(render_linked_text_type(ctx, type_)),
                        Node::DescSignatureSpace,
                        Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
                        Node::DescParameterList(parameter_list_items),
                    ];
                }
                None => {
                    signature_line = vec![
                        Node::Target { ids, names },
                        Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
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
                Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
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

pub fn render_enum_value(ctx: &Context, enum_value: &e::EnumvalueType) -> Node {
    let mut content_nodes = Vec::new();

    if let Some(ref description) = enum_value.briefdescription {
        content_nodes.append(&mut render_description(ctx, description));
    }
    if let Some(ref description) = enum_value.detaileddescription {
        content_nodes.append(&mut render_description(ctx, description));
    }

    let content = Node::DescContent(content_nodes);
    Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(vec![Node::DescName(Box::new(
                Node::DescSignatureName(enum_value.name.clone()),
            ))])],
        )],
        Box::new(content),
    )
}

fn render_description(ctx: &Context, element: &e::DescriptionType) -> Vec<Node> {
    element
        .para
        .iter()
        .map(|element| render_doc_para_type(ctx, element))
        .collect()
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

fn render_doc_para_type(ctx: &Context, element: &e::DocParaType) -> Node {
    let mut nodes = Vec::new();

    for entry in element.content.iter() {
        match entry {
            e::DocParaTypeItem::DocCmdGroup(ref element) => {
                render_doc_cmd_group(ctx, element).map(|node| nodes.push(node));
            }
            e::DocParaTypeItem::Text(text) => nodes.push(Node::Text(text.clone())),
        }
    }

    Node::Paragraph(nodes)
}

fn render_doc_cmd_group(ctx: &Context, element: &e::DocCmdGroup) -> Option<Node> {
    match element {
        e::DocCmdGroup::DocTitleCmdGroup(element) => render_doc_title_cmd_group(ctx, element),
        e::DocCmdGroup::Parameterlist(element) => {
            Some(Node::BulletList(render_doc_param_list_type(ctx, element)))
        }
        e::DocCmdGroup::Simplesect(element) => {
            Some(Node::Container(render_doc_simple_sect_type(ctx, element)))
        }
        e::DocCmdGroup::Itemizedlist(element) => Some(render_doc_list_type(ctx, element)),
        e::DocCmdGroup::Orderedlist(element) => Some(render_doc_list_type(ctx, element)),
        e::DocCmdGroup::Programlisting(element) => Some(render_listing_type(ctx, element)),
        e::DocCmdGroup::Verbatim(text) => Some(render_verbatim_text(ctx, text)),
        e::DocCmdGroup::Xrefsect(element) => Some(render_doc_xref_sect_type(ctx, element)),
        e::DocCmdGroup::Preformatted(element) => {
            Some(Node::LiteralBlock(render_doc_markup_type(ctx, element)))
        }
        // TODO: Change to panic
        _ => {
            tracing::error!("Unhandled DocCmdGroup node: {element:?} in render_doc_cmd_group");
            None
        }
    }
}

fn render_doc_xref_sect_type(ctx: &Context, element: &e::DocXRefSectType) -> Node {
    Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::Emphasis(vec![Node::Text(format!(
                "{}:",
                element.xreftitle
            ))])],
        )],
        Box::new(Node::DescContent(render_description(
            ctx,
            &element.xrefdescription,
        ))),
    )
}

fn render_verbatim_text(_ctx: &Context, text: &str) -> Node {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("embed:rst") {
        return Node::LiteralBlock(vec![Node::Text(text.to_string())]);
    }

    if trimmed.starts_with("embed:rst:leading-asterisk") {
        let text = text
            .lines()
            .skip(1) // skip the line with 'embed:rst' on it
            .map(|line| line.replacen("*", " ", 1))
            .collect::<Vec<_>>()
            .join("\n");
        Node::ReStructuredTextBlock(text)
    } else if trimmed.starts_with("embed:rst:leading-slashes") {
        let text = text
            .lines()
            .skip(1) // skip the line with 'embed:rst' on it
            .map(|line| line.replacen("///", " ", 1))
            .collect::<Vec<_>>()
            .join("\n");
        Node::ReStructuredTextBlock(text)
    } else if trimmed.starts_with("embed:rst:inline") {
        let text = text.replacen("embed:rst:inline", "", 1).replace("\n", "");
        Node::ReStructuredTextInline(text)
    } else {
        // Attempt to split off the first line to remove the 'embed:rst'
        match text.split_once('\n') {
            // If we find a \n then use the rest
            Some((_first_line, rest)) => Node::ReStructuredTextBlock(rest.to_string()),
            // If we don't find one, then remove the embed:rst and use the text
            None => Node::ReStructuredTextBlock(text.replacen("embed:rst", "", 1)),
        }
    }
}

fn render_listing_type(ctx: &Context, element: &e::ListingType) -> Node {
    let lines: Vec<Vec<Node>> = element
        .codeline
        .iter()
        .map(|element| render_code_line_type(ctx, element))
        .collect();

    let nodes = itertools::intersperse(lines.into_iter(), vec![Node::Text("\n".to_string())])
        .flat_map(|vec| vec.into_iter())
        .collect();

    Node::LiteralBlock(nodes)
}

fn render_code_line_type(ctx: &Context, element: &e::CodelineType) -> Vec<Node> {
    element
        .highlight
        .iter()
        .flat_map(|element| render_highlight_type(ctx, element))
        .collect()
}

fn render_highlight_type(ctx: &Context, element: &e::HighlightType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in element.content.iter() {
        match entry {
            e::HighlightTypeItem::Sp(ref content) => nodes.push(render_sp_type(ctx, content)),
            e::HighlightTypeItem::Ref(ref content) => {
                nodes.push(render_ref_text_type(ctx, content))
            }
            e::HighlightTypeItem::Text(text) => nodes.push(Node::Text(text.clone())),
        }
    }

    nodes
}

fn render_sp_type(_ctx: &Context, _elementt: &e::SpType) -> Node {
    Node::Text(" ".to_string())
}

fn render_doc_list_type(ctx: &Context, element: &e::DocListType) -> Node {
    let items = element
        .listitem
        .iter()
        .map(|element| render_doc_list_item_type(ctx, element))
        .collect();
    Node::EnumeratedList(items)
}

fn render_doc_list_item_type(ctx: &Context, element: &e::DocListItemType) -> Node {
    let contents = element
        .para
        .iter()
        .map(|element| render_doc_para_type(ctx, element))
        .collect();
    Node::ListItem(contents)
}

/// Incomplete - just renders the para blocks at the moment
fn render_doc_simple_sect_type(ctx: &Context, element: &e::DocSimpleSectType) -> Vec<Node> {
    element
        .para
        .iter()
        .map(|element| render_doc_para_type(ctx, element))
        .collect()
}

fn render_doc_param_list_type(ctx: &Context, element: &e::DocParamListType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for item in element.parameteritem.iter() {
        let mut contents = render_doc_param_name_list(ctx, &item.parameternamelist);
        contents.push(Node::Text(" - ".to_string()));

        let description = render_description(ctx, &item.parameterdescription);
        let mut inner_description = extract_inner_description(description);
        contents.append(&mut inner_description);

        nodes.push(Node::ListItem(contents))
    }

    nodes
}

fn render_doc_param_name_list(ctx: &Context, element: &e::DocParamNameList) -> Vec<Node> {
    element
        .parametername
        .iter()
        .flat_map(|element| render_doc_param_name(ctx, element))
        .collect()
}

// TODO: Create macros or abstraction for this Ref + Text pattern
fn render_doc_param_name(ctx: &Context, element: &e::DocParamName) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in element.content.iter() {
        match entry {
            e::DocParamNameItem::Ref(ref content) => nodes.push(render_ref_text_type(ctx, content)),
            e::DocParamNameItem::Text(text) => nodes.push(Node::Text(text.clone())),
        }
    }

    nodes
}

// TODO: Create macros or abstraction for this Ref + Text pattern
fn render_linked_text_type(ctx: &Context, linked_text_type: &e::LinkedTextType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in linked_text_type.content.iter() {
        match entry {
            e::LinkedTextTypeItem::Ref(ref content) => {
                nodes.push(render_ref_text_type(ctx, content))
            }
            e::LinkedTextTypeItem::Text(text) => nodes.push(Node::Text(text.clone())),
        }
    }

    nodes
}

fn render_ref_text_type(_ctx: &Context, ref_text_type: &e::RefTextType) -> Node {
    Node::Reference {
        internal: Some(true),
        refid: Some(ref_text_type.refid.clone()),
        refuri: None,
        children: vec![Node::DescSignatureName(ref_text_type.content.clone())],
    }
}

fn render_doc_ref_text_type(ctx: &Context, doc_ref_text_type: &e::DocRefTextType) -> Node {
    let mut nodes = Vec::new();

    for entry in doc_ref_text_type.content.iter() {
        match entry {
            e::DocRefTextTypeItem::DocTitleCmdGroup(ref content) => {
                render_doc_title_cmd_group(ctx, content).map(|node| nodes.push(node));
            }
            e::DocRefTextTypeItem::Text(text) => nodes.push(Node::Text(text.clone())),
        }
    }

    Node::Reference {
        internal: Some(true),
        refid: Some(doc_ref_text_type.refid.clone()),
        refuri: None,
        children: nodes,
    }
}

fn render_doc_title_cmd_group(
    ctx: &Context,
    doc_title_cmd_group: &e::DocTitleCmdGroup,
) -> Option<Node> {
    tracing::debug!("render_doc_title_cmd_group {doc_title_cmd_group:?}");
    match doc_title_cmd_group {
        e::DocTitleCmdGroup::Ref(element) => Some(render_doc_ref_text_type(ctx, element)),
        e::DocTitleCmdGroup::Bold(element) => {
            Some(Node::Strong(render_doc_markup_type(ctx, element)))
        }
        e::DocTitleCmdGroup::Emphasis(element) => {
            Some(Node::Emphasis(render_doc_markup_type(ctx, element)))
        }
        e::DocTitleCmdGroup::Computeroutput(element) => {
            Some(Node::Literal(render_doc_markup_type(ctx, element)))
        }
        // This might not be the correct way to handle it but there isn't a reStructuredText line break node
        e::DocTitleCmdGroup::Linebreak => Some(Node::Text("\n".to_string())),
        e::DocTitleCmdGroup::Htmlonly(element) => {
            if ctx.skip_xml_nodes.contains(&"htmlonly".to_string()) {
                None
            } else {
                Some(Node::HtmlOnly(vec![Node::RawHtml(element.content.clone())]))
            }
        }
        e::DocTitleCmdGroup::Ulink(element) => Some(render_doc_url_link(ctx, element)),

        // Simple characters
        // Use unicode sequence as rustfmt doesn't seem to like the en-dash character
        e::DocTitleCmdGroup::Mdash => Some(Node::Text("\u{2014}".to_string())),
        e::DocTitleCmdGroup::Ndash => Some(Node::Text("\u{2013}".to_string())),
        e::DocTitleCmdGroup::Lsquo => Some(Node::Text("\u{2018}".to_string())),
        e::DocTitleCmdGroup::Rsquo => Some(Node::Text("\u{2019}".to_string())),
        e::DocTitleCmdGroup::Nonbreakablespace => Some(Node::Text("\u{00A0}".to_string())),

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
            Some(Node::UnknownInline(render_doc_markup_type(ctx, element)))
        }
        element => {
            tracing::error!("No render handled for {element:?} in render_doc_title_cmd_group");
            None
        }
    }
}

fn render_doc_markup_type(ctx: &Context, element: &e::DocMarkupType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for entry in element.content.iter() {
        match entry {
            e::DocMarkupTypeItem::DocCmdGroup(ref content) => {
                render_doc_cmd_group(ctx, content).map(|node| nodes.push(node));
            }
            e::DocMarkupTypeItem::Text(text) => nodes.push(Node::Text(text.clone())),
        }
    }

    nodes
}

fn render_doc_url_link(ctx: &Context, element: &e::DocUrlLink) -> Node {
    let mut nodes = Vec::new();

    for entry in element.content.iter() {
        match entry {
            e::DocUrlLinkItem::DocTitleCmdGroup(ref content) => {
                render_doc_title_cmd_group(ctx, content).map(|node| nodes.push(node));
            }
            e::DocUrlLinkItem::Text(text) => nodes.push(Node::Text(text.clone())),
        }
    }

    Node::Reference {
        internal: None,
        refid: None,
        refuri: Some(element.url.clone()),
        children: nodes,
    }
}
