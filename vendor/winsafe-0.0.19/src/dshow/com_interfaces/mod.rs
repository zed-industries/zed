mod ibasefilter;
mod ienumfilters;
mod ienummediatypes;
mod ienumpins;
mod ifilesinkfilter;
mod ifiltergraph;
mod ifiltergraph2;
mod igraphbuilder;
mod imediacontrol;
mod imediafilter;
mod imediaseeking;
mod ipin;

pub mod decl {
	pub use super::ibasefilter::IBaseFilter;
	pub use super::ienumfilters::IEnumFilters;
	pub use super::ienummediatypes::IEnumMediaTypes;
	pub use super::ienumpins::IEnumPins;
	pub use super::ifilesinkfilter::IFileSinkFilter;
	pub use super::ifiltergraph::IFilterGraph;
	pub use super::ifiltergraph2::IFilterGraph2;
	pub use super::igraphbuilder::IGraphBuilder;
	pub use super::imediacontrol::IMediaControl;
	pub use super::imediafilter::IMediaFilter;
	pub use super::imediaseeking::IMediaSeeking;
	pub use super::ipin::IPin;
}

pub mod traits {
	pub use super::ibasefilter::dshow_IBaseFilter;
	pub use super::ienumfilters::dshow_IEnumFilters;
	pub use super::ienummediatypes::dshow_IEnumMediaTypes;
	pub use super::ienumpins::dshow_IEnumPins;
	pub use super::ifilesinkfilter::dshow_IFileSinkFilter;
	pub use super::ifiltergraph::dshow_IFilterGraph;
	pub use super::ifiltergraph2::dshow_IFilterGraph2;
	pub use super::igraphbuilder::dshow_IGraphBuilder;
	pub use super::imediacontrol::dshow_IMediaControl;
	pub use super::imediafilter::dshow_IMediaFilter;
	pub use super::imediaseeking::dshow_IMediaSeeking;
	pub use super::ipin::dshow_IPin;
}

pub mod vt {
	pub use super::ibasefilter::IBaseFilterVT;
	pub use super::ienumfilters::IEnumFiltersVT;
	pub use super::ienummediatypes::IEnumMediaTypesVT;
	pub use super::ienumpins::IEnumPinsVT;
	pub use super::ifilesinkfilter::IFileSinkFilterVT;
	pub use super::ifiltergraph::IFilterGraphVT;
	pub use super::ifiltergraph2::IFilterGraph2VT;
	pub use super::igraphbuilder::IGraphBuilderVT;
	pub use super::imediacontrol::IMediaControlVT;
	pub use super::imediafilter::IMediaFilterVT;
	pub use super::imediaseeking::IMediaSeekingVT;
	pub use super::ipin::IPinVT;
}
