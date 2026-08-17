#![allow(non_camel_case_types, non_upper_case_globals)]

const_ordinary! { ME: u32;
	/// [`IMFMediaEvent::GetType`](crate::prelude::mf_IMFMediaEvent::GetType)
	/// return value ([`u32`]).
	=>
	=>
	Unknown 0
	Error 1
	ExtendedType 2
	NonFatalError 3
	GenericV1Anchor Self::NonFatalError.0
	SessionUnknown 100
	SessionTopologySet 101
	SessionTopologiesCleared 102
	SessionStarted 103
	SessionPaused 104
	SessionStopped 105
	SessionClosed 106
	SessionEnded 107
	SessionRateChanged 108
	SessionScrubSampleComplete 109
	SessionCapabilitiesChanged 110
	SessionTopologyStatus 111
	SessionNotifyPresentationTime 112
	NewPresentation 113
	LicenseAcquisitionStart 114
	LicenseAcquisitionCompleted 115
	IndividualizationStart 116
	IndividualizationCompleted 117
	EnablerProgress 118
	EnablerCompleted 119
	PolicyError 120
	PolicyReport 121
	BufferingStarted 122
	BufferingStopped 123
	ConnectStart 124
	ConnectEnd 125
	ReconnectStart 126
	ReconnectEnd 127
	RendererEvent 128
	SessionStreamSinkFormatChanged 129
	SessionV1Anchor Self::SessionStreamSinkFormatChanged.0
	SourceUnknown 200
	SourceStarted 201
	StreamStarted 202
	SourceSeeked 203
	StreamSeeked 204
	NewStream 205
	UpdatedStream 206
	SourceStopped 207
	StreamStopped 208
	SourcePaused 209
	StreamPaused 210
	EndOfPresentation 211
	EndOfStream 212
	MediaSample 213
	StreamTick 214
	StreamThinMode 215
	StreamFormatChanged 216
	SourceRateChanged 217
	EndOfPresentationSegment 218
	SourceCharacteristicsChanged 219
	SourceRateChangeRequested 220
	SourceMetadataChanged 221
	SequencerSourceTopologyUpdated 222
	SourceV1Anchor Self::SequencerSourceTopologyUpdated.0
	SinkUnknown 300
	StreamSinkStarted 301
	StreamSinkStopped 302
	StreamSinkPaused 303
	StreamSinkRateChanged 304
	StreamSinkRequestSample 305
	StreamSinkMarker 306
	StreamSinkPrerolled 307
	StreamSinkScrubSampleComplete 308
	StreamSinkFormatChanged 309
	StreamSinkDeviceChanged 310
	QualityNotify 311
	SinkInvalidated 312
	AudioSessionNameChanged 313
	AudioSessionVolumeChanged 314
	AudioSessionDeviceRemoved 315
	AudioSessionServerShutdown 316
	AudioSessionGroupingParamChanged 317
	AudioSessionIconChanged 318
	AudioSessionFormatChanged 319
	AudioSessionDisconnected 320
	AudioSessionExclusiveModeOverride 321
	SinkV1Anchor Self::AudioSessionExclusiveModeOverride.0
	CaptureAudioSessionVolumeChanged 322
	CaptureAudioSessionDeviceRemoved 323
	CaptureAudioSessionFormatChanged 324
	CaptureAudioSessionDisconnected 325
	CaptureAudioSessionExclusiveModeOverride 326
	CaptureAudioSessionServerShutdown 327
	SinkV2Anchor Self::CaptureAudioSessionServerShutdown.0
	TrustUnknown 400
	PolicyChanged 401
	ContentProtectionMessage 402
	PolicySet 403
	TrustV1Anchor Self::PolicySet.0
	WMDRMLicenseBackupCompleted 500
	WMDRMLicenseBackupProgress 501
	WMDRMLicenseRestoreCompleted 502
	WMDRMLicenseRestoreProgress 503
	WMDRMLicenseAcquisitionCompleted 506
	WMDRMIndividualizationCompleted 508
	WMDRMIndividualizationProgress 513
	WMDRMProximityCompleted 514
	WMDRMLicenseStoreCleaned 515
	WMDRMRevocationDownloadCompleted 516
	WMDRMV1Anchor Self::WMDRMRevocationDownloadCompleted.0
	TransformUnknown 600
	TransformNeedInput Self::TransformUnknown.0 + 1
	TransformHaveOutput Self::TransformNeedInput.0 + 1
	TransformDrainComplete Self::TransformHaveOutput.0 + 1
	TransformMarker Self::TransformDrainComplete.0 + 1
	TransformInputStreamStateChanged Self::TransformMarker.0 + 1
	ByteStreamCharacteristicsChanged 700
	VideoCaptureDeviceRemoved 800
	VideoCaptureDevicePreempted 801
	StreamSinkFormatInvalidated 802
	EncodingParameters 803
	ContentProtectionMetadata 900
	DeviceThermalStateChanged 950
}

const_ordinary! { MF_EVENT_FLAG: u32;
	/// [`IMFMediaEvent::GetType`](crate::prelude::mf_IMFMediaEvent::GetType)
	/// return type ([`u32`]).
	=>
	=>
	NO_WAIT 0x0000_0001
}

const_ordinary! { MF_TOPOLOGY: u32;
	/// [`MF_TOPOLOGY_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mf_topology_type)
	/// enumeration ([`u32`]).
	=>
	=>
	OUTPUT_NODE 0
	SOURCESTREAM_NODE 1
	TRANSFORM_NODE 2
	TEE_NODE 3
}

const_bitflag! { MFASYNC: u32;
	/// [`IMFAsyncCallback::GetParameters`](crate::prelude::mf_IMFAsyncCallback::GetParameters)
	/// `flags` ([`u32`]).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	FAST_IO_PROCESSING_CALLBACK 0x0000_0001
	SIGNAL_CALLBACK 0x0000_0002
	BLOCKING_CALLBACK 0x0000_0004
	REPLY_CALLBACK 0x0000_0008
}

const_bitflag! { MFCLOCK_CHARACTERISTICS_FLAG: u32;
	/// [`MFCLOCK_CHARACTERISTICS_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfclock_characteristics_flags)
	/// enumeration ([`u32`]).
	=>
	=>
	FREQUENCY_10MHZ 0x2
	ALWAYS_RUNNING 0x4
	IS_SYSTEM_CLOCK 0x8
}

const_bitflag! { MFCLOCK_RELATIONAL_FLAG: u32;
	/// [`MFCLOCK_RELATIONAL_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfclock_relational_flags)
	/// enumeration ([`u32`]).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	JITTER_NEVER_AHEAD 0x1
}

const_ordinary! { MFCLOCK_STATE: u32;
	/// [`MFCLOCK_STATE`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfclock_state)
	/// enumeration ([`u32`]).
	=>
	=>
	INVALID 0
	RUNNING 1
	STOPPED 2
	PAUSED 3
}

const_bitflag! { MFMEDIASOURCE: u32;
	/// [`MFMEDIASOURCE_CHARACTERISTICS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfmediasource_characteristics)
	/// enumeration ([`u32`]).
	=>
	=>
	IS_LIVE 0x1
	CAN_SEEK 0x2
	CAN_PAUSE 0x4
	HAS_SLOW_SEEK 0x8
	HAS_MULTIPLE_PRESENTATIONS 0x10
	CAN_SKIPFORWARD 0x20
	CAN_SKIPBACKWARD 0x40
	DOES_NOT_USE_NETWORK 0x80
}

const_bitflag! { MFSESSION_GETFULLTOPOLOGY: u32;
	/// [`MFSESSION_GETFULLTOPOLOGY_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfsession_getfulltopology_flags)
	/// enumeration ([`u32`]).
	=>
	=>
	CURRENT 0x1
}

const_bitflag! { MFSESSION_SETTOPOLOGY: u32;
	/// [`MFSESSION_SETTOPOLOGY_FLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/ne-mfidl-mfsession_settopology_flags)
	/// enumeration ([`u32`]).
	=>
	=>
	IMMEDIATE 0x1
	NORESOLUTION 0x2
	CLEAR_CURRENT 0x4
}

const_bitflag! { MFSESSIONCAP: u32;
	/// [`IMFMediaSession::GetSessionCapabilities`](crate::prelude::mf_IMFMediaSession::GetSessionCapabilities)
	/// `caps` ([`u32`]).
	=>
	=>
	START 0x0000_0001
	SEEK 0x0000_0002
	PAUSE 0x0000_0004
	RATE_FORWARD 0x0000_0010
	RATE_REVERSE 0x0000_0020
	DOES_NOT_USE_NETWORK 0x0000_0040
}

const_ordinary! { MFSTARTUP: u32;
	/// [`MFStartup`](crate::MFStartup) `flags` ([`u32`]).
	=>
	=>
	NOSOCKET 0x1
	LITE Self::NOSOCKET.0
	FULL 0
}

const_ordinary! { MFVideoARMode: u32;
	/// [`MFVideoAspectRatioMode`](https://learn.microsoft.com/en-us/windows/win32/api/evr/ne-evr-mfvideoaspectratiomode)
	/// enumeration ([`u32`]).
	=>
	=>
	None 0
	PreservePicture 0x1
	PreservePixel 0x2
	NonLinearStretch 0x4
}

const_bitflag! { MFVideoRenderPrefs: u32;
	/// [`MFVideoRenderPrefs`](https://learn.microsoft.com/en-us/windows/win32/api/evr/ne-evr-mfvideorenderprefs)
	/// enumeration ([`u32`]).
	=>
	=>
	DoNotRenderBorder 0x1
	DoNotClipToDevice 0x2
	AllowOutputThrottling 0x4
	ForceOutputThrottling 0x8
	ForceBatching 0x10
	AllowBatching 0x20
	ForceScaling 0x40
	AllowScaling 0x80
	DoNotRepaintOnStop 0x100
}
