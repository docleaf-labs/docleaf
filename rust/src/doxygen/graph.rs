use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::doxygen::compound::generated as e;

pub fn render(
    compound_id: &str,
    inheritancegraph: &e::GraphType,
    build_dir: &Path,
) -> anyhow::Result<PathBuf> {
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
    let content = format!("classDiagram\n{lines}\n");

    let mmd_file_name = format!("{compound_id}.mmd");
    let svg_file_name = format!("{compound_id}.svg");
    let mmd_file_path = build_dir.join(&mmd_file_name);
    let svg_file_path = build_dir.join(&svg_file_name);
    let file = std::fs::write(&mmd_file_path, content);

    let output = std::process::Command::new("mmdc")
        .args([
            &std::ffi::OsStr::new("-i"),
            mmd_file_path.as_os_str(),
            &std::ffi::OsStr::new("-o"),
            svg_file_path.as_os_str(),
        ])
        .output()?;

    Ok(svg_file_path)
}
