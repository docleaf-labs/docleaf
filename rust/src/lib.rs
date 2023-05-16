mod doxygen;
mod xml;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::doxygen::compound::generated as compound;
use crate::doxygen::index::generated as index;
use crate::doxygen::nodes::Node;

/// Cache class exposed to python with no function methods beyond the
/// constructor. Used to hold the Arc Mutex for the inner cache so that
/// we can more easily mutate with actual cache data without worrying
/// about accessing it through a &mut the whole time as it is easy to
/// fall foul of the borrow checker with that.
#[pyclass]
struct Cache {
    inner: Arc<Mutex<CacheInner>>,
}

#[pymethods]
impl Cache {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CacheInner::new())),
        }
    }
}

/// Inner cache data which is held with an Arc Mutex by the exposed Cache to
/// make it easier to access and mutate.
///
/// Cache for xml files so that we don't have to keep re-reading them
pub struct CacheInner {
    index_cache: HashMap<PathBuf, Arc<index::DoxygenType>>,
    compound_cache: HashMap<PathBuf, Arc<compound::DoxygenType>>,
}

impl CacheInner {
    fn new() -> Self {
        Self {
            index_cache: HashMap::new(),
            compound_cache: HashMap::new(),
        }
    }
}

impl CacheInner {
    fn parse_index(&mut self, path: PathBuf) -> anyhow::Result<Arc<index::DoxygenType>> {
        // TODO: Figure out how to avoid double lookup - previous attempt led to borrow checker errors
        if self.index_cache.contains_key(&path) {
            return Ok(self.index_cache.get(&path).unwrap().clone());
        } else {
            let info = doxygen::index::parse_file(&path)?;
            let info = Arc::new(info);
            self.index_cache.insert(path, info.clone());
            Ok(info)
        }
    }

    fn parse_compound(&mut self, path: PathBuf) -> anyhow::Result<Arc<compound::DoxygenType>> {
        // TODO: Figure out how to avoid double lookup - previous attempt led to borrow checker errors
        if self.compound_cache.contains_key(&path) {
            return Ok(self.compound_cache.get(&path).unwrap().clone());
        } else {
            let info = doxygen::compound::parse_file(&path)?;
            let info = Arc::new(info);
            self.compound_cache.insert(path, info.clone());
            Ok(info)
        }
    }
}

#[pyclass]
struct Context {
    pub skip_xml_nodes: Vec<String>,
}

#[pymethods]
impl Context {
    #[new]
    fn new(skip_xml_nodes: Vec<String>) -> Self {
        Self { skip_xml_nodes }
    }
}

#[pyfunction]
fn render_class(name: String, path: String, cache: &Cache) -> PyResult<Vec<Node>> {
    tracing::info!("render_class {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = std::fs::canonicalize(xml_path.join("index.xml"))?;

    let mut xml_loader = XmlLoader::new(xml_path.clone(), cache.inner.clone());

    let index = {
        let mut cache = cache.inner.lock().unwrap();
        cache.parse_index(index_xml_path)?
    };

    let compound = index
        .compound
        .iter()
        .find(|compound| compound.name == name && compound.kind == index::CompoundKind::Class);

    match compound {
        Some(compound) => {
            let ref_id = &compound.refid;
            let compound_xml_path = std::fs::canonicalize(xml_path.join(format!("{ref_id}.xml")))?;
            let root = {
                let mut cache = cache.inner.lock().unwrap();
                cache.parse_compound(compound_xml_path)?
            };

            let context = doxygen::render::Context::default();
            let inner_groups = false;
            doxygen::render::render_compound(&context, root.as_ref(), inner_groups, &mut xml_loader)
                .map_err(|err| PyValueError::new_err(format!("{}", err)))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find class matching '{name}'"
        ))),
    }
}

#[pyfunction]
fn render_struct(name: String, path: String, cache: &Cache) -> PyResult<Vec<Node>> {
    tracing::info!("render_struct {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);
    let index_xml_path = std::fs::canonicalize(xml_path.join("index.xml"))?;

    let mut xml_loader = XmlLoader::new(xml_path.clone(), cache.inner.clone());

    let index = doxygen::index::parse_file(&index_xml_path)?;

    let compound = index
        .compound
        .iter()
        .find(|compound| compound.name == name && compound.kind == index::CompoundKind::Struct);

    match compound {
        Some(compound) => {
            let ref_id = &compound.refid;
            let compound_xml_path = std::fs::canonicalize(xml_path.join(format!("{ref_id}.xml")))?;
            let root = {
                let mut cache = cache.inner.lock().unwrap();
                cache.parse_compound(compound_xml_path)?
            };

            let context = doxygen::render::Context::default();
            let inner_groups = false;
            doxygen::render::render_compound(&context, root.as_ref(), inner_groups, &mut xml_loader)
                .map_err(|err| PyValueError::new_err(format!("{}", err)))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find struct matching '{name}'"
        ))),
    }
}

#[pyfunction]
fn render_enum(
    name: String,
    path: String,
    context: &Context,
    cache: &Cache,
) -> PyResult<Vec<Node>> {
    tracing::info!("render_enum {} {}", name, path);
    render_member(name, index::MemberKind::Enum, path, context, cache)
}

#[pyfunction]
fn render_function(
    name: String,
    path: String,
    context: &Context,
    cache: &Cache,
) -> PyResult<Vec<Node>> {
    tracing::info!("render_function {} {}", name, path);
    render_member(name, index::MemberKind::Function, path, context, cache)
}

fn render_member(
    name: String,
    kind: index::MemberKind,
    path: String,
    context: &Context,
    cache: &Cache,
) -> PyResult<Vec<Node>> {
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
            .find(|member| member.name == name && member.kind == kind);

        member.map(|member| (compound, member))
    });

    match found {
        Some((compound, member)) => {
            let ref_id = &compound.refid;
            let compound_xml_path = std::fs::canonicalize(xml_path.join(format!("{ref_id}.xml")))?;
            let root = {
                let mut cache = cache.inner.lock().unwrap();
                cache.parse_compound(compound_xml_path)?
            };

            let context = doxygen::render::Context {
                domain: None,
                skip_xml_nodes: context.skip_xml_nodes.clone(),
            };

            Ok(doxygen::render::render_member(
                &context,
                root.as_ref(),
                &member.refid,
            ))
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find {kind:?} matching '{name}'"
        ))),
    }
}

/// Abstraction to help with loading xml files from a particular folder and
/// caching the resulting parsed data in the cache
pub struct XmlLoader {
    root: PathBuf,
    cache: Arc<Mutex<CacheInner>>,
}

impl XmlLoader {
    pub fn new(root: PathBuf, cache: Arc<Mutex<CacheInner>>) -> Self {
        Self { root, cache }
    }

    pub fn load_index(&mut self) -> anyhow::Result<Arc<index::DoxygenType>> {
        let index_xml_path = std::fs::canonicalize(self.root.join("index.xml"))?;
        let mut cache = self.cache.lock().unwrap();
        cache.parse_index(index_xml_path)
    }

    pub fn load(&mut self, ref_id: &str) -> anyhow::Result<Arc<compound::DoxygenType>> {
        let xml_path = std::fs::canonicalize(self.root.join(format!("{ref_id}.xml")))?;
        let mut cache = self.cache.lock().unwrap();
        cache.parse_compound(xml_path)
    }
}

#[pyfunction]
fn render_group(
    name: String,
    path: String,
    context: &Context,
    content_only: bool,
    // TODO: Use 'filter' concept instead of passing this bool around
    inner_groups: bool,
    cache: &Cache,
) -> PyResult<Vec<Node>> {
    tracing::info!("render_group {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let source_directory = cwd.join("source");

    let xml_path = source_directory.join(xml_directory);

    let mut xml_loader = XmlLoader::new(xml_path, cache.inner.clone());
    let compound_ref_id = {
        let index = xml_loader.load_index()?;

        index
            .compound
            .iter()
            .find(|compound| compound.name == name && compound.kind == index::CompoundKind::Group)
            .map(|compound| compound.refid.clone())
    };

    match compound_ref_id {
        Some(compound_ref_id) => {
            let root = xml_loader.load(&compound_ref_id)?;

            tracing::debug!("Compound root: {root:?}");

            let context = doxygen::render::Context {
                domain: None,
                skip_xml_nodes: context.skip_xml_nodes.clone(),
            };

            if content_only {
                let Some(ref compounddef) = root.compounddef else {
                    return Err(PyValueError::new_err("Not compounddef node found in xml file".to_string()));
                };

                let contents =
                    doxygen::compound::extract_compounddef_contents(compounddef, inner_groups);
                let nodes = contents
                    .into_iter()
                    .map(|entry| {
                        doxygen::render::render_compounddef_content(
                            &context,
                            entry,
                            inner_groups,
                            &mut xml_loader,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .flatten()
                    .collect();

                Ok(nodes)
            } else {
                doxygen::render::render_compound(
                    &context,
                    root.as_ref(),
                    inner_groups,
                    &mut xml_loader,
                )
                .map_err(|err| PyValueError::new_err(format!("{}", err)))
            }
        }
        None => Err(PyValueError::new_err(format!(
            "Unable to find group matching '{name}'"
        ))),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn backend(_py: Python, module: &PyModule) -> PyResult<()> {
    {
        use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::WARN.into())
                    .with_env_var("DOCLEAF_LOG")
                    .from_env_lossy(),
            )
            .init();
    }

    module.add_class::<Cache>()?;
    module.add_class::<Context>()?;

    module.add_wrapped(pyo3::wrap_pyfunction!(render_class))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(render_struct))?;

    module.add_wrapped(pyo3::wrap_pyfunction!(render_function))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(render_enum))?;

    module.add_wrapped(pyo3::wrap_pyfunction!(render_group))?;

    Ok(())
}
