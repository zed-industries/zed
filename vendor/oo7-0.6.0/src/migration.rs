use crate::{AsAttributes, Result, dbus::Service, file::UnlockedKeyring};

/// Helper to migrate your secrets from the host Secret Service
/// to the sandboxed file backend.
///
/// If the migration is successful, the items are removed from the host
/// Secret Service.
pub async fn migrate(attributes: Vec<impl AsAttributes>, replace: bool) -> Result<()> {
    let service = Service::new().await?;
    let secret = crate::Secret::from(
        ashpd::desktop::secret::retrieve()
            .await
            .map_err(crate::file::Error::from)?,
    );
    let file_backend =
        match UnlockedKeyring::load(crate::file::api::Keyring::default_path()?, secret).await {
            Ok(file) => Ok(file),
            Err(super::file::Error::Portal(ashpd::Error::PortalNotFound(_))) => {
                #[cfg(feature = "tracing")]
                tracing::debug!("Portal not available, no migration to do");
                return Ok(());
            }
            Err(err) => Err(err),
        }?;

    let collection = service.default_collection().await?;
    let mut all_items = Vec::default();

    for attrs in attributes {
        let items = collection.search_items(&attrs).await?;
        all_items.extend(items);
    }
    let mut new_items = Vec::with_capacity(all_items.capacity());

    for item in all_items.iter() {
        let attributes = item.attributes().await?;
        let label = item.label().await?;
        let secret = item.secret().await?;

        new_items.push((label, attributes, secret, replace));
    }

    file_backend.create_items(new_items).await?;

    // Delete items from source after successful creation in destination
    let mut deletion_errors = Vec::new();
    for item in all_items.iter() {
        if let Err(e) = item.delete(None).await {
            deletion_errors.push(e);
        }
    }

    // Report deletion failures - partial migration is still an error condition
    if !deletion_errors.is_empty() {
        #[cfg(feature = "tracing")]
        tracing::error!(
            "Migration partially failed: {} items could not be deleted from source",
            deletion_errors.len()
        );
        return Err(deletion_errors.into_iter().next().unwrap().into());
    }

    Ok(())
}
