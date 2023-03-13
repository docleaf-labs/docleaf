use std::path::{Path, PathBuf};

use proc_macro2::Span;
use syn::__private::ToTokens;

fn generate_mod_from_xsd(mod_name: &str, _xsd_path: &Path) -> anyhow::Result<()> {
    let ast = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: vec![syn::Item::Struct(syn::ItemStruct {
            attrs: vec![],
            vis: syn::Visibility::Public(syn::VisPublic {
                pub_token: syn::Token![pub](Span::call_site()),
            }), // syn::Visibility::Public(syn::VisPublic,
            struct_token: syn::Token![struct](Span::call_site()),
            ident: syn::Ident::new("Example", Span::call_site()),
            generics: syn::Generics::default(),
            fields: syn::Fields::Unit,
            semi_token: None,
        })],
    };

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let dir = out_dir.join("xsds");

    std::fs::create_dir_all(&dir)?;

    let path = dir.join(format!("{mod_name}.rs"));

    eprintln!("ast {}", ast.to_token_stream().to_string());

    std::fs::write(&path, ast.to_token_stream().to_string())?;

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
