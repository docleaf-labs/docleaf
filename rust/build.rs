use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    xsd_codegen::Builder::new(
        std::path::PathBuf::from("xsd/compound.xsd"),
        "doxygen".to_string(),
        "DoxygenType".to_string(),
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

    xsd_codegen::Builder::new(
        std::path::PathBuf::from("xsd/index.xsd"),
        "doxygenindex".to_string(),
        "DoxygenType".to_string(),
    )
    .generate()?;

    Ok(())
}
