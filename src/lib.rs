mod doxygen;
mod nodes;
mod xml;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use std::path::PathBuf;

use crate::nodes::Node;

#[pyfunction]
fn render_class(name: String, path: String) -> PyResult<Vec<Node>> {
    log::trace!("extract_class {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = xml_path.join("index.xml");

    let index = doxygen::index::parse_file(&index_xml_path)?;

    let compound = index
        .compounds
        .iter()
        .find(|compound| compound.name == name && compound.kind == "class");

    match compound {
        Some(compound) => {
            let refid = &compound.refid;
            let compound_xml_path = xml_path.join(format!("{}.xml", refid));
            let compound = doxygen::compound::parse_file(&compound_xml_path)?;

            Ok(doxygen::render_class_compound(compound))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find class matching '{}'",
            name
        ))),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn backend(_py: Python, module: &PyModule) -> PyResult<()> {
    env_logger::init();

    module.add_wrapped(pyo3::wrap_pyfunction!(render_class))?;
    Ok(())
}
