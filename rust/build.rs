fn main() -> anyhow::Result<()> {
    xsd_codegen::Builder::new(std::path::PathBuf::from("xsd/compound.xsd"))
        .module("compound")
        .generate()?;

    Ok(())
}
