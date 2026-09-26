use crate::SandboxError;
use anyhow::Result;
use futures::future::BoxFuture;
use std::{future::Future, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WslHelperDownloadRequest {
    pub distro: String,
    pub channel: String,
    pub version: String,
}

pub type WslHelperDownloadGate =
    Arc<dyn Fn(WslHelperDownloadRequest) -> BoxFuture<'static, Result<()>> + Send + Sync>;

pub(super) async fn prepare_helper(
    request: WslHelperDownloadRequest,
    gate: Option<&WslHelperDownloadGate>,
    probe: impl Future<Output = Result<Option<String>>>,
    download: impl AsyncFnOnce() -> Result<String>,
) -> Result<String> {
    if let Some(path) = probe.await? {
        return Ok(path);
    }
    let purpose = format!(
        "Linux Zed {} {} sandbox helper in WSL {}",
        request.channel, request.version, request.distro
    );
    let gate = gate.ok_or_else(|| SandboxError::DownloadDenied(purpose.clone()))?;
    gate(request)
        .await
        .map_err(|error| SandboxError::DownloadDenied(format!("{purpose}: {error:#}")))?;
    download().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{channel::oneshot, executor::block_on};
    use std::sync::{
        Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    };

    #[test]
    fn helper_cache_reuse_and_missing_authority() {
        for cached in [true, false] {
            let downloads = AtomicUsize::new(0);
            let result = block_on(prepare_helper(
                WslHelperDownloadRequest {
                    distro: "Ubuntu".to_string(),
                    channel: "stable".to_string(),
                    version: "1.0.0".to_string(),
                },
                None,
                async { Ok(cached.then(|| "/cached/zed-editor".to_string())) },
                async || {
                    downloads.fetch_add(1, Ordering::SeqCst);
                    Ok("/downloaded/zed-editor".to_string())
                },
            ));
            if cached {
                assert_eq!(result.unwrap(), "/cached/zed-editor");
            } else {
                assert_eq!(
                    crate::map_anyhow_error(result.unwrap_err().context("wrapping terminal")),
                    SandboxError::DownloadDenied(
                        "Linux Zed stable 1.0.0 sandbox helper in WSL Ubuntu".to_string()
                    )
                );
            }
            assert_eq!(downloads.load(Ordering::SeqCst), 0);
        }
    }

    #[test]
    fn helper_authorization_is_after_probe_and_scoped_to_distro_and_release() {
        for revoke in [true, false] {
            let permitted = Arc::new(AtomicBool::new(true));
            let requests = Arc::new(Mutex::new(Vec::new()));
            let downloads = AtomicUsize::new(0);
            let request = WslHelperDownloadRequest {
                distro: "Debian".to_string(),
                channel: "preview".to_string(),
                version: "2.0.0".to_string(),
            };
            let gate: WslHelperDownloadGate = Arc::new({
                let permitted = permitted.clone();
                let requests = requests.clone();
                move |request| {
                    let permitted = permitted.clone();
                    let requests = requests.clone();
                    Box::pin(async move {
                        requests.lock().unwrap().push(request);
                        anyhow::ensure!(permitted.load(Ordering::SeqCst), "revoked");
                        Ok(())
                    })
                }
            });
            let result = block_on(async {
                let (resume, probe) = oneshot::channel();
                let mut preparation = Box::pin(prepare_helper(
                    request.clone(),
                    Some(&gate),
                    async {
                        probe.await?;
                        Ok(None)
                    },
                    async || {
                        downloads.fetch_add(1, Ordering::SeqCst);
                        Ok("/downloaded/zed-editor".to_string())
                    },
                ));
                assert!(futures::poll!(&mut preparation).is_pending());
                assert_eq!(
                    *requests.lock().unwrap(),
                    Vec::<WslHelperDownloadRequest>::new()
                );
                assert_eq!(downloads.load(Ordering::SeqCst), 0);
                permitted.store(!revoke, Ordering::SeqCst);
                resume.send(()).unwrap();
                preparation.await
            });
            assert_eq!(*requests.lock().unwrap(), vec![request]);
            if revoke {
                assert_eq!(
                    crate::map_anyhow_error(result.unwrap_err().context("wrapping terminal")),
                    SandboxError::DownloadDenied(
                        "Linux Zed preview 2.0.0 sandbox helper in WSL Debian: revoked".to_string()
                    )
                );
                assert_eq!(downloads.load(Ordering::SeqCst), 0);
            } else {
                assert_eq!(result.unwrap(), "/downloaded/zed-editor");
                assert_eq!(downloads.load(Ordering::SeqCst), 1);
            }
        }
    }
}
