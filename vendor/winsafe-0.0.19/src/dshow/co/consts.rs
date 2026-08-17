#![allow(non_camel_case_types, non_upper_case_globals)]

const_ordinary! { FILTER_STATE: u32;
	/// [`FILTER_STATE`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/ne-strmif-filter_state)
	/// enumeration (`u32`).
	=>
	=>
	Stopped 0
	Paused 1
	Running 2
}

const_ordinary! { PIN_DIRECTION: u32;
	/// [`PIN_DIRECTION`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/ne-strmif-pin_direction)
	/// enumeration (`u32`).
	=>
	=>
	INPUT 0
	OUTPUT 1
}

const_ordinary! { SEEKING_FLAGS: u32;
	/// [`IMediaSeeking::SetPositions`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setpositions)
	/// flags (`u32`).
	///
	/// Originally `AM_SEEKING_SeekingFlags` enum.
	=>
	=>
	NoPositioning 0x0
	AbsolutePositioning 0x1
	RelativePositioning 0x2
	IncrementalPositioning 0x3
	SeekToKeyFrame 0x4
	ReturnTime 0x8
	Segment 0x10
	NoFlush 0x20
}
