use std::{collections::HashSet, path::Path};

fn main() -> anyhow::Result<()> {
    let compound_path = xsd_codegen::Builder::new(
        std::path::PathBuf::from("xsd/compound.xsd"),
        xsd_codegen::Root {
            tag: "doxygen".to_string(),
            type_: "DoxygenType".to_string(),
        },
    )
    .skip_types(HashSet::from(["docEmptyType".to_string()]))
    .rename_enum_variants(vec![(
        "DoxOlType".to_string(),
        vec![
            ("1".to_string(), "Numeric".to_string()),
            ("a".to_string(), "LowerA".to_string()),
            ("A".to_string(), "UpperA".to_string()),
            ("i".to_string(), "LowerI".to_string()),
            ("I".to_string(), "UpperI".to_string()),
        ],
    )])
    .generate()?;

    let index_path = xsd_codegen::Builder::new(
        std::path::PathBuf::from("xsd/index.xsd"),
        xsd_codegen::Root {
            tag: "doxygenindex".to_string(),
            type_: "DoxygenType".to_string(),
        },
    )
    .generate()?;

    format(&compound_path);
    format(&index_path);

    Ok(())
}

fn format(path: &Path) {
    // Attempt to run rustfmt on the resulting code but don't worry if it
    // fails for any reason. The build should still work without it.
    let output = std::process::Command::new("rustfmt").arg(&path).output();
    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to run rustfmt on {}", path.display());
            }
        }
        Err(err) => {
            eprintln!("Failed to run rustfmt on {}: {err}", path.display());
        }
    }
}
