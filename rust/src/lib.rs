mod cache;
mod doxygen;
mod xml;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::cache::{Cache, FileCache, TrackedCache};
use crate::doxygen::compound::generated as compound;
use crate::doxygen::index::generated as index;
use crate::doxygen::nodes::{Domain, Node};
use crate::doxygen::render::Skip;

#[pyclass]
struct Context {
    pub project_root: PathBuf,
    pub skip_settings: Vec<Skip>,
    pub domain_by_extension: HashMap<String, Domain>,
}

#[pymethods]
impl Context {
    #[new]
    fn new(
        project_root: String,
        skip_settings: Vec<String>,
        domain_by_extension: HashMap<String, String>,
    ) -> PyResult<Self> {
        let domain_by_extension = Domain::create_lookup(domain_by_extension)
            .map_err(|err| PyValueError::new_err(format!("{}", err)))?;

        let skip_settings = skip_settings
            .into_iter()
            .map(|value| {
                if value == "members:all_caps" {
                    Ok(Skip::MembersAllCaps)
                } else {
                    match value.strip_prefix("xml-nodes:") {
                        Some(node) => Ok(Skip::XmlNode(node.to_string())),
                        None => Err(format!("Unrecognised skip setting: {value}")),
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| PyValueError::new_err(format!("{}", err)))?;

        Ok(Self {
            project_root: PathBuf::from(project_root),
            skip_settings,
            domain_by_extension,
        })
    }
}

#[pyfunction]
fn render_class(
    name: String,
    path: String,
    context: &Context,
    cache: &TrackedCache,
) -> PyResult<Vec<Node>> {
    tracing::info!("render_class {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let xml_path = cwd.join(xml_directory);
    let index_xml_path = std::fs::canonicalize(xml_path.join("index.xml"))?;

    let mut xml_loader = XmlLoader::new(xml_path.clone(), (*cache).clone());

    let index = cache.parse_index(index_xml_path)?;

    let compound = index
        .compound
        .iter()
        .find(|compound| compound.name == name && compound.kind == index::CompoundKind::Class);

    match compound {
        Some(compound) => {
            let ref_id = &compound.refid;
            let compound_xml_path = std::fs::canonicalize(xml_path.join(format!("{ref_id}.xml")))?;
            let root = cache.parse_compound(compound_xml_path.clone())?;

            let context = doxygen::render::Context {
                project_root: context.project_root.clone(),
                domain: None,
                skip: context.skip_settings.clone(),
                extension_domain_lookup: context.domain_by_extension.clone(),
                enumerated_list_depth: 0,
            };
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
fn render_struct(
    name: String,
    path: String,
    context: &Context,
    cache: &TrackedCache,
) -> PyResult<Vec<Node>> {
    tracing::info!("render_struct {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let xml_path = cwd.join(xml_directory);
    let index_xml_path = std::fs::canonicalize(xml_path.join("index.xml"))?;

    let mut xml_loader = XmlLoader::new(xml_path.clone(), cache.clone());

    let index = doxygen::index::parse_file(&index_xml_path)?;

    let compound = index
        .compound
        .iter()
        .find(|compound| compound.name == name && compound.kind == index::CompoundKind::Struct);

    match compound {
        Some(compound) => {
            let ref_id = &compound.refid;
            let compound_xml_path = std::fs::canonicalize(xml_path.join(format!("{ref_id}.xml")))?;
            let root = cache.parse_compound(compound_xml_path.clone())?;

            let context = doxygen::render::Context {
                project_root: context.project_root.clone(),
                domain: None,
                skip: context.skip_settings.clone(),
                extension_domain_lookup: context.domain_by_extension.clone(),
                enumerated_list_depth: 0,
            };

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
    cache: &TrackedCache,
) -> PyResult<Vec<Node>> {
    tracing::info!("render_enum {} {}", name, path);
    render_member(name, index::MemberKind::Enum, path, context, cache)
}

#[pyfunction]
fn render_function(
    name: String,
    path: String,
    context: &Context,
    cache: &TrackedCache,
) -> PyResult<Vec<Node>> {
    tracing::info!("render_function {} {}", name, path);
    render_member(name, index::MemberKind::Function, path, context, cache)
}

fn render_member(
    name: String,
    kind: index::MemberKind,
    path: String,
    context: &Context,
    cache: &TrackedCache,
) -> PyResult<Vec<Node>> {
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let xml_path = cwd.join(xml_directory);
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
            let root = cache.parse_compound(compound_xml_path)?;

            let context = doxygen::render::Context {
                project_root: context.project_root.clone(),
                domain: None,
                skip: context.skip_settings.clone(),
                extension_domain_lookup: context.domain_by_extension.clone(),
                enumerated_list_depth: 0,
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
    cache: TrackedCache,
}

impl XmlLoader {
    pub fn new(root: PathBuf, cache: TrackedCache) -> Self {
        Self { root, cache }
    }

    pub fn load_index(&mut self) -> anyhow::Result<Arc<index::DoxygenType>> {
        let index_xml_path = std::fs::canonicalize(self.root.join("index.xml"))?;
        self.cache.parse_index(index_xml_path)
    }

    pub fn load(&mut self, ref_id: &str) -> anyhow::Result<Arc<compound::DoxygenType>> {
        let xml_path = std::fs::canonicalize(self.root.join(format!("{ref_id}.xml")))?;
        self.cache.parse_compound(xml_path)
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
    cache: &TrackedCache,
) -> PyResult<Vec<Node>> {
    tracing::info!("render_group {} {}", name, path);
    let xml_directory = PathBuf::from(path);

    let cwd = std::env::current_dir()?;
    let xml_path = cwd.join(xml_directory);

    let mut xml_loader = XmlLoader::new(xml_path, (*cache).clone());
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
                project_root: context.project_root.clone(),
                domain: None,
                skip: context.skip_settings.clone(),
                extension_domain_lookup: context.domain_by_extension.clone(),
                enumerated_list_depth: 0,
            };

            if content_only {
                let Some(ref compounddef) = root.compounddef else {
                    return Err(PyValueError::new_err("Not compounddef node found in xml file".to_string()));
                };

                let contents =
                    doxygen::compound::extract_compounddef_contents(compounddef, inner_groups);
                Ok(contents
                    .into_iter()
                    .map(|entry| {
                        doxygen::render::render_compounddef_content(
                            &context,
                            &compounddef.id,
                            &compounddef.kind,
                            entry,
                            inner_groups,
                            &mut xml_loader,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .flatten()
                    .collect())
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

    module.add_class::<FileCache>()?;
    module.add_class::<TrackedCache>()?;
    module.add_class::<Context>()?;

    module.add_wrapped(pyo3::wrap_pyfunction!(render_class))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(render_struct))?;

    module.add_wrapped(pyo3::wrap_pyfunction!(render_function))?;
    module.add_wrapped(pyo3::wrap_pyfunction!(render_enum))?;

    module.add_wrapped(pyo3::wrap_pyfunction!(render_group))?;

    Ok(())
}
