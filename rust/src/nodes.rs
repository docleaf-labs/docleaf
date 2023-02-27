use std::collections::HashMap;

use pyo3::prelude::*;

#[derive(Clone)]
pub enum CallAs {
    Source,
    SourceText,
    // Args,
}

impl IntoPy<PyObject> for CallAs {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Source => "source".into_py(py),
            Self::SourceText => "source-text".into_py(py),
            // Self::Args => "args".into_py(py),
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
pub enum Node {
    // Plain text
    Text(String),

    // Nodes
    Bold(Vec<Node>),
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
    // Index,
    Paragraph(Vec<Node>),
    Reference {
        internal: bool,
        refid: String,
        children: Vec<Node>,
    },
    Rubric(Vec<Node>),
    Target {
        ids: String,
        names: String,
    },

    // Placeholder node for when we haven't handled the case
    Unknown,
}

impl IntoPy<PyObject> for Node {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            // Plain text
            Self::Text(text_) => text(text_).into_py(py),

            // Nodes
            Self::Bold(nodes) => {
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
            /*
                Self::Index => node(
                    py,
                    "index",
                    CallAs::Source,
                    Attributes::new(),
                    Vec::<Node>::new(),
                )
                .into_py(py),
            */
            Self::Emphasis(nodes) => {
                node(py, "emphasis", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::Paragraph(children) => node(
                py,
                "paragraph",
                CallAs::SourceText,
                Attributes::new(),
                children,
            )
            .into_py(py),
            Self::Reference {
                internal,
                refid,
                children,
            } => {
                let attributes = HashMap::from([
                    ("internal".to_string(), internal.into_py(py)),
                    ("refid".to_string(), refid.into_py(py)),
                ]);
                node(py, "reference", CallAs::SourceText, attributes, children).into_py(py)
            }
            Self::Rubric(nodes) => {
                node(py, "rubric", CallAs::Source, Attributes::new(), nodes).into_py(py)
            }
            Self::Target { ids, names } => node(
                py,
                "target",
                CallAs::Source,
                Attributes::from([
                    ("ids".into(), vec![ids].into_py(py)),
                    ("names".into(), vec![names].into_py(py)),
                ]),
                Vec::<Node>::new(),
            )
            .into_py(py),

            // Just show empty text at the moment
            Self::Unknown => text(String::new()).into_py(py),
        }
    }
}
