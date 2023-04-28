use crate::doxygen::compound::generated as e;

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
            /*
            let name = &member_def.name;

            match member_def.type_ {
                Some(ref type_) => {
                    let type_ = render_linked_text_type(type_);
                    format!("{type_} {name}()")
                }
                None => {
                    format!("{name}()")
                }
            }
            */
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
