pub mod compound;
pub mod index;

use super::nodes::Node;
use compound::{DescriptionType, DocParaType};

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
