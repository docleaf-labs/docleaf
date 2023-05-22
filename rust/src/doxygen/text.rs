use crate::doxygen::compound::generated as e;
use crate::Domain;

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

pub fn render_member_def(domain: &Domain, member_def: &e::MemberdefType) -> String {
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
        e::DoxMemberKind::Define => {
            if member_def.param.is_empty() {
                member_def.name.clone()
            } else {
                let params: Vec<String> = member_def
                    .param
                    .iter()
                    .flat_map(|param| param.defname.clone())
                    .collect();
                let params = params.join(", ");
                format!("{}({})", member_def.name, params)
            }
        }
        e::DoxMemberKind::Enum => member_def.name.clone(),
        e::DoxMemberKind::Variable => {
            let name = member_def
                .qualifiedname
                .as_ref()
                .map(|name| match domain {
                    Domain::CPlusPlus => name.clone(),
                    Domain::C => name.replace("::", "."),
                })
                .unwrap_or_else(|| member_def.name.clone());

            match member_def.type_ {
                Some(ref type_) => format!("{} {}", render_linked_text_type(type_), name),
                None => name,
            }
        }
        _ => todo!(
            "text::render_member_def not implemented for {:?}",
            member_def.kind
        ),
    }
}

pub fn render_enum_value(
    domain: &Domain,
    enum_name: &str,
    enum_value: &e::EnumvalueType,
) -> String {
    match domain {
        // Use the enum name as a qualifier if we're in the C++ domain
        Domain::CPlusPlus => format!("{enum_name}::{}", enum_value.name),
        // Otherwise all we want is the enumerator name
        Domain::C => enum_value.name.to_string(),
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
