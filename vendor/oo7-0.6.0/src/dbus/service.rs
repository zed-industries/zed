use std::sync::Arc;

use ashpd::WindowIdentifier;
use futures_util::{Stream, StreamExt};
use zbus::zvariant::OwnedObjectPath;

use super::{Algorithm, Collection, Error, ServiceError, api};
use crate::Key;

/// The entry point of communicating with a [`org.freedesktop.Secrets`](https://specifications.freedesktop.org/secret-service-spec/latest/index.html) implementation.
///
/// It will automatically create a session for you and allow you to retrieve
/// collections or create new ones.
///
/// Certain actions requires on the Secret Service implementation requires a
/// user prompt to complete like creating a collection, locking or unlocking a
/// collection. The library handles that automatically for you.
///
/// ```no_run
/// use oo7::dbus::Service;
///
/// # async fn run() -> oo7::Result<()> {
/// let service = Service::new().await?;
/// let collection = service.default_collection().await?;
/// // Do something with the collection
///
/// #   Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Service {
    inner: Arc<api::Service>,
    aes_key: Option<Arc<Key>>,
    session: Arc<api::Session>,
    algorithm: Algorithm,
}

impl Service {
    /// The default collection alias.
    ///
    /// In general, you are supposed to use [`Service::default_collection`].
    pub const DEFAULT_COLLECTION: &'static str = "default";

    /// A session collection.
    ///
    /// The collection is cleared when the user ends the session.
    pub const SESSION_COLLECTION: &'static str = "session";

    /// Create a new instance of the Service, an encrypted communication would
    /// be attempted first and would fall back to a plain one if that fails.
    pub async fn new() -> Result<Self, Error> {
        let service = match Self::encrypted().await {
            Ok(service) => Ok(service),
            Err(Error::ZBus(zbus::Error::MethodError(..))) => Self::plain().await,
            Err(Error::Service(ServiceError::ZBus(zbus::Error::MethodError(..)))) => {
                Self::plain().await
            }
            Err(e) => Err(e),
        }?;
        Ok(service)
    }

    /// Create a new instance of the Service with plain algorithm.
    pub async fn plain() -> Result<Self, Error> {
        Self::with_algorithm(Algorithm::Plain).await
    }

    /// Create a new instance of the Service with encrypted algorithm.
    pub async fn encrypted() -> Result<Self, Error> {
        Self::with_algorithm(Algorithm::Encrypted).await
    }

    /// Create a new instance of the Service.
    async fn with_algorithm(algorithm: Algorithm) -> Result<Self, Error> {
        let cnx = zbus::connection::Builder::session()?
            .method_timeout(std::time::Duration::from_secs(30))
            .build()
            .await?;

        let service = Arc::new(api::Service::new(&cnx).await?);

        let (aes_key, session) = match algorithm {
            Algorithm::Plain => {
                #[cfg(feature = "tracing")]
                tracing::debug!("Starting an unencrypted Secret Service session");
                let (_service_key, session) = service.open_session(None).await?;
                (None, session)
            }
            Algorithm::Encrypted => {
                #[cfg(feature = "tracing")]
                tracing::debug!("Starting an encrypted Secret Service session");
                let private_key = Key::generate_private_key()?;
                let public_key = Key::generate_public_key(&private_key)?;
                let (service_key, session) = service.open_session(Some(public_key)).await?;
                let aes_key = service_key
                    .map(|service_key| Key::generate_aes_key(&private_key, &service_key))
                    .transpose()?
                    .map(Arc::new);

                (aes_key, session)
            }
        };

        Ok(Self {
            aes_key,
            inner: service,
            session: Arc::new(session),
            algorithm,
        })
    }

    /// Retrieve the default collection if any or create one.
    ///
    /// The created collection label is set to `Default`. If you want to
    /// translate the string, use [Self::with_alias_or_create] instead.
    pub async fn default_collection(&self) -> Result<Collection, Error> {
        // TODO: Figure how to make those labels translatable
        self.with_alias_or_create(Self::DEFAULT_COLLECTION, "Default", None)
            .await
    }

    /// Retrieve the session collection if any or create one.
    ///
    /// The created collection label is set to `Default`. If you want to
    /// translate the string, use [Self::with_alias_or_create] instead.
    pub async fn session_collection(&self) -> Result<Collection, Error> {
        // TODO: Figure how to make those labels translatable
        self.with_alias_or_create(Self::SESSION_COLLECTION, "Session", None)
            .await
    }

    pub async fn with_alias_or_create(
        &self,
        alias: &str,
        label: &str,
        window_id: Option<WindowIdentifier>,
    ) -> Result<Collection, Error> {
        match self.with_alias(alias).await {
            Ok(Some(collection)) => Ok(collection),
            Ok(None) => self.create_collection(label, Some(alias), window_id).await,
            Err(err) => Err(err),
        }
    }

    /// Find a collection with it alias.
    ///
    /// Applications should make use of [`Service::default_collection`] instead.
    pub async fn with_alias(&self, alias: &str) -> Result<Option<Collection>, Error> {
        Ok(self
            .inner
            .read_alias(alias)
            .await?
            .map(|collection| self.new_collection(collection)))
    }

    /// Get a list of all the available collections.
    pub async fn collections(&self) -> Result<Vec<Collection>, Error> {
        Ok(self
            .inner
            .collections()
            .await?
            .into_iter()
            .map(|collection| self.new_collection(collection))
            .collect::<Vec<_>>())
    }

    /// Create a new collection.
    pub async fn create_collection(
        &self,
        label: &str,
        alias: Option<&str>,
        window_id: Option<WindowIdentifier>,
    ) -> Result<Collection, Error> {
        self.inner
            .create_collection(label, alias, window_id)
            .await
            .map(|collection| self.new_collection(collection))
    }

    /// Find a collection with it label.
    pub async fn with_label(&self, label: &str) -> Result<Option<Collection>, Error> {
        let collections = self.collections().await?;
        for collection in collections {
            if collection.label().await? == label {
                return Ok(Some(collection));
            }
        }
        Ok(None)
    }

    /// Stream yielding when new collections get created
    pub async fn receive_collection_created(
        &self,
    ) -> Result<impl Stream<Item = Collection> + '_, Error> {
        Ok(self
            .inner
            .receive_collection_created()
            .await?
            .map(|collection| self.new_collection(collection)))
    }

    /// Stream yielding when existing collections get changed
    pub async fn receive_collection_changed(
        &self,
    ) -> Result<impl Stream<Item = Collection> + '_, Error> {
        Ok(self
            .inner
            .receive_collection_changed()
            .await?
            .map(|collection| self.new_collection(collection)))
    }

    /// Stream yielding when existing collections get deleted
    pub async fn receive_collection_deleted(
        &self,
    ) -> Result<impl Stream<Item = OwnedObjectPath>, Error> {
        self.inner.receive_collection_deleted().await
    }

    // Get public `Collection` from `api::Collection`
    fn new_collection(&self, collection: api::Collection) -> Collection {
        Collection::new(
            Arc::clone(&self.inner),
            Arc::clone(&self.session),
            self.algorithm,
            collection,
            self.aes_key.clone(), // Cheap clone, it is an Arc,
        )
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        // Only close the session if this is the last reference to it
        if Arc::strong_count(&self.session) == 1 {
            let session = Arc::clone(&self.session);
            #[cfg(feature = "tokio")]
            {
                tokio::spawn(async move {
                    let _ = session.close().await;
                });
            }
            #[cfg(feature = "async-std")]
            {
                blocking::unblock(move || {
                    futures_lite::future::block_on(async move {
                        let _ = session.close().await;
                    })
                })
                .detach();
            }
        }
    }
}
