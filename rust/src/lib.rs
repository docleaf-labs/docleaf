mod doxygen;
mod nodes;
mod xml;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use std::path::PathBuf;

use crate::doxygen::index::elements::{CompoundKind, MemberKind};
use crate::nodes::Node;

#[pyfunction]
fn render_class(name: String, path: String) -> PyResult<Vec<Node>> {
    log::info!("render_class {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = xml_path.join("index.xml");

    let index = doxygen::index::parse_file(&index_xml_path)?;

    let compound = index
        .compound
        .iter()
        .find(|compound| compound.name == name && compound.kind == CompoundKind::Class);

    match compound {
        Some(compound) => {
            let ref_id = &compound.ref_id;
            let compound_xml_path = xml_path.join(format!("{ref_id}.xml"));
            let root = doxygen::compound::parse_file(&compound_xml_path)?;

            Ok(doxygen::render::render_compound(root))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find class matching '{name}'"
        ))),
    }
}

#[pyfunction]
fn render_struct(name: String, path: String) -> PyResult<Vec<Node>> {
    log::info!("render_struct {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = xml_path.join("index.xml");

    let index = doxygen::index::parse_file(&index_xml_path)?;

    let compound = index
        .compound
        .iter()
        .find(|compound| compound.name == name && compound.kind == CompoundKind::Struct);

    match compound {
        Some(compound) => {
            let ref_id = &compound.ref_id;
            let compound_xml_path = xml_path.join(format!("{ref_id}.xml"));
            let root = doxygen::compound::parse_file(&compound_xml_path)?;

            Ok(doxygen::render::render_compound(root))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find struct matching '{name}'"
        ))),
    }
}

#[pyfunction]
fn render_function(name: String, path: String) -> PyResult<Vec<Node>> {
    log::info!("render_function {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = xml_path.join("index.xml");

    let index = doxygen::index::parse_file(&index_xml_path)?;

    let found = index.compound.iter().find_map(|compound| {
        let member = compound
            .member
            .iter()
            .find(|member| member.name == name && member.kind == MemberKind::Function);

        member.map(|member| (compound, member))
    });

    //  log::info!("found member ref_id {}", member.ref_id);

    match found {
        Some((compound, member)) => {
            let ref_id = &compound.ref_id;
            let compound_xml_path = xml_path.join(format!("{ref_id}.xml"));
            let root = doxygen::compound::parse_file(&compound_xml_path)?;

            Ok(doxygen::render::render_member(root, &member.ref_id))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find function matching '{name}'"
        ))),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn backend(_py: Python, module: &PyModule) -> PyResult<()> {
    env_logger::init();

    module.add_wrapped(pyo3::wrap_pyfunction!(render_class))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(render_struct))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(render_function))?;
    Ok(())
}
