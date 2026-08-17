use super::MakeSpawnReady;
use tower_layer::Layer;

/// Spawns tasks to drive its inner service to readiness.
#[derive(Debug, Clone, Default)]
pub struct SpawnReadyLayer;

impl SpawnReadyLayer {
    /// Builds a [`SpawnReadyLayer`] with the default executor.
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for SpawnReadyLayer {
    type Service = MakeSpawnReady<S>;

    fn layer(&self, service: S) -> Self::Service {
        MakeSpawnReady::new(service)
    }
}
