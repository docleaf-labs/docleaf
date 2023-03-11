use std::path::{Path, PathBuf};

fn generate_mod_from_xsd(mod_name: &str, _xsd_path: &Path) -> anyhow::Result<()> {
    let ast = quote::quote! {
        #[derive(Debug, PartialEq)]
        pub struct DoxygenType {
            // Attributes
            // pub version: DoxVersionNumber,
            // Elements
            pub compound_def: Option<CompoundDefType>,
        }
    };

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let dir = out_dir.join("xsds");

    std::fs::create_dir_all(&dir)?;

    let path = dir.join(format!("{mod_name}.rs"));
    std::fs::write(&path, ast.to_string())?;

    let output = std::process::Command::new("rustfmt").arg(&path).output()?;
    if !output.status.success() {
        anyhow::bail!("Failed to run rustfmt on {}", path.display());
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    generate_mod_from_xsd("compound", &PathBuf::from("xsd/compound.xsd"))?;

    Ok(())
}
