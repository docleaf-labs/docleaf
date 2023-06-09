use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use pyo3::prelude::*;

use crate::doxygen::compound::generated as compound;
use crate::doxygen::index::generated as index;

pub trait Cache {
    fn parse_index(&self, path: PathBuf) -> anyhow::Result<Arc<index::DoxygenType>>;
    fn parse_compound(&self, path: PathBuf) -> anyhow::Result<Arc<compound::DoxygenType>>;
}

/// Cache class exposed to python with no function methods beyond the
/// constructor. Used to hold the Arc Mutex for the inner cache so that
/// we can more easily mutate with actual cache data without worrying
/// about accessing it through a &mut the whole time as it is easy to
/// fall foul of the borrow checker with that.
#[pyclass]
pub struct FileCache {
    inner: Arc<Mutex<CacheInner>>,
}

#[pymethods]
impl FileCache {
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
            let info = crate::doxygen::index::parse_file(&path)?;
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
            let info = crate::doxygen::compound::parse_file(&path)?;
            let info = Arc::new(info);
            self.compound_cache.insert(path, info.clone());
            Ok(info)
        }
    }
}

/// Light weight for cloning due to Arcs
#[pyclass]
#[derive(Clone)]
pub struct TrackedCache {
    inner: Arc<Mutex<CacheInner>>,
    xml_paths: Arc<Mutex<HashSet<PathBuf>>>,
}

#[pymethods]
impl TrackedCache {
    #[new]
    fn new(cache: &FileCache) -> Self {
        Self {
            inner: cache.inner.clone(),
            xml_paths: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Returns a copy of the paths to xml files as strings
    fn xml_paths(&self) -> PyResult<Vec<String>> {
        let xml_paths = self.xml_paths.lock().unwrap();
        Ok(xml_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect())
    }
}

impl Cache for TrackedCache {
    /// Does not track the consumed file
    fn parse_index(&self, xml_path: PathBuf) -> anyhow::Result<Arc<index::DoxygenType>> {
        let mut cache = self.inner.lock().unwrap();
        cache.parse_index(xml_path)
    }

    fn parse_compound(&self, xml_path: PathBuf) -> anyhow::Result<Arc<compound::DoxygenType>> {
        let mut xml_paths = self.xml_paths.lock().unwrap();
        xml_paths.insert(xml_path.clone());

        let mut cache = self.inner.lock().unwrap();
        cache.parse_compound(xml_path)
    }
}
