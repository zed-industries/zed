use crate::{
	accessible::AccessibleProxy, action::ActionProxy, application::ApplicationProxy,
	cache::CacheProxy, collection::CollectionProxy, component::ComponentProxy,
	document::DocumentProxy, editable_text::EditableTextProxy, hyperlink::HyperlinkProxy,
	hypertext::HypertextProxy, image::ImageProxy, selection::SelectionProxy, table::TableProxy,
	table_cell::TableCellProxy, text::TextProxy, value::ValueProxy, AtspiError,
};
use atspi_common::{Interface, InterfaceSet, Result};

/// Acquire the other interface proxies an object may have implemented.
///
/// Equip objects with conversions to proxies of the objects' implemented interfaces
/// by extending `AccessibleProxy`.
///
/// The `proxies` method returns a `Proxies` struct.
pub trait ProxyExt<'a> {
	/// Get `Proxies` for the current object.
	fn proxies(&self) -> impl std::future::Future<Output = Result<Proxies<'a>>>;
}

/// An object for safe conversion to the related interface proxies.
#[derive(Clone, Debug)]
pub struct Proxies<'a> {
	interfaces: InterfaceSet,
	proxy: zbus::Proxy<'a>,
}

impl<'a> ProxyExt<'a> for AccessibleProxy<'a> {
	async fn proxies(&self) -> Result<Proxies<'a>> {
		let iface_set: InterfaceSet = self.get_interfaces().await?;
		let proxy = self.inner().clone();

		Ok(Proxies { interfaces: iface_set, proxy })
	}
}

impl<'a> Proxies<'a> {
	/// Get the `Action` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn action(&self) -> Result<ActionProxy<'a>> {
		if self.interfaces.contains(Interface::Action) {
			Ok(ActionProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Action"))
		}
	}

	/// Get the `Application` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn application(&self) -> Result<ApplicationProxy<'a>> {
		if self.interfaces.contains(Interface::Application) {
			Ok(ApplicationProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Application"))
		}
	}

	/// Get the `Cache` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn cache(&self) -> Result<CacheProxy<'a>> {
		if self.interfaces.contains(Interface::Cache) {
			Ok(CacheProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Cache"))
		}
	}

	/// Get the `Collection` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn collection(&self) -> Result<CollectionProxy<'a>> {
		if self.interfaces.contains(Interface::Collection) {
			Ok(CollectionProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Collection"))
		}
	}

	/// Get the `Component` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn component(&self) -> Result<ComponentProxy<'a>> {
		if self.interfaces.contains(Interface::Component) {
			Ok(ComponentProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Component"))
		}
	}

	/// Get the `Document` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn document(&self) -> Result<DocumentProxy<'a>> {
		if self.interfaces.contains(Interface::Document) {
			Ok(DocumentProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Document"))
		}
	}

	/// Get the `EditableText` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn editable_text(&self) -> Result<EditableTextProxy<'a>> {
		if self.interfaces.contains(Interface::EditableText) {
			Ok(EditableTextProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("EditableText"))
		}
	}

	/// Get the `Hyperlink` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn hyperlink(&self) -> Result<HyperlinkProxy<'a>> {
		if self.interfaces.contains(Interface::Hyperlink) {
			Ok(HyperlinkProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Hyperlink"))
		}
	}

	/// Get the `Hypertext` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn hypertext(&self) -> Result<HypertextProxy<'a>> {
		if self.interfaces.contains(Interface::Hypertext) {
			Ok(HypertextProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Hypertext"))
		}
	}

	/// Get the `Image` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn image(&self) -> Result<ImageProxy<'a>> {
		if self.interfaces.contains(Interface::Image) {
			Ok(ImageProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Image"))
		}
	}

	/// Get the `Registry` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn selection(&self) -> Result<SelectionProxy<'a>> {
		if self.interfaces.contains(Interface::Selection) {
			Ok(SelectionProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Selection"))
		}
	}

	/// Get the `Table` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn table(&self) -> Result<TableProxy<'a>> {
		if self.interfaces.contains(Interface::Table) {
			Ok(TableProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Table"))
		}
	}

	/// Get the `TableCell` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn table_cell(&self) -> Result<TableCellProxy<'a>> {
		if self.interfaces.contains(Interface::TableCell) {
			Ok(TableCellProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("TableCell"))
		}
	}

	/// Get the `Text` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn text(&self) -> Result<TextProxy<'a>> {
		if self.interfaces.contains(Interface::Text) {
			Ok(TextProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Text"))
		}
	}

	/// Get the `Value` interface proxy.
	///
	/// # Errors
	///
	/// Returns an error if the interface is not available.
	pub async fn value(&self) -> Result<ValueProxy<'a>> {
		if self.interfaces.contains(Interface::Value) {
			Ok(ValueProxy::builder(self.proxy.connection())
				.cache_properties(zbus::proxy::CacheProperties::No)
				.destination(self.proxy.destination())?
				.path(self.proxy.path())?
				.build()
				.await?)
		} else {
			Err(AtspiError::InterfaceNotAvailable("Value"))
		}
	}
}
