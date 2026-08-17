#![allow(non_camel_case_types)]

const_guid! { MF_SERVICE;
	/// [`IMFGetService::GetService`](`crate::prelude::mf_IMFGetService::GetService`)
	/// `service_guid` (`GUID`).
	///
	/// Originally has `MF`, `MFNET` and `MR` prefixes.
	=>
	MR_VIDEO_RENDER_SERVICE "1092a86c-ab1a-459a-a336-831fbc4d11ff"
	MR_VIDEO_MIXER_SERVICE "073cd2fc-6cf4-40b7-8859-e89552c841f8"
	MR_VIDEO_ACCELERATION_SERVICE "efef5175-5c7d-4ce2-bbbd-34ff8bca6554"
	MR_BUFFER_SERVICE "a562248c-9ac6-4ffc-9fba-3af8f8ad1a4d"
}

const_guid! { MF_TIME_FORMAT;
	/// [`IMFMediaSession::Start`](crate::prelude::mf_IMFMediaSession::Start)
	/// `time_format` (`GUID`).
	=>
	NULL "00000000-0000-0000-0000-000000000000"
	SEGMENT_OFFSET "c8b8be77-869c-431d-812e-169693f65a39"
	ENTRY_RELATIVE "4399f178-46d3-4504-afda-20d32e9ba360"
}
