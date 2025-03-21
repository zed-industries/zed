use language::ManifestName;

pub trait ManifestProvider: Send + Sync {
    fn name(&self) -> ManifestName;
    fn manifest_path(
        &self,
        _path: &Path,
        _ancestor_depth: usize,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Option<Arc<Path>>;
}

struct ManifestStoreState {
    providers: HashMap<ManifestName, Arc<dyn ManifestProvider>>,
}

struct ManifestStore(Arc<RwLock<ManifestStoreState>>);
