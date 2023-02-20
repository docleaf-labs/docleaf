use std::str::FromStr;

use anyhow::anyhow;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

pub fn parse_text(reader: &mut Reader<&[u8]>) -> anyhow::Result<String> {
    match reader.read_event() {
        Ok(Event::Text(text)) => {
            String::from_utf8(text.to_vec()).map_err(|err| anyhow!("{:?}", err))
        }
        event => Err(anyhow!(
            "parse_text called on non-text node resulting in event: {event:?}"
        )),
    }
}

pub fn get_attribute<'a>(name: &[u8], tag: &'a BytesStart<'a>) -> anyhow::Result<Attribute<'a>> {
    tag.attributes()
        .find(|result| {
            result
                .as_ref()
                .map(|attr| attr.key.local_name().as_ref() == name)
                .unwrap_or(false)
        })
        .ok_or(anyhow!("Unable to find refid"))?
        .map_err(|err| anyhow!("{:?}", err))
}

pub fn get_attribute_string<'a>(name: &[u8], tag: &'a BytesStart<'a>) -> anyhow::Result<String> {
    let attr = get_attribute(name, tag)?;
    Ok(String::from_utf8(attr.value.into_owned())?)
}

pub fn get_attribute_enum<'a, T: FromStr>(
    name: &[u8],
    tag: &'a BytesStart<'a>,
) -> anyhow::Result<T> {
    let attr = get_attribute(name, tag)?;
    let str = String::from_utf8(attr.value.into_owned())?;
    T::from_str(&str).map_err(|_| anyhow::anyhow!("Failed to parse string '{str}' to enum"))
}
