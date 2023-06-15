use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use pyo3::prelude::*;

use crate::doxygen::compound::generated as e;

#[derive(Clone)]
pub enum CallAs {
    Source,
    SourceText,
    Args,
}

impl IntoPy<PyObject> for CallAs {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Source => "source".into_py(py),
            Self::SourceText => "source-text".into_py(py),
            Self::Args => "args".into_py(py),
        }
    }
}

#[pyclass]
pub struct NodeDetails {
    #[pyo3(get)]
    r#type: String,
    #[pyo3(get)]
    call_as: CallAs,
    #[pyo3(get)]
    attributes: PyObject,
    #[pyo3(get)]
    children: PyObject,
}

type Attributes = HashMap<String, PyObject>;

pub fn node(
    py: Python<'_>,
    str: &str,
    call_as: CallAs,
    attributes: Attributes,
    children: Vec<impl IntoPy<PyObject>>,
) -> NodeDetails {
    NodeDetails {
        r#type: str.to_string(),
        call_as,
        attributes: attributes.into_py(py),
        children: children.into_py(py),
    }
}

#[pyclass]
pub struct TextDetails {
    #[pyo3(get)]
    r#type: String,
    #[pyo3(get)]
    text: String,
}

pub fn text(text_: String) -> TextDetails {
    TextDetails {
        r#type: "text".to_string(),
        text: text_,
    }
}

#[derive(Debug, Clone)]
pub enum SignatureType {
    // SingleLine,
    MultiLine,
}

#[derive(Debug, Clone)]
pub enum Domain {
    CPlusPlus,
    C,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum DomainError {
    #[error("Unrecognised domain name: {0}")]
    Unrecognised(String),
}

impl Domain {
    pub fn create_lookup(
        lookup: HashMap<String, String>,
    ) -> Result<HashMap<String, Domain>, DomainError> {
        lookup
            .into_iter()
            .map(|(key, value)| Domain::from(value).map(|domain| (key, domain)))
            .collect::<Result<HashMap<_, _>, _>>()
    }

    // Change to proper trait
    pub fn from(str: String) -> Result<Self, DomainError> {
        match str.as_str() {
            "cpp" => Ok(Domain::CPlusPlus),
            "c" => Ok(Domain::C),
            _ => Err(DomainError::Unrecognised(str)),
        }
    }
}

impl IntoPy<PyObject> for Domain {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::CPlusPlus => "cpp".into_py(py),
            Self::C => "c".into_py(py),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Target {
    pub ids: String,
    pub names: String,
}

impl IntoPy<PyObject> for Target {
    fn into_py(self, py: Python<'_>) -> PyObject {
        HashMap::<String, PyObject>::from([
            ("ids".into(), vec![self.ids].into_py(py)),
            ("names".into(), vec![self.names].into_py(py)),
        ])
        .into_py(py)
    }
}

#[derive(Debug, Clone)]
pub enum DomainEntryType {
    Class,
    Define,
    Enum,
    Enumerator,
    Function,
    Member,
    Struct,
    Typedef,
    Union,
}

impl IntoPy<PyObject> for DomainEntryType {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Class => "class".into_py(py),
            Self::Define => "define".into_py(py),
            Self::Enum => "enum".into_py(py),
            Self::Enumerator => "enumerator".into_py(py),
            Self::Function => "function".into_py(py),
            Self::Member => "member".into_py(py),
            Self::Struct => "struct".into_py(py),
            Self::Typedef => "typedef".into_py(py),
            Self::Union => "union".into_py(py),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub path: String,
    pub line: i32,
}

impl Location {
    pub fn from(root: &Path, location_type: &e::LocationType) -> Option<Self> {
        location_type.line.map(|line| Self {
            path: root
                .join(PathBuf::from(&location_type.file))
                .display()
                .to_string(),
            line,
        })
    }
}

impl IntoPy<PyObject> for Location {
    fn into_py(self, py: Python<'_>) -> PyObject {
        HashMap::<String, PyObject>::from([
            ("path".into(), self.path.into_py(py)),
            ("line".into(), self.line.into_py(py)),
        ])
        .into_py(py)
    }
}

#[derive(Debug, Clone)]
pub struct DomainEntry {
    pub domain: Domain,
    pub type_: DomainEntryType,
    pub target: Target,
    pub location: Option<Location>,
    pub declaration: String,
    pub content: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    // Plain text
    Text(String),

    // Domains
    DomainEntry(Box<DomainEntry>),

    // Nodes
    /// Used in this code base like an html5 div - just a block level wrapper
    Container(Vec<Node>),
    Desc(Vec<Node>, Box<Node>),
    DescContent(Vec<Node>),
    DescName(Box<Node>),
    DescParameter(Vec<Node>),
    DescParameterList(Vec<Node>),
    DescSignature(SignatureType, Vec<Node>),
    DescSignatureKeyword(Vec<Node>),
    DescSignatureLine(Vec<Node>),
    DescSignatureName(String),
    // DescSignaturePunctuation(String),
    DescSignatureSpace,
    Emphasis(Vec<Node>),
    // Index(Vec<IndexEntry>),
    HtmlOnly(Vec<Node>),
    Literal(Vec<Node>),
    LiteralBlock(Vec<Node>),
    LiteralStrong(Vec<Node>),
    Paragraph(Vec<Node>),
    RawHtml(String),
    Reference {
        internal: Option<bool>,
        refid: Option<String>,
        refuri: Option<String>,
        children: Vec<Node>,
    },
    Rubric {
        classes: Vec<String>,
        nodes: Vec<Node>,
    },
    Strong(Vec<Node>),
    Target(Target),

    // Tables
    Table(Vec<Node>),
    TableGroup {
        cols: i32,
        nodes: Vec<Node>,
    },
    TableColSpec {
        colwidth: String,
    },
    TableHead(Vec<Node>),
    TableBody(Vec<Node>),
    TableRow(Vec<Node>),
    TableRowEntry {
        heading: bool,
        nodes: Vec<Node>,
    },

    // Field lists
    FieldList(Vec<Node>),
    Field(Box<Node>, Box<Node>),
    FieldName(Vec<Node>),
    FieldBody(Vec<Node>),

    // Lists
    BulletList(Vec<Node>),
    EnumeratedList {
        type_: Option<ListEnumType>,
        items: Vec<Node>,
    },
    ListItem(Vec<Node>),

    // Notes
    Note(Vec<Node>),
    SeeAlso(Vec<Node>),
    Warning(Vec<Node>),

    // Embedded ReStructuredText
    ReStructuredTextBlock(String),
    ReStructuredTextInline(String),

    // Placeholder node for when we haven't handled the case
    UnknownInline(Vec<Node>),
}

// Docutils enum types for lists
#[derive(Debug, Clone, Copy)]
pub enum ListEnumType {
    Arabic,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
}

impl IntoPy<PyObject> for ListEnumType {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Arabic => "arabic".into_py(py),
            Self::LowerAlpha => "loweralpha".into_py(py),
            Self::UpperAlpha => "upperalpha".into_py(py),
            Self::LowerRoman => "lowerroman".into_py(py),
            Self::UpperRoman => "upperroman".into_py(py),
        }
    }
}

impl IntoPy<PyObject> for Node {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            // Plain text
            Self::Text(text_) => {
                text(html_escape::decode_html_entities(&text_).into_owned()).into_py(py)
            }

            // Domains
            Self::DomainEntry(entry) => node(
                py,
                "domain_entry",
                CallAs::Args,
                [
                    Some(("domain".into(), entry.domain.into_py(py))),
                    Some(("type".into(), entry.type_.into_py(py))),
                    Some(("declaration".into(), entry.declaration.into_py(py))),
                    entry
                        .location
                        .map(|loc| ("location".into(), loc.into_py(py))),
                    Some(("target".into(), entry.target.into_py(py))),
                ]
                .into_iter()
                .flatten()
                .collect::<HashMap<_, _>>(),
                entry.content,
            )
            .into_py(py),

            // Nodes
            Self::Strong(nodes) => {
                node(py, "strong", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::Container(nodes) => {
                node(py, "container", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::Desc(lines, content) => {
                let mut children: Vec<_> =
                    lines.into_iter().map(|entry| entry.into_py(py)).collect();
                children.push(content.into_py(py));

                node(py, "desc", CallAs::Source, Attributes::new(), children).into_py(py)
            }
            Self::DescContent(nodes) => {
                node(py, "desc_content", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::DescName(desc_sig_name) => node(
                py,
                "desc_name",
                CallAs::SourceText,
                Attributes::new(),
                vec![*desc_sig_name],
            )
            .into_py(py),
            Self::DescParameter(nodes) => node(
                py,
                "desc_parameter",
                CallAs::SourceText,
                Attributes::new(),
                nodes,
            )
            .into_py(py),
            Self::DescParameterList(nodes) => node(
                py,
                "desc_parameterlist",
                CallAs::SourceText,
                Attributes::new(),
                nodes,
            )
            .into_py(py),
            Self::DescSignature(_type, nodes) => {
                // The Sphinx docs make it look like it should be CallAs::Args but it seems to be SourceText instead
                node(
                    py,
                    "desc_signature",
                    CallAs::SourceText,
                    Attributes::new(),
                    nodes,
                )
                .into_py(py)
            }
            Self::DescSignatureKeyword(nodes) => node(
                py,
                "desc_sig_keyword",
                CallAs::SourceText,
                Attributes::new(),
                nodes,
            )
            .into_py(py),
            Self::DescSignatureLine(nodes) => node(
                py,
                "desc_signature_line",
                CallAs::SourceText,
                Attributes::new(),
                nodes,
            )
            .into_py(py),
            Self::DescSignatureName(name) => node(
                py,
                "desc_sig_name",
                CallAs::SourceText,
                Attributes::new(),
                vec![text(name)],
            )
            .into_py(py),
            /*
                Self::DescSignaturePunctuation(text_) => node(
                    py,
                    "desc_sig_punctuation",
                    CallAs::SourceText,
                    Attributes::new(),
                    vec![text(text_)],
                )
                .into_py(py),
            */
            Self::DescSignatureSpace => node(
                py,
                "desc_sig_space",
                CallAs::SourceText,
                Attributes::new(),
                vec![text(" ".to_string())],
            )
            .into_py(py),
            Self::Emphasis(nodes) => {
                node(py, "emphasis", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::HtmlOnly(nodes) => node(
                py,
                "only",
                CallAs::Source,
                Attributes::from([("expr".into(), "html".into_py(py))]),
                nodes,
            )
            .into_py(py),
            /*
            Self::Index(entries) => node(
                py,
                "index",
                CallAs::Source,
                Attributes::from([("entries".into(), entries.into_py(py))]),
                Vec::<Node>::new(),
            )
            .into_py(py),
            */
            Self::Literal(nodes) => {
                node(py, "literal", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::LiteralBlock(nodes) => node(
                py,
                "literal_block",
                CallAs::Source,
                Attributes::new(),
                nodes,
            )
            .into_py(py),
            Self::LiteralStrong(nodes) => node(
                py,
                "literal_strong",
                CallAs::Source,
                Attributes::new(),
                nodes,
            )
            .into_py(py),
            Self::Paragraph(children) => node(
                py,
                "paragraph",
                CallAs::SourceText,
                Attributes::new(),
                children,
            )
            .into_py(py),
            Self::RawHtml(content) => node(
                py,
                "raw",
                CallAs::Source,
                Attributes::from([("format".into(), "html".into_py(py))]),
                vec![text(content).into_py(py)],
            )
            .into_py(py),
            Self::Reference {
                internal,
                refid,
                refuri,
                children,
            } => {
                let attributes = [
                    internal.map(|value| ("internal".to_string(), value.into_py(py))),
                    refid.map(|value| ("refid".to_string(), value.into_py(py))),
                    refuri.map(|value| ("refuri".to_string(), value.into_py(py))),
                ]
                .into_iter()
                .flatten()
                .collect::<HashMap<_, _>>();

                node(py, "reference", CallAs::SourceText, attributes, children).into_py(py)
            }
            Self::Rubric { classes, nodes } => node(
                py,
                "rubric",
                CallAs::Source,
                Attributes::from([("classes".into(), classes.into_py(py))]),
                nodes,
            )
            .into_py(py),
            Self::Target(target) => node(
                py,
                "target",
                CallAs::Source,
                Attributes::from([
                    ("ids".into(), vec![target.ids].into_py(py)),
                    ("names".into(), vec![target.names].into_py(py)),
                ]),
                Vec::<Node>::new(),
            )
            .into_py(py),

            // Tables
            Self::Table(nodes) => node(
                py,
                "table",
                CallAs::Source,
                Attributes::from([("classes".into(), vec!["colwidths-auto"].into_py(py))]),
                nodes,
            )
            .into_py(py),
            Self::TableGroup { cols, nodes } => node(
                py,
                "tgroup",
                CallAs::Source,
                Attributes::from([("cols".into(), cols.into_py(py))]),
                nodes,
            )
            .into_py(py),
            Self::TableColSpec { colwidth } => node(
                py,
                "colspec",
                CallAs::Source,
                Attributes::from([("colwidth".into(), colwidth.into_py(py))]),
                Vec::<Node>::new(),
            )
            .into_py(py),
            Self::TableHead(nodes) => {
                node(py, "thead", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::TableBody(nodes) => {
                node(py, "tbody", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::TableRow(nodes) => {
                node(py, "row", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::TableRowEntry { heading, nodes } => node(
                py,
                "entry",
                CallAs::Source,
                if heading {
                    Attributes::from([("heading".into(), heading.into_py(py))])
                } else {
                    Attributes::new()
                },
                nodes,
            )
            .into_py(py),

            // Field lists
            Self::FieldList(nodes) => {
                node(py, "field_list", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::Field(name, body) => node(
                py,
                "field",
                CallAs::Source,
                Attributes::new(),
                vec![*name, *body],
            )
            .into_py(py),
            Self::FieldName(nodes) => node(
                py,
                "field_name",
                CallAs::SourceText,
                Attributes::new(),
                nodes,
            )
            .into_py(py),
            Self::FieldBody(nodes) => {
                node(py, "field_body", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }

            // Lists
            Self::BulletList(nodes) => {
                node(py, "bullet_list", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::EnumeratedList { type_, items } => {
                let attributes = [type_.map(|value| ("enumtype".to_string(), value.into_py(py)))]
                    .into_iter()
                    .flatten()
                    .collect::<HashMap<_, _>>();

                node(py, "enumerated_list", CallAs::Source, attributes, items).into_py(py)
            }
            Self::ListItem(nodes) => {
                node(py, "list_item", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }

            // Notes
            Self::Note(nodes) => {
                node(py, "note", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::SeeAlso(nodes) => {
                node(py, "see_also", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::Warning(nodes) => {
                node(py, "warning", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }

            // Embedded ReStructuredText
            Self::ReStructuredTextBlock(text_) => node(
                py,
                "restructured_text_block",
                CallAs::Args,
                Attributes::new(),
                vec![text(text_)],
            )
            .into_py(py),
            Self::ReStructuredTextInline(text_) => node(
                py,
                "restructured_text_inline",
                CallAs::Args,
                Attributes::new(),
                vec![text(text_)],
            )
            .into_py(py),

            // Just show empty text at the moment
            Self::UnknownInline(nodes) => {
                node(py, "inline", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
        }
    }
}
