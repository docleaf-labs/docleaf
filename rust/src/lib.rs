mod doxygen;
mod nodes;
mod xml;

use std::collections::HashMap;
use std::path::PathBuf;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::doxygen::index::elements::{CompoundKind, DoxygenType, MemberKind};
use crate::nodes::Node;

/// Cache for xml files so that we don't have to keep re-reading them
#[pyclass]
struct Cache {
    index_cache: HashMap<PathBuf, DoxygenType>,
}

#[pymethods]
impl Cache {
    #[new]
    fn new() -> Self {
        Self {
            index_cache: HashMap::new(),
        }
    }
}

impl Cache {
    fn parse_index(&mut self, path: PathBuf) -> anyhow::Result<&DoxygenType> {
        // TODO: Figure out how to avoid double lookup - previous attempt led to borrow checker errors
        if self.index_cache.contains_key(&path) {
            return Ok(self.index_cache.get(&path).unwrap());
        } else {
            let info = {
                let info = doxygen::index::parse_file(&path)?;
                self.index_cache.insert(path.clone(), info);

                // Can safely unwrap as we've just inserted it
                self.index_cache.get(&path).unwrap()
            };

            Ok(info)
        }
    }
}

#[pyfunction]
fn render_class(name: String, path: String, cache: &mut Cache) -> PyResult<Vec<Node>> {
    tracing::info!("render_class {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = std::fs::canonicalize(xml_path.join("index.xml"))?;

    let index = cache.parse_index(index_xml_path)?;

    let compound = index
        .compound
        .iter()
        .find(|compound| compound.name == name && compound.kind == CompoundKind::Class);

    match compound {
        Some(compound) => {
            let ref_id = &compound.ref_id;
            let compound_xml_path = std::fs::canonicalize(xml_path.join(format!("{ref_id}.xml")))?;
            let root = doxygen::compound::parse_file(&compound_xml_path)?;

            Ok(doxygen::render::render_compound(root))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find class matching '{name}'"
        ))),
    }
}

#[pyfunction]
fn render_struct(name: String, path: String, cache: &mut Cache) -> PyResult<Vec<Node>> {
    tracing::info!("render_struct {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = std::fs::canonicalize(xml_path.join("index.xml"))?;

    let index = doxygen::index::parse_file(&index_xml_path)?;

    let compound = index
        .compound
        .iter()
        .find(|compound| compound.name == name && compound.kind == CompoundKind::Struct);

    match compound {
        Some(compound) => {
            let ref_id = &compound.ref_id;
            let compound_xml_path = std::fs::canonicalize(xml_path.join(format!("{ref_id}.xml")))?;
            let root = doxygen::compound::parse_file(&compound_xml_path)?;

            Ok(doxygen::render::render_compound(root))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find struct matching '{name}'"
        ))),
    }
}

#[pyfunction]
fn render_function(name: String, path: String, cache: &mut Cache) -> PyResult<Vec<Node>> {
    tracing::info!("render_function {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = std::fs::canonicalize(xml_path.join("index.xml"))?;

    let index = doxygen::index::parse_file(&index_xml_path)?;

    let found = index.compound.iter().find_map(|compound| {
        let member = compound
            .member
            .iter()
            .find(|member| member.name == name && member.kind == MemberKind::Function);

        member.map(|member| (compound, member))
    });

    //  tracing::info!("found member ref_id {}", member.ref_id);

    match found {
        Some((compound, member)) => {
            let ref_id = &compound.ref_id;
            let compound_xml_path = std::fs::canonicalize(xml_path.join(format!("{ref_id}.xml")))?;
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
    {
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();
    }

    module.add_class::<Cache>()?;

    module.add_wrapped(pyo3::wrap_pyfunction!(render_class))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(render_struct))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(render_function))?;

    Ok(())
}
