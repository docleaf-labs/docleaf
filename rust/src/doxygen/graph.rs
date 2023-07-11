use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::doxygen::compound::generated as e;

pub fn render(
    compound_id: &str,
    inheritancegraph: &e::GraphType,
    build_dir: &Path,
    mermaid_command: &[&str],
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

    // Hash contents so that we only re-run mermaid image generation if we haven't already got the appropriate file
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let hash = hasher.finish();

    let svg_file_name = format!("{hash:x}_{compound_id}.svg");
    let svg_file_path = build_dir.join(&svg_file_name);

    if !svg_file_path.exists() {
        tracing::info!("Generating {svg_file_name}");
        let mmd_file_name = format!("{hash:x}_{compound_id}.mmd");
        let mmd_file_path = build_dir.join(&mmd_file_name);
        std::fs::write(&mmd_file_path, content)
            .with_context(|| format!("Failed to write to path: {}", mmd_file_path.display()))?;

        // Allow users to provide multi-component way to run the mermaid command line program. It might just be
        // ["mmdc"] but it might also be ["npx", "mmdc"] or something else if installed in a specific way
        match mermaid_command.split_first() {
            // Split off the first from the rest so that we can prep the values for std::process::Command
            Some((first, rest)) => {
                // Concatenate the user provided 'rest' with the args that we know we need to pass to mermaid
                let args: Vec<_> = rest
                    .iter()
                    .map(|str| std::ffi::OsStr::new(str))
                    .chain([
                        &std::ffi::OsStr::new("-i"),
                        mmd_file_path.as_os_str(),
                        &std::ffi::OsStr::new("-o"),
                        svg_file_path.as_os_str(),
                    ])
                    .collect();

                // Attempt to run the command and provide a reasonable error message if it fails
                std::process::Command::new(first)
                    .args(&args)
                    .output()
                    .with_context(|| {
                        format!(
                            "Failed to run mermaid command: {:?}",
                            [&[*first], rest].concat()
                        )
                    })?;
            }
            None => {}
        }
    }

    Ok(svg_file_path)
}
