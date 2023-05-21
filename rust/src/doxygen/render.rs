use std::collections::HashMap;

use crate::XmlLoader;

use crate::doxygen::compound::generated as e;
use crate::doxygen::compound::CompoundDefEntry;
use crate::doxygen::nodes::{
    Domain, DomainEntry, DomainEntryType, ListEnumType, Node, SignatureType, Target,
};
use crate::doxygen::text;

fn domain_from_language(language: &e::DoxLanguage) -> Option<Domain> {
    match language {
        e::DoxLanguage::CPlusPlus => Some(Domain::CPlusPlus),
        _ => None,
    }
}

fn domain_from_location(ctx: &Context, location: &e::LocationType) -> Option<Domain> {
    let extension = std::path::Path::new(&location.file).extension();
    if let Some(extension) = extension.and_then(|str| str.to_str()) {
        ctx.extension_domain_lookup.get(extension).cloned()
    } else {
        None
    }
}

/// Information and options for rendering
#[derive(Debug, Default)]
pub struct Context {
    pub domain: Option<Domain>,
    /// A list of Doxygen xml nodes names to ignore when rendering. Limited support.
    pub skip_xml_nodes: Vec<String>,
    pub extension_domain_lookup: HashMap<String, Domain>,
    pub enumerated_list_depth: usize,
}

impl Context {
    fn with_domain(
        &self,
        location: Option<&e::LocationType>,
        language: Option<&e::DoxLanguage>,
    ) -> Context {
        // Prefer calculating the domain from the location as there are time when the doxygen
        // xml seems keen to classify something as C++ even if it could be C and we need users
        // to be able to determine it
        let domain = location
            .and_then(|loc| domain_from_location(self, loc))
            .or_else(|| language.and_then(domain_from_language))
            // Fallback to current domain if we don't find one so that we don't
            // lose domain information
            .or(self.domain.clone());

        Context {
            domain,
            skip_xml_nodes: self.skip_xml_nodes.clone(),
            extension_domain_lookup: self.extension_domain_lookup.clone(),
            enumerated_list_depth: self.enumerated_list_depth,
        }
    }

    fn with_next_enumerated_list_level(&self) -> Context {
        Context {
            domain: self.domain.clone(),
            skip_xml_nodes: self.skip_xml_nodes.clone(),
            extension_domain_lookup: self.extension_domain_lookup.clone(),
            enumerated_list_depth: self.enumerated_list_depth + 1,
        }
    }

    fn enumerated_list_type(&self) -> ListEnumType {
        const TYPES: &[ListEnumType] = &[
            ListEnumType::Arabic,
            ListEnumType::LowerAlpha,
            ListEnumType::LowerRoman,
            ListEnumType::UpperAlpha,
            ListEnumType::UpperRoman,
        ];

        TYPES[self.enumerated_list_depth % TYPES.len()]
    }
}

/// Entry point
pub fn render_compounddef_content(
    ctx: &Context,
    entry: CompoundDefEntry,
    inner_groups: bool,
    xml_loader: &mut crate::XmlLoader,
) -> anyhow::Result<Vec<Node>> {
    match entry {
        CompoundDefEntry::SectionDef(section_def) => Ok(vec![render_section_def(ctx, section_def)]),
        CompoundDefEntry::Class(ref_type) => {
            let root = xml_loader.load(&ref_type.refid)?;
            render_compound(ctx, root.as_ref(), inner_groups, xml_loader)
        }
        CompoundDefEntry::Group(ref_type) => {
            let root = xml_loader.load(&ref_type.refid)?;
            render_compound(ctx, root.as_ref(), inner_groups, xml_loader)
        }
    }
}

pub fn render_compound(
    ctx: &Context,
    root: &e::DoxygenType,
    inner_groups: bool,
    xml_loader: &mut XmlLoader,
) -> anyhow::Result<Vec<Node>> {
    let Some(ref compound_def) = root.compounddef else {
        return Ok(Vec::new());
    };

    let ctx = ctx.with_domain(
        compound_def.location.as_ref(),
        compound_def.language.as_ref(),
    );

    let mut content_nodes = Vec::new();

    if let Some(ref description) = compound_def.briefdescription {
        content_nodes.append(&mut render_description(&ctx, description));
    }

    if let Some(ref description) = compound_def.detaileddescription {
        content_nodes.append(&mut render_description(&ctx, description));
    }

    for innerclass in compound_def.innerclass.iter() {
        let root = xml_loader.load(&innerclass.refid)?;
        content_nodes.append(&mut render_compound(
            &ctx,
            root.as_ref(),
            inner_groups,
            xml_loader,
        )?);
    }

    if inner_groups {
        for innergroup in compound_def.innergroup.iter() {
            let root = xml_loader.load(&innergroup.refid)?;
            content_nodes.append(&mut render_compound(
                &ctx,
                root.as_ref(),
                inner_groups,
                xml_loader,
            )?);
        }
    }

    content_nodes.append(
        &mut compound_def
            .sectiondef
            .iter()
            .map(|section_def| render_section_def(&ctx, section_def))
            .collect(),
    );

    let ids = compound_def.id.clone();
    let names = compound_def.id.clone();
    let target = Target { ids, names };

    // If we have a valid domain and compound type pairing then we return a request to use a domain entry
    // instead of attempting to render the compound signature ourselves
    match (ctx.domain.as_ref(), &compound_def.kind) {
        (Some(domain), e::DoxCompoundKind::Class) => {
            return Ok(vec![Node::DomainEntry(Box::new(DomainEntry {
                domain: domain.clone(),
                type_: DomainEntryType::Class,
                target,
                declaration: text::render_compound_def(compound_def),
                content: content_nodes,
            }))]);
        }
        (Some(domain), e::DoxCompoundKind::Struct) => {
            return Ok(vec![Node::DomainEntry(Box::new(DomainEntry {
                domain: domain.clone(),
                type_: DomainEntryType::Struct,
                target,
                declaration: text::render_compound_def(compound_def),
                content: content_nodes,
            }))]);
        }
        _ => {}
    }

    let content = Node::DescContent(content_nodes);

    let kind = text::render_compound_kind(&compound_def.kind);

    Ok(vec![Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(vec![
                Node::Target(target),
                Node::DescSignatureKeyword(vec![Node::Text(kind.to_string())]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(
                    compound_def.compoundname.clone(),
                ))),
            ])],
        )],
        Box::new(content),
    )])
}

/// Entry point
pub fn render_member(ctx: &Context, root: &e::DoxygenType, member_ref_id: &str) -> Vec<Node> {
    let Some(ref compound_def) = root.compounddef else {
        return Vec::new();
    };

    let ctx = ctx.with_domain(
        compound_def.location.as_ref(),
        compound_def.language.as_ref(),
    );

    let member_def = compound_def.sectiondef.iter().find_map(|section_def| {
        section_def
            .memberdef
            .iter()
            .find(|member_def| member_def.id == member_ref_id)
    });

    match member_def {
        Some(member_def) => render_member_def(&ctx, member_def),
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
            .flat_map(|element| render_member_def(ctx, element))
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

pub fn render_member_def(ctx: &Context, member_def: &e::MemberdefType) -> Vec<Node> {
    let name = member_kind_name(&member_def.kind);
    let mut content_nodes = Vec::new();

    // Create a new context with the location information if it is there
    let ctx = &ctx.with_domain(member_def.location.as_ref(), None);

    if let Some(ref description) = member_def.briefdescription {
        content_nodes.append(&mut render_description(ctx, description));
    }

    if let Some(ref description) = member_def.detaileddescription {
        content_nodes.append(&mut render_description(ctx, description));
    }

    let ids = member_def.id.clone();
    let names = member_def.id.clone();
    let target = Target { ids, names };

    let signature_line;

    match member_def.kind {
        e::DoxMemberKind::Enum => {
            content_nodes.append(
                &mut member_def
                    .enumvalue
                    .iter()
                    .map(|element| render_enum_value(ctx, &member_def.name, element))
                    .collect(),
            );

            // Early exit if there is domain information for rendering this entry
            if let Some(ref domain) = ctx.domain {
                return vec![Node::DomainEntry(Box::new(DomainEntry {
                    domain: domain.clone(),
                    type_: DomainEntryType::Enum,
                    target,
                    declaration: text::render_member_def(member_def),
                    content: content_nodes,
                }))];
            }

            signature_line = vec![
                Node::Target(target),
                Node::DescSignatureKeyword(vec![Node::Text(name)]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
            ];
        }
        e::DoxMemberKind::Function => {
            // Early exit if there is domain information for rendering this entry
            if let Some(ref domain) = ctx.domain {
                return vec![Node::DomainEntry(Box::new(DomainEntry {
                    domain: domain.clone(),
                    type_: DomainEntryType::Function,
                    target,
                    declaration: text::render_member_def(member_def),
                    content: content_nodes,
                }))];
            }

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
                        Node::Target(target),
                        Node::DescSignatureKeyword(render_linked_text_type(ctx, type_)),
                        Node::DescSignatureSpace,
                        Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
                        Node::DescParameterList(parameter_list_items),
                    ];
                }
                None => {
                    signature_line = vec![
                        Node::Target(target),
                        Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
                        Node::DescParameterList(parameter_list_items),
                    ];
                }
            }
        }
        e::DoxMemberKind::Define => {
            // Early exit if there is domain information for rendering this entry
            if let Some(ref domain) = ctx.domain {
                return vec![Node::DomainEntry(Box::new(DomainEntry {
                    domain: domain.clone(),
                    type_: DomainEntryType::Define,
                    target,
                    declaration: text::render_member_def(member_def),
                    content: content_nodes,
                }))];
            }

            signature_line = vec![
                Node::Target(target),
                Node::DescSignatureKeyword(vec![Node::Text(name)]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
            ];
        }
        _ => {
            signature_line = vec![
                Node::Target(target),
                Node::DescSignatureKeyword(vec![Node::Text(name)]),
                Node::DescSignatureSpace,
                Node::DescName(Box::new(Node::DescSignatureName(member_def.name.clone()))),
            ];
        }
    };

    let content = Node::DescContent(content_nodes);

    vec![Node::Desc(
        vec![Node::DescSignature(
            SignatureType::MultiLine,
            vec![Node::DescSignatureLine(signature_line)],
        )],
        Box::new(content),
    )]
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

pub fn render_enum_value(ctx: &Context, enum_name: &str, enum_value: &e::EnumvalueType) -> Node {
    let mut content_nodes = Vec::new();

    if let Some(ref description) = enum_value.briefdescription {
        content_nodes.append(&mut render_description(ctx, description));
    }
    if let Some(ref description) = enum_value.detaileddescription {
        content_nodes.append(&mut render_description(ctx, description));
    }

    if let Some(ref domain) = ctx.domain {
        let ids = enum_value.id.clone();
        let names = enum_value.id.clone();
        let target = Target { ids, names };

        Node::DomainEntry(Box::new(DomainEntry {
            domain: domain.clone(),
            type_: DomainEntryType::Enumerator,
            target,
            declaration: text::render_enum_value(domain, enum_name, enum_value),
            content: content_nodes,
        }))
    } else {
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
}

/// We treat certain nodes as special, like the parameter lists.
fn render_description(ctx: &Context, element: &e::DescriptionType) -> Vec<Node> {
    let cat_nodes: Vec<_> = element
        .para
        .iter()
        // Render the para node contents and then lift all the special nodes (list paramater lists) out of the para
        // output and group the rest under Paragraph nodes. This allows us to manage the special nodes whilst still
        // having the original content arranged in Paragraph nodes as you'd expect from rendering a 'para'
        .flat_map(|element| {
            let inner_cat_nodes = render_doc_para_type(ctx, element);

            let (field_list_candidates, other): (Vec<_>, Vec<_>) = inner_cat_nodes
                .into_iter()
                .partition(|cat| cat.requires_field_list_entry());

            let mut result = Vec::new();

            for entry in field_list_candidates {
                result.push(entry)
            }

            // Nodes which should be a standalone block (and not inside a paragraph) like literal_blocks are presented
            // in the xml as nested inside para tags so we chose to lift them out here. We do this by adding all the
            // nodes to a paragraphs except the ones we care about (eg. ProgramListing)
            let mut paragraph_nodes = Vec::new();
            for entry in other {
                match entry {
                    CategorizedNode::Node(node) => paragraph_nodes.push(node),
                    CategorizedNode::Block(node) => {
                        if !paragraph_nodes.is_empty() {
                            if all_white_space(&paragraph_nodes) {
                                paragraph_nodes = Vec::new();
                            } else {
                                result
                                    .push(CategorizedNode::Node(Node::Paragraph(paragraph_nodes)));
                                paragraph_nodes = Vec::new();
                            }
                        }
                        result.push(CategorizedNode::Node(node))
                    }
                    CategorizedNode::ParameterList(_) => {
                        // Shouldn't happen due to filtering above
                    }
                }
            }

            if !paragraph_nodes.is_empty() && !all_white_space(&paragraph_nodes) {
                result.push(CategorizedNode::Node(Node::Paragraph(paragraph_nodes)));
            }

            result
        })
        .collect();

    // Having separate the special nodes from the paragraphs for each 'para' node, we then separate all the special
    // nodes from all the paragraph nodes and render the special nodes separately
    let (special, paragraphs): (Vec<_>, Vec<_>) = cat_nodes
        .into_iter()
        .partition(|cat| cat.requires_field_list_entry());

    let mut nodes = paragraphs.into_nodes();

    if !special.is_empty() {
        let fields = special
            .into_iter()
            .flat_map(|cat_node| match cat_node {
                CategorizedNode::ParameterList(node) => Some(Node::Field(
                    Box::new(Node::FieldName(vec![Node::Text("Parameters".to_string())])),
                    Box::new(Node::FieldBody(vec![node])),
                )),
                // These entries have already been filtered out so we don't worry about them
                CategorizedNode::Block(_) => None,
                CategorizedNode::Node(_) => None,
            })
            .collect();

        nodes.push(Node::FieldList(fields))
    }

    nodes
}

/// Returns true if all the nodes are Text nodes with only white space contents
fn all_white_space(nodes: &[Node]) -> bool {
    nodes.iter().all(only_white_space_text)
}

fn only_white_space_text(node: &Node) -> bool {
    match node {
        Node::Text(text) => text.chars().all(|char| char == ' ' || char == '\n'),
        _ => false,
    }
}

fn extract_inner_description(nodes: Vec<Node>) -> Vec<Node> {
    tracing::debug!("extract_inner_description: {nodes:#?}");
    if nodes.len() == 1 {
        // Check without taking ownership
        match nodes.first() {
            Some(Node::Paragraph(_)) => {
                // Extract and take ownership
                if let Some(Node::Paragraph(inner)) = nodes.into_iter().next() {
                    inner
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

/// Renders the contents of the doc para type but attempts to separate special values like parameters lists from the
/// regular paragraphs as we want to identify and display those in a special manner
fn render_doc_para_type(ctx: &Context, element: &e::DocParaType) -> Vec<CategorizedNode> {
    let mut nodes = Vec::new();

    for entry in element.content.iter() {
        match entry {
            e::DocParaTypeItem::DocCmdGroup(ref element) => {
                if let Some(node) = render_doc_cmd_group(ctx, element) {
                    nodes.push(node)
                }
            }
            e::DocParaTypeItem::Text(text) => {
                nodes.push(CategorizedNode::Node(Node::Text(text.clone())))
            }
        }
    }

    nodes
}

/// Enum for categorizing nodes based on how we want them to be treated within a description block
#[derive(Debug)]
enum CategorizedNode {
    /// Parameter lists are lifted out and rendered in a separate field list
    ParameterList(Node),
    /// Anything we wrap in a 'Block' should be lifted up to place as siblings to the paragraph elements
    /// that we're generating for description contents
    Block(Node),
    /// All other nodes can be treated normally
    Node(Node),
}

impl CategorizedNode {
    pub fn to_node(self) -> Node {
        match self {
            Self::ParameterList(node) => node,
            Self::Block(node) => node,
            Self::Node(node) => node,
        }
    }

    pub fn requires_field_list_entry(&self) -> bool {
        match self {
            Self::ParameterList(_) => true,
            Self::Block(_) => false,
            Self::Node(_) => false,
        }
    }
}

// Trait to allow us to add 'to_nodes' to a Vec<CategorizedNode> for ergonomics
trait ToNodes {
    fn into_nodes(self) -> Vec<Node>;
}

// Provides an easy way to get from Vec<CategorizedNode> to Vec<Node> for situations where the categorization isn't
// important
impl ToNodes for Vec<CategorizedNode> {
    fn into_nodes(self) -> Vec<Node> {
        self.into_iter().map(|cn| cn.to_node()).collect()
    }
}

// As we need to treat the ParameterList (and maybe some other nodes) as a special case we render into the
// CategorizedNode type so that we can separate the parameter lists, etc, further up in the stack if needed
fn render_doc_cmd_group(ctx: &Context, element: &e::DocCmdGroup) -> Option<CategorizedNode> {
    match element {
        e::DocCmdGroup::DocTitleCmdGroup(element) => {
            render_doc_title_cmd_group(ctx, element).map(CategorizedNode::Node)
        }
        e::DocCmdGroup::Parameterlist(element) => Some(CategorizedNode::ParameterList(
            Node::BulletList(render_doc_param_list_type(ctx, element)),
        )),
        e::DocCmdGroup::Simplesect(element) => Some(CategorizedNode::Node(Node::Container(
            render_doc_simple_sect_type(ctx, element),
        ))),
        e::DocCmdGroup::Itemizedlist(element) => Some(CategorizedNode::Node(render_doc_list_type(
            ctx,
            element,
            ListType::Itemized,
        ))),
        e::DocCmdGroup::Orderedlist(element) => Some(CategorizedNode::Node(render_doc_list_type(
            ctx,
            element,
            ListType::Ordered,
        ))),
        e::DocCmdGroup::Programlisting(element) => {
            Some(CategorizedNode::Block(render_listing_type(ctx, element)))
        }
        e::DocCmdGroup::Verbatim(text) => Some(render_verbatim_text(ctx, text)),
        e::DocCmdGroup::Xrefsect(element) => Some(CategorizedNode::Node(
            render_doc_xref_sect_type(ctx, element),
        )),
        e::DocCmdGroup::Preformatted(element) => Some(CategorizedNode::Node(Node::LiteralBlock(
            render_doc_markup_type(ctx, element).into_nodes(),
        ))),
        e::DocCmdGroup::Table(element) => {
            Some(CategorizedNode::Node(render_doc_table_type(ctx, element)))
        }
        // TODO: Change to panic
        _ => {
            tracing::error!("Unhandled DocCmdGroup node: {element:?} in render_doc_cmd_group");
            None
        }
    }
}

fn render_doc_table_type(ctx: &Context, element: &e::DocTableType) -> Node {
    tracing::debug!("render_doc_table_type");
    let rows: Vec<_> = element
        .row
        .iter()
        .map(|element| render_doc_row_type(ctx, element))
        .collect();

    let (header_rows, body_rows): (Vec<_>, Vec<_>) = rows.into_iter().partition(|row| row.heading);
    let header_nodes = header_rows.into_iter().map(|row| row.entry).collect();
    let body_nodes = body_rows.into_iter().map(|row| row.entry).collect();

    let mut nodes: Vec<_> = (0..element.cols)
        .map(|_| Node::TableColSpec {
            colwidth: "auto".to_string(),
        })
        .collect();

    nodes.push(Node::TableHead(header_nodes));
    nodes.push(Node::TableBody(body_nodes));

    Node::Table(vec![Node::TableGroup {
        cols: element.cols,
        nodes,
    }])
}

/// Custom structure to allow us to bubble up the 'heading' value from the table cells as whether
/// or not they are headings impacts what rst nodes we use but that information is only available
/// on the cells instead of further up on the rows or something
struct TableRow {
    heading: bool,
    entry: Node,
}

/// Custom structure to allow us to bubble up the 'heading' value from the table cells
struct TableCell {
    heading: bool,
    entry: Node,
}

fn render_doc_row_type(ctx: &Context, element: &e::DocRowType) -> TableRow {
    let cells: Vec<_> = element
        .entry
        .iter()
        .map(|element| render_doc_entry_type(ctx, element))
        .collect();

    TableRow {
        heading: cells.iter().any(|cell| cell.heading),
        entry: Node::TableRow(cells.into_iter().map(|cell| cell.entry).collect()),
    }
}

fn render_doc_entry_type(ctx: &Context, element: &e::DocEntryType) -> TableCell {
    let nodes = element
        .para
        .iter()
        .map(|element| Node::Paragraph(render_doc_para_type(ctx, element).into_nodes()))
        .collect();

    let heading = element.thead == e::DoxBool::Yes;

    TableCell {
        heading,
        entry: Node::TableRowEntry { heading, nodes },
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

fn render_verbatim_text(_ctx: &Context, text: &str) -> CategorizedNode {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("embed:rst") {
        return CategorizedNode::Block(Node::LiteralBlock(vec![Node::Text(text.to_string())]));
    }

    if trimmed.starts_with("embed:rst:leading-asterisk") {
        let text = text
            .lines()
            .skip(1) // skip the line with 'embed:rst' on it
            .map(|line| line.replacen('*', " ", 1))
            .collect::<Vec<_>>()
            .join("\n");
        CategorizedNode::Block(Node::ReStructuredTextBlock(text))
    } else if trimmed.starts_with("embed:rst:leading-slashes") {
        let text = text
            .lines()
            .skip(1) // skip the line with 'embed:rst' on it
            .map(|line| line.replacen("///", " ", 1))
            .collect::<Vec<_>>()
            .join("\n");
        CategorizedNode::Block(Node::ReStructuredTextBlock(text))
    } else if trimmed.starts_with("embed:rst:inline") {
        let text = text.replacen("embed:rst:inline", "", 1).replace('\n', "");
        CategorizedNode::Node(Node::ReStructuredTextInline(text))
    } else {
        // Attempt to split off the first line to remove the 'embed:rst'
        match text.split_once('\n') {
            // If we find a \n then use the rest
            Some((_first_line, rest)) => {
                CategorizedNode::Block(Node::ReStructuredTextBlock(rest.to_string()))
            }
            // If we don't find one, then remove the embed:rst and use the text
            None => CategorizedNode::Block(Node::ReStructuredTextBlock(text.replacen(
                "embed:rst",
                "",
                1,
            ))),
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

enum ListType {
    Itemized,
    Ordered,
}

fn render_doc_list_type(ctx: &Context, element: &e::DocListType, type_: ListType) -> Node {
    match type_ {
        ListType::Itemized => {
            let items = element
                .listitem
                .iter()
                .map(|element| render_doc_list_item_type(&ctx, element))
                .collect();
            Node::BulletList(items)
        }
        ListType::Ordered => {
            let enum_type = element
                .type_
                .as_ref()
                .and_then(enum_type_from_ol_type)
                .unwrap_or_else(|| ctx.enumerated_list_type());

            let item_context = ctx.with_next_enumerated_list_level();

            let items = element
                .listitem
                .iter()
                .map(|element| render_doc_list_item_type(&item_context, element))
                .collect();

            Node::EnumeratedList {
                type_: Some(enum_type),
                items,
            }
        }
    }
}

fn enum_type_from_ol_type(type_: &e::DoxOlType) -> Option<ListEnumType> {
    match type_ {
        e::DoxOlType::Numeric => None,
        e::DoxOlType::LowerA => None,
        e::DoxOlType::UpperA => None,
        e::DoxOlType::LowerI => None,
        e::DoxOlType::UpperI => None,
    }
}

fn render_doc_list_item_type(ctx: &Context, element: &e::DocListItemType) -> Node {
    let contents = element
        .para
        .iter()
        .map(|element| Node::Paragraph(render_doc_para_type(ctx, element).into_nodes()))
        .collect();
    Node::ListItem(contents)
}

/// TODO: Incomplete - just renders the para blocks at the moment
fn render_doc_simple_sect_type(ctx: &Context, element: &e::DocSimpleSectType) -> Vec<Node> {
    element
        .para
        .iter()
        .map(|element| Node::Paragraph(render_doc_para_type(ctx, element).into_nodes()))
        .collect()
}

fn render_doc_param_list_type(ctx: &Context, element: &e::DocParamListType) -> Vec<Node> {
    let mut nodes = Vec::new();

    for item in element.parameteritem.iter() {
        let mut contents = vec![Node::LiteralStrong(render_doc_param_name_list(
            ctx,
            &item.parameternamelist,
        ))];

        contents.push(Node::Text(" - ".to_string()));

        let description = render_description(ctx, &item.parameterdescription);
        let mut inner_description = extract_inner_description(description);
        contents.append(&mut inner_description);

        // Paragraph (or some kind of TextElement) node is required to avoid crash in
        // Sphinx/docutils html rendering (not a bug in their code just an expectation)
        nodes.push(Node::ListItem(vec![Node::Paragraph(contents)]))
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
                if let Some(node) = render_doc_title_cmd_group(ctx, content) {
                    nodes.push(node)
                }
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
        e::DocTitleCmdGroup::Bold(element) => Some(Node::Strong(
            render_doc_markup_type(ctx, element).into_nodes(),
        )),
        e::DocTitleCmdGroup::Emphasis(element) => Some(Node::Emphasis(
            render_doc_markup_type(ctx, element).into_nodes(),
        )),
        e::DocTitleCmdGroup::Computeroutput(element) => Some(Node::Literal(
            render_doc_markup_type(ctx, element).into_nodes(),
        )),
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
            Some(Node::UnknownInline(
                render_doc_markup_type(ctx, element).into_nodes(),
            ))
        }
        element => {
            tracing::error!("No render handled for {element:?} in render_doc_title_cmd_group");
            None
        }
    }
}

fn render_doc_markup_type(ctx: &Context, element: &e::DocMarkupType) -> Vec<CategorizedNode> {
    let mut nodes = Vec::new();

    for entry in element.content.iter() {
        match entry {
            e::DocMarkupTypeItem::DocCmdGroup(ref content) => {
                if let Some(node) = render_doc_cmd_group(ctx, content) {
                    nodes.push(node)
                }
            }
            e::DocMarkupTypeItem::Text(text) => {
                nodes.push(CategorizedNode::Node(Node::Text(text.clone())))
            }
        }
    }

    nodes
}

fn render_doc_url_link(ctx: &Context, element: &e::DocUrlLink) -> Node {
    let mut nodes = Vec::new();

    for entry in element.content.iter() {
        match entry {
            e::DocUrlLinkItem::DocTitleCmdGroup(ref content) => {
                if let Some(node) = render_doc_title_cmd_group(ctx, content) {
                    nodes.push(node)
                }
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
