mod iadvisesink;
mod ibindctx;
mod idataobject;
mod idroptarget;
mod imoniker;
mod ipersist;
mod ipersistfile;
mod ipersiststream;
mod ipicture;
mod isequentialstream;
mod istorage;
mod istream;
mod iunknown;

pub mod decl {
	pub use super::iadvisesink::IAdviseSink;
	pub use super::ibindctx::IBindCtx;
	pub use super::idataobject::IDataObject;
	pub use super::idroptarget::IDropTarget;
	pub use super::imoniker::IMoniker;
	pub use super::ipersist::IPersist;
	pub use super::ipersistfile::IPersistFile;
	pub use super::ipersiststream::IPersistStream;
	pub use super::ipicture::IPicture;
	pub use super::isequentialstream::ISequentialStream;
	pub use super::istorage::IStorage;
	pub use super::istream::IStream;
	pub use super::iunknown::IUnknown;
}

pub mod traits {
	pub use super::iadvisesink::ole_IAdviseSink;
	pub use super::ibindctx::ole_IBindCtx;
	pub use super::idataobject::ole_IDataObject;
	pub use super::idroptarget::ole_IDropTarget;
	pub use super::imoniker::ole_IMoniker;
	pub use super::ipersist::ole_IPersist;
	pub use super::ipersistfile::ole_IPersistFile;
	pub use super::ipersiststream::ole_IPersistStream;
	pub use super::ipicture::ole_IPicture;
	pub use super::isequentialstream::ole_ISequentialStream;
	pub use super::istorage::ole_IStorage;
	pub use super::istream::ole_IStream;
	pub use super::iunknown::ole_IUnknown;
}

pub mod vt {
	pub use super::iadvisesink::IAdviseSinkVT;
	pub use super::ibindctx::IBindCtxVT;
	pub use super::idataobject::IDataObjectVT;
	pub use super::idroptarget::IDropTargetVT;
	pub use super::imoniker::IMonikerVT;
	pub use super::ipersist::IPersistVT;
	pub use super::ipersistfile::IPersistFileVT;
	pub use super::ipersiststream::IPersistStreamVT;
	pub use super::ipicture::IPictureVT;
	pub use super::isequentialstream::ISequentialStreamVT;
	pub use super::istorage::IStorageVT;
	pub use super::istream::IStreamVT;
	pub use super::iunknown::IUnknownVT;
}
