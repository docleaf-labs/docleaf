pub mod generated {
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/xsds/compound.rs"));
}

pub fn parse_file(compound_xml_path: &std::path::Path) -> anyhow::Result<generated::DoxygenType> {
    tracing::info!("Reading {}", compound_xml_path.display());
    let xml = std::fs::read_to_string(compound_xml_path)?;
    generated::parse(&xml)
}

pub enum CompoundDefEntry<'a> {
    SectionDef(&'a generated::SectiondefType),
    Class(&'a generated::RefType),
    Group(&'a generated::RefType),
}

pub fn extract_compounddef_contents(
    compounddef: &generated::CompounddefType,
    inner_groups: bool,
) -> Vec<CompoundDefEntry> {
    let class_iter = compounddef
        .innerclass
        .iter()
        .map(|class| CompoundDefEntry::Class(class));

    let group_iter = compounddef
        .innergroup
        .iter()
        .map(|group| CompoundDefEntry::Group(group));

    let section_def_iter = compounddef
        .sectiondef
        .iter()
        .map(|sectiondef| CompoundDefEntry::SectionDef(sectiondef));

    class_iter
        .chain(group_iter)
        .chain(section_def_iter)
        .collect()
}
