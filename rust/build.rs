fn main() -> anyhow::Result<()> {
    xsd_codegen::generate_mod("compound", &std::path::PathBuf::from("xsd/compound.xsd"))?;

    Ok(())
}
