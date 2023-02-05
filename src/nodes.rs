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
    children: PyObject,
}

pub fn node(
    py: Python<'_>,
    str: &str,
    call_as: CallAs,
    children: Vec<impl IntoPy<PyObject>>,
) -> NodeDetails {
    NodeDetails {
        r#type: str.to_string(),
        call_as,
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
    SingleLine,
    MultiLine,
}

#[derive(Debug, Clone)]
pub enum Node {
    // Plain text
    Text(String),

    // Nodes
    Container(Vec<Node>),
    Desc(Vec<Node>, Box<Node>),
    DescContent(Vec<Node>),
    DescName(Box<Node>),
    DescParameter(Vec<Node>),
    DescParameterList(Vec<Node>),
    DescSignature(SignatureType, Vec<Node>),
    DescSignatureKeyword(String),
    DescSignatureLine(Vec<Node>),
    DescSignatureName(String),
    DescSignaturePunctuation(String),
    DescSignatureSpace,
    Index,
    Paragraph(Vec<Node>),
    Rubric(Vec<Node>),
}

impl IntoPy<PyObject> for Node {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            // Plain text
            Self::Text(text_) => text(text_).into_py(py),

            // Nodes
            Self::Container(nodes) => node(py, "container", CallAs::Source, nodes).into_py(py),
            Self::Desc(lines, content) => {
                let mut children: Vec<_> =
                    lines.into_iter().map(|entry| entry.into_py(py)).collect();
                children.push(content.into_py(py));

                node(py, "desc", CallAs::Source, children).into_py(py)
            }
            Self::DescContent(nodes) => node(py, "desc_content", CallAs::Source, nodes).into_py(py),
            Self::DescName(desc_sig_name) => {
                node(py, "desc_name", CallAs::SourceText, vec![*desc_sig_name]).into_py(py)
            }
            Self::DescParameter(nodes) => {
                node(py, "desc_parameter", CallAs::SourceText, nodes).into_py(py)
            }
            Self::DescParameterList(nodes) => {
                node(py, "desc_parameterlist", CallAs::SourceText, nodes).into_py(py)
            }
            Self::DescSignature(_type, nodes) => {
                // The Sphinx docs make it look like it should be CallAs::Args but it seems to be SourceText instead
                node(py, "desc_signature", CallAs::SourceText, nodes).into_py(py)
            }
            Self::DescSignatureKeyword(keyword) => node(
                py,
                "desc_sig_keyword",
                CallAs::SourceText,
                vec![text(keyword)],
            )
            .into_py(py),
            Self::DescSignatureLine(nodes) => {
                node(py, "desc_signature_line", CallAs::SourceText, nodes).into_py(py)
            }
            Self::DescSignatureName(name) => {
                node(py, "desc_sig_name", CallAs::SourceText, vec![text(name)]).into_py(py)
            }
            Self::DescSignaturePunctuation(text_) => node(
                py,
                "desc_sig_punctuation",
                CallAs::SourceText,
                vec![text(text_)],
            )
            .into_py(py),
            Self::DescSignatureSpace => node(
                py,
                "desc_sig_space",
                CallAs::SourceText,
                vec![text(" ".to_string())],
            )
            .into_py(py),
            Self::Index => node(py, "index", CallAs::Source, Vec::<Node>::new()).into_py(py),
            Self::Paragraph(children) => {
                node(py, "paragraph", CallAs::SourceText, children).into_py(py)
            }
            Self::Rubric(nodes) => node(py, "rubric", CallAs::Source, nodes).into_py(py),
        }
    }
}
