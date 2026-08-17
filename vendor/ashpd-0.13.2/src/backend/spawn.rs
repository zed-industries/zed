#[cfg(feature = "tokio")]
pub struct TokioSpawner;

#[cfg(feature = "tokio")]
impl futures_util::task::Spawn for TokioSpawner {
    fn spawn_obj(
        &self,
        future: futures_util::task::FutureObj<'static, ()>,
    ) -> std::result::Result<(), futures_util::task::SpawnError> {
        tokio::spawn(future);
        Ok(())
    }
}
