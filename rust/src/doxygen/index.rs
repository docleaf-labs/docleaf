pub mod generated {
    #![allow(dead_code)]
    #![allow(clippy::single_match)]
    #![allow(clippy::match_single_binding)]

    include!(concat!(env!("OUT_DIR"), "/xsds/index.rs"));
}

pub fn parse_file(index_xml_path: &std::path::Path) -> anyhow::Result<generated::DoxygenType> {
    tracing::info!("Reading {}", index_xml_path.display());
    let xml = std::fs::read_to_string(index_xml_path)?;
    generated::parse(&xml)
}
