// Code generated; DO NOT EDIT.
package roomobs

type ConnectionResult string

const (
	ConnectionResultUndefined ConnectionResult = ""
	ConnectionResultSuccess   ConnectionResult = "success"
	ConnectionResultFailure   ConnectionResult = "failure"
)

type ConnectionType string

const (
	ConnectionTypeUndefined ConnectionType = ""
	ConnectionTypeUDP       ConnectionType = "udp"
	ConnectionTypeTCP       ConnectionType = "tcp"
	ConnectionTypeTurn      ConnectionType = "turn"
)

type ClientOS string

const (
	ClientOSUndefined ClientOS = ""
	ClientOSIos       ClientOS = "ios"
	ClientOSAndroid   ClientOS = "android"
	ClientOSWindows   ClientOS = "windows"
	ClientOSMac       ClientOS = "mac"
	ClientOSLinux     ClientOS = "linux"
)

type TrackKind string

const (
	TrackKindUndefined TrackKind = ""
	TrackKindPub       TrackKind = "pub"
	TrackKindSub       TrackKind = "sub"
)

type TrackType string

const (
	TrackTypeUndefined TrackType = ""
	TrackTypeAudio     TrackType = "audio"
	TrackTypeVideo     TrackType = "video"
	TrackTypeData      TrackType = "data"
)

type TrackSource string

const (
	TrackSourceUndefined        TrackSource = ""
	TrackSourceCamera           TrackSource = "camera"
	TrackSourceMicrophone       TrackSource = "microphone"
	TrackSourceScreenShare      TrackSource = "screen_share"
	TrackSourceScreenShareAudio TrackSource = "screen_share_audio"
)

type MimeType string

const (
	MimeTypeUndefined      MimeType = ""
	MimeTypeVideoH264      MimeType = "video/H264"
	MimeTypeVideoH265      MimeType = "video/H265"
	MimeTypeAudioOpus      MimeType = "audio/opus"
	MimeTypeAudioRed       MimeType = "audio/red"
	MimeTypeVideoVp8       MimeType = "video/VP8"
	MimeTypeVideoVp9       MimeType = "video/VP9"
	MimeTypeVideoAv1       MimeType = "video/AV1"
	MimeTypeAudioG722      MimeType = "audio/G722"
	MimeTypeAudioPcmu      MimeType = "audio/PCMU"
	MimeTypeAudioPcma      MimeType = "audio/PCMA"
	MimeTypeVideoRtx       MimeType = "video/rtx"
	MimeTypeVideoFlexfec   MimeType = "video/flexfec"
	MimeTypeVideoFlexfec03 MimeType = "video/flexfec-03"
	MimeTypeVideoUlpfec    MimeType = "video/ulpfec"
)

type Rollup string

const (
	RollupUndefined               Rollup = ""
	RollupProject                 Rollup = "project"
	RollupRoomSessionIndex        Rollup = "room_session_index"
	RollupParticipantIndex        Rollup = "participant_index"
	RollupParticipantSessionIndex Rollup = "participant_session_index"
	RollupParticipantSession      Rollup = "participant_session"
	RollupTrackIndex              Rollup = "track_index"
	RollupTrack                   Rollup = "track"
	RollupStartTimeIndex          Rollup = "start_time_index"
	RollupEndTimeIndex            Rollup = "end_time_index"
	RollupSessionIDIndex          Rollup = "session_id_index"
	RollupRoomSession             Rollup = "room_session"
)
