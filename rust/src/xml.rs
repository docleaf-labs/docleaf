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

pub fn get_optional_attribute<'a>(
    name: &[u8],
    tag: &'a BytesStart<'a>,
) -> anyhow::Result<Option<Attribute<'a>>> {
    Ok(tag
        .attributes()
        .find(|result| {
            result
                .as_ref()
                .map(|attr| attr.key.local_name().as_ref() == name)
                .unwrap_or(false)
        })
        .transpose()?)
}

pub fn get_attribute<'a>(name: &[u8], tag: &'a BytesStart<'a>) -> anyhow::Result<Attribute<'a>> {
    get_optional_attribute(name, tag)?
        .ok_or_else(|| anyhow!("Unable to find {}", String::from_utf8_lossy(name)))
}

pub fn get_optional_attribute_string<'a>(
    name: &[u8],
    tag: &'a BytesStart<'a>,
) -> anyhow::Result<Option<String>> {
    Ok(get_optional_attribute(name, tag)?
        .map(|attr| String::from_utf8(attr.value.into_owned()))
        .transpose()?)
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

pub fn get_optional_attribute_enum<'a, T: FromStr>(
    name: &[u8],
    tag: &'a BytesStart<'a>,
) -> anyhow::Result<Option<T>> {
    let attr = get_attribute(name, tag)?;
    let str = String::from_utf8(attr.value.into_owned())?;
    Ok(get_optional_attribute(name, tag)?
        .map(|attr| {
            String::from_utf8(attr.value.into_owned())
                .map_err(|_| anyhow::anyhow!("Failed to parse string '{str}' to enum"))
                .and_then(|str| {
                    T::from_str(&str)
                        .map_err(|_| anyhow::anyhow!("Failed to parse string '{str}' to enum"))
                })
        })
        .transpose()?)
}
