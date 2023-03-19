fn main() -> anyhow::Result<()> {
    xsd_codegen::Builder::new(
        std::path::PathBuf::from("xsd/compound.xsd"),
        "doxygen".to_string(),
        "DoxygenType".to_string(),
    )
    .generate()?;

    Ok(())
}
