mod imfasynccallback;
mod imfasyncresult;
mod imfattributes;
mod imfclock;
mod imfgetservice;
mod imfmediaevent;
mod imfmediaeventgenerator;
mod imfmediasession;
mod imfmediasource;
mod imfpresentationdescriptor;
mod imfsourceresolver;
mod imftopology;
mod imftopologynode;
mod imfvideodisplaycontrol;

pub mod decl {
	pub use super::imfasynccallback::IMFAsyncCallback;
	pub use super::imfasyncresult::IMFAsyncResult;
	pub use super::imfattributes::IMFAttributes;
	pub use super::imfclock::IMFClock;
	pub use super::imfgetservice::IMFGetService;
	pub use super::imfmediaevent::IMFMediaEvent;
	pub use super::imfmediaeventgenerator::IMFMediaEventGenerator;
	pub use super::imfmediasession::IMFMediaSession;
	pub use super::imfmediasource::IMFMediaSource;
	pub use super::imfpresentationdescriptor::IMFPresentationDescriptor;
	pub use super::imfsourceresolver::IMFSourceResolver;
	pub use super::imftopology::IMFTopology;
	pub use super::imftopologynode::IMFTopologyNode;
	pub use super::imfvideodisplaycontrol::IMFVideoDisplayControl;
}

pub mod traits {
	pub use super::imfasynccallback::mf_IMFAsyncCallback;
	pub use super::imfasyncresult::mf_IMFAsyncResult;
	pub use super::imfattributes::mf_IMFAttributes;
	pub use super::imfclock::mf_IMFClock;
	pub use super::imfgetservice::mf_IMFGetService;
	pub use super::imfmediaevent::mf_IMFMediaEvent;
	pub use super::imfmediaeventgenerator::mf_IMFMediaEventGenerator;
	pub use super::imfmediasession::mf_IMFMediaSession;
	pub use super::imfmediasource::mf_IMFMediaSource;
	pub use super::imfpresentationdescriptor::mf_IMFPresentationDescriptor;
	pub use super::imfsourceresolver::mf_IMFSourceResolver;
	pub use super::imftopology::mf_IMFTopology;
	pub use super::imftopologynode::mf_IMFTopologyNode;
	pub use super::imfvideodisplaycontrol::mf_IMFVideoDisplayControl;
}

pub mod vt {
	pub use super::imfasynccallback::IMFAsyncCallbackVT;
	pub use super::imfasyncresult::IMFAsyncResultVT;
	pub use super::imfattributes::IMFAttributesVT;
	pub use super::imfclock::IMFClockVT;
	pub use super::imfgetservice::IMFGetServiceVT;
	pub use super::imfmediaevent::IMFMediaEventVT;
	pub use super::imfmediaeventgenerator::IMFMediaEventGeneratorVT;
	pub use super::imfmediasession::IMFMediaSessionVT;
	pub use super::imfmediasource::IMFMediaSourceVT;
	pub use super::imfpresentationdescriptor::IMFPresentationDescriptorVT;
	pub use super::imfsourceresolver::IMFSourceResolverVT;
	pub use super::imftopology::IMFTopologyVT;
	pub use super::imftopologynode::IMFTopologyNodeVT;
	pub use super::imfvideodisplaycontrol::IMFVideoDisplayControlVT;
}
