pub mod generated {
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/xsds/compound.rs"));
}

pub fn parse_file(compound_xml_path: &std::path::Path) -> anyhow::Result<generated::DoxygenType> {
    tracing::info!("Reading {}", compound_xml_path.display());
    let xml = std::fs::read_to_string(compound_xml_path)?;
    generated::parse(&xml)
}
