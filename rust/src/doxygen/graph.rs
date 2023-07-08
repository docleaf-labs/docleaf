use std::collections::HashMap;
use std::path::PathBuf;

use crate::doxygen::compound::generated as e;

pub fn render(compound_id: &str, inheritancegraph: &e::GraphType) -> anyhow::Result<PathBuf> {
    let mut template_lines: Vec<String> = Vec::new();

    let node_lookup: HashMap<&str, &e::NodeType> = HashMap::from_iter(
        inheritancegraph
            .node
            .iter()
            .map(|node| (node.id.as_str(), node)),
    );

    for node in inheritancegraph.node.iter() {
        for child in node.childnode.iter() {
            if let Some(linked_node) = node_lookup.get(child.refid.as_str()) {
                template_lines.push(format!(
                    "{parent} <|-- {child}",
                    parent = linked_node.label,
                    child = node.label
                ));
            }
        }
    }

    let lines = template_lines
        .into_iter()
        .map(|line| format!("    {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    let content = format!("classDiagram\n{lines}");

    let path = PathBuf::from(format!("{compound_id}.mermaid"));
    let file = std::fs::write(&path, content);

    Ok(path)
}
