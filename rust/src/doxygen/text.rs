use crate::doxygen::compound::generated as e;

pub fn render_compound_def(compound_def: &e::CompounddefType) -> String {
    // format!("{} {}", render_compound_kind(&compound_def.kind), compound_def.compoundname)
    compound_def.compoundname.to_string()
}

pub fn render_compound_kind(kind: &e::DoxCompoundKind) -> &'static str {
    match kind {
        e::DoxCompoundKind::Class => "class",
        e::DoxCompoundKind::Struct => "struct",
        e::DoxCompoundKind::Union => "union",
        e::DoxCompoundKind::Interface => "interface",
        e::DoxCompoundKind::Protocol => "protocol",
        e::DoxCompoundKind::Category => "category",
        e::DoxCompoundKind::Exception => "exception",
        e::DoxCompoundKind::Service => "service",
        e::DoxCompoundKind::Singleton => "singleton",
        e::DoxCompoundKind::Module => "module",
        e::DoxCompoundKind::Type => "type",
        e::DoxCompoundKind::File => "file",
        e::DoxCompoundKind::Namespace => "namespace",
        e::DoxCompoundKind::Group => "group",
        e::DoxCompoundKind::Page => "page",
        e::DoxCompoundKind::Example => "example",
        e::DoxCompoundKind::Dir => "dir",
        e::DoxCompoundKind::Concept => "concept",
    }
}

pub fn render_member_def(member_def: &e::MemberdefType) -> String {
    match member_def.kind {
        e::DoxMemberKind::Function => {
            match (
                member_def.definition.as_ref(),
                member_def.argsstring.as_ref(),
            ) {
                (Some(definition), Some(args)) => {
                    format!("{}{}", definition, args)
                }
                _ => String::new(),
            }
        }
        _ => String::new(),
    }
}

fn render_linked_text_type(linked_text_type: &e::LinkedTextType) -> String {
    let mut nodes = Vec::new();

    for entry in linked_text_type.content.iter() {
        match entry {
            e::LinkedTextTypeItem::Ref(ref content) => nodes.push(render_ref_text_type(content)),
            e::LinkedTextTypeItem::Text(text) => nodes.push(text.clone()),
        }
    }

    nodes.join(" ")
}

fn render_ref_text_type(ref_text_type: &e::RefTextType) -> String {
    ref_text_type.content.clone()
}
