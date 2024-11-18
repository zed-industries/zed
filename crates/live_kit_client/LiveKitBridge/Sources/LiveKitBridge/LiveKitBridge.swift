import Foundation
import LiveKit
import WebRTC
import ScreenCaptureKit

class LKRoomDelegate: RoomDelegate {
    var data: UnsafeRawPointer
    var onDidDisconnect: @convention(c) (UnsafeRawPointer) -> Void
    var onDidSubscribeToRemoteAudioTrack: @convention(c) (UnsafeRawPointer, CFString, CFString, UnsafeRawPointer, UnsafeRawPointer) -> Void
    var onDidUnsubscribeFromRemoteAudioTrack: @convention(c) (UnsafeRawPointer, CFString, CFString) -> Void
    var onMuteChangedFromRemoteAudioTrack: @convention(c) (UnsafeRawPointer, CFString, Bool) -> Void
    var onActiveSpeakersChanged: @convention(c) (UnsafeRawPointer, CFArray) -> Void
    var onDidSubscribeToRemoteVideoTrack: @convention(c) (UnsafeRawPointer, CFString, CFString, UnsafeRawPointer) -> Void
    var onDidUnsubscribeFromRemoteVideoTrack: @convention(c) (UnsafeRawPointer, CFString, CFString) -> Void
    var onDidPublishOrUnpublishLocalAudioTrack: @convention(c) (UnsafeRawPointer, UnsafeRawPointer, Bool) -> Void
    var onDidPublishOrUnpublishLocalVideoTrack: @convention(c) (UnsafeRawPointer, UnsafeRawPointer, Bool) -> Void

    init(
        data: UnsafeRawPointer,
        onDidDisconnect: @escaping @convention(c) (UnsafeRawPointer) -> Void,
        onDidSubscribeToRemoteAudioTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, CFString, UnsafeRawPointer, UnsafeRawPointer) -> Void,
        onDidUnsubscribeFromRemoteAudioTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, CFString) -> Void,
        onMuteChangedFromRemoteAudioTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, Bool) -> Void,
        onActiveSpeakersChanged: @convention(c) (UnsafeRawPointer, CFArray) -> Void,
        onDidSubscribeToRemoteVideoTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, CFString, UnsafeRawPointer) -> Void,
        onDidUnsubscribeFromRemoteVideoTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, CFString) -> Void,
        onDidPublishOrUnpublishLocalAudioTrack: @escaping @convention(c) (UnsafeRawPointer, UnsafeRawPointer, Bool) -> Void,
        onDidPublishOrUnpublishLocalVideoTrack: @escaping @convention(c) (UnsafeRawPointer, UnsafeRawPointer, Bool) -> Void
    )
    {
        self.data = data
        self.onDidDisconnect = onDidDisconnect
        self.onDidSubscribeToRemoteAudioTrack = onDidSubscribeToRemoteAudioTrack
        self.onDidUnsubscribeFromRemoteAudioTrack = onDidUnsubscribeFromRemoteAudioTrack
        self.onDidSubscribeToRemoteVideoTrack = onDidSubscribeToRemoteVideoTrack
        self.onDidUnsubscribeFromRemoteVideoTrack = onDidUnsubscribeFromRemoteVideoTrack
        self.onMuteChangedFromRemoteAudioTrack = onMuteChangedFromRemoteAudioTrack
        self.onActiveSpeakersChanged = onActiveSpeakersChanged
        self.onDidPublishOrUnpublishLocalAudioTrack = onDidPublishOrUnpublishLocalAudioTrack
        self.onDidPublishOrUnpublishLocalVideoTrack = onDidPublishOrUnpublishLocalVideoTrack
    }

    func room(_ room: Room, didUpdate connectionState: ConnectionState, oldValue: ConnectionState) {
        if connectionState.isDisconnected {
            self.onDidDisconnect(self.data)
        }
    }

    func room(_ room: Room, participant: RemoteParticipant, didSubscribe publication: RemoteTrackPublication, track: Track) {
        if track.kind == .video {
            self.onDidSubscribeToRemoteVideoTrack(self.data, participant.identity as CFString, track.sid! as CFString, Unmanaged.passUnretained(track).toOpaque())
        } else if track.kind == .audio {
            self.onDidSubscribeToRemoteAudioTrack(self.data, participant.identity as CFString, track.sid! as CFString, Unmanaged.passUnretained(track).toOpaque(), Unmanaged.passUnretained(publication).toOpaque())
        }
    }

    func room(_ room: Room, participant: Participant, didUpdate publication: TrackPublication, muted: Bool) {
        if publication.kind == .audio {
            self.onMuteChangedFromRemoteAudioTrack(self.data, publication.sid as CFString, muted)
        }
    }

    func room(_ room: Room, didUpdate speakers: [Participant]) {
        guard let speaker_ids = speakers.compactMap({ $0.identity as CFString }) as CFArray? else { return }
        self.onActiveSpeakersChanged(self.data, speaker_ids)
    }

    func room(_ room: Room, participant: RemoteParticipant, didUnsubscribe publication: RemoteTrackPublication, track: Track) {
        if track.kind == .video {
            self.onDidUnsubscribeFromRemoteVideoTrack(self.data, participant.identity as CFString, track.sid! as CFString)
        } else if track.kind == .audio {
            self.onDidUnsubscribeFromRemoteAudioTrack(self.data, participant.identity as CFString, track.sid! as CFString)
        }
    }

    func room(_ room: Room, localParticipant: LocalParticipant, didPublish publication: LocalTrackPublication) {
        if publication.kind == .video {
            self.onDidPublishOrUnpublishLocalVideoTrack(self.data, Unmanaged.passUnretained(publication).toOpaque(), true)
        } else if publication.kind == .audio {
            self.onDidPublishOrUnpublishLocalAudioTrack(self.data, Unmanaged.passUnretained(publication).toOpaque(), true)
        }
    }

    func room(_ room: Room, localParticipant: LocalParticipant, didUnpublish publication: LocalTrackPublication) {
        if publication.kind == .video {
            self.onDidPublishOrUnpublishLocalVideoTrack(self.data, Unmanaged.passUnretained(publication).toOpaque(), false)
        } else if publication.kind == .audio {
            self.onDidPublishOrUnpublishLocalAudioTrack(self.data, Unmanaged.passUnretained(publication).toOpaque(), false)
        }
    }
}

class LKVideoRenderer: NSObject, VideoRenderer {
    var data: UnsafeRawPointer
    var onFrame: @convention(c) (UnsafeRawPointer, CVPixelBuffer) -> Bool
    var onDrop: @convention(c) (UnsafeRawPointer) -> Void
    var adaptiveStreamIsEnabled: Bool = false
    var adaptiveStreamSize: CGSize = .zero
    weak var track: VideoTrack?

    init(data: UnsafeRawPointer, onFrame: @escaping @convention(c) (UnsafeRawPointer, CVPixelBuffer) -> Bool, onDrop: @escaping @convention(c) (UnsafeRawPointer) -> Void) {
        self.data = data
        self.onFrame = onFrame
        self.onDrop = onDrop
    }

    deinit {
        self.onDrop(self.data)
    }

    func setSize(_ size: CGSize) {
    }

    func renderFrame(_ frame: RTCVideoFrame?) {
        let buffer = frame?.buffer as? RTCCVPixelBuffer
        if let pixelBuffer = buffer?.pixelBuffer {
            if !self.onFrame(self.data, pixelBuffer) {
                DispatchQueue.main.async {
                    self.track?.remove(videoRenderer: self)
                }
            }
        }
    }
}

@_cdecl("LKRoomDelegateCreate")
public func LKRoomDelegateCreate(
    data: UnsafeRawPointer,
    onDidDisconnect: @escaping @convention(c) (UnsafeRawPointer) -> Void,
    onDidSubscribeToRemoteAudioTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, CFString, UnsafeRawPointer, UnsafeRawPointer) -> Void,
    onDidUnsubscribeFromRemoteAudioTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, CFString) -> Void,
    onMuteChangedFromRemoteAudioTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, Bool) -> Void,
    onActiveSpeakerChanged: @escaping @convention(c) (UnsafeRawPointer, CFArray) -> Void,
    onDidSubscribeToRemoteVideoTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, CFString, UnsafeRawPointer) -> Void,
    onDidUnsubscribeFromRemoteVideoTrack: @escaping @convention(c) (UnsafeRawPointer, CFString, CFString) -> Void,
    onDidPublishOrUnpublishLocalAudioTrack: @escaping @convention(c) (UnsafeRawPointer, UnsafeRawPointer, Bool) -> Void,
    onDidPublishOrUnpublishLocalVideoTrack: @escaping @convention(c) (UnsafeRawPointer, UnsafeRawPointer, Bool) -> Void
) -> UnsafeMutableRawPointer {
    let delegate = LKRoomDelegate(
        data: data,
        onDidDisconnect: onDidDisconnect,
        onDidSubscribeToRemoteAudioTrack: onDidSubscribeToRemoteAudioTrack,
        onDidUnsubscribeFromRemoteAudioTrack: onDidUnsubscribeFromRemoteAudioTrack,
        onMuteChangedFromRemoteAudioTrack: onMuteChangedFromRemoteAudioTrack,
        onActiveSpeakersChanged: onActiveSpeakerChanged,
        onDidSubscribeToRemoteVideoTrack: onDidSubscribeToRemoteVideoTrack,
        onDidUnsubscribeFromRemoteVideoTrack: onDidUnsubscribeFromRemoteVideoTrack,
        onDidPublishOrUnpublishLocalAudioTrack: onDidPublishOrUnpublishLocalAudioTrack,
        onDidPublishOrUnpublishLocalVideoTrack: onDidPublishOrUnpublishLocalVideoTrack
    )
    return Unmanaged.passRetained(delegate).toOpaque()
}

@_cdecl("LKRoomCreate")
public func LKRoomCreate(delegate: UnsafeRawPointer) -> UnsafeMutableRawPointer  {
    let delegate = Unmanaged<LKRoomDelegate>.fromOpaque(delegate).takeUnretainedValue()
    return Unmanaged.passRetained(Room(delegate: delegate)).toOpaque()
}

@_cdecl("LKRoomConnect")
public func LKRoomConnect(room: UnsafeRawPointer, url: CFString, token: CFString, callback: @escaping @convention(c) (UnsafeRawPointer, CFString?) -> Void, callback_data: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()

    room.connect(url as String, token as String).then { _ in
        callback(callback_data, UnsafeRawPointer(nil) as! CFString?)
    }.catch { error in
        callback(callback_data, error.localizedDescription as CFString)
    }
}

@_cdecl("LKRoomDisconnect")
public func LKRoomDisconnect(room: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()
    room.disconnect()
}

@_cdecl("LKRoomPublishVideoTrack")
public func LKRoomPublishVideoTrack(room: UnsafeRawPointer, track: UnsafeRawPointer, callback: @escaping @convention(c) (UnsafeRawPointer, UnsafeMutableRawPointer?, CFString?) -> Void, callback_data: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()
    let track = Unmanaged<LocalVideoTrack>.fromOpaque(track).takeUnretainedValue()
    room.localParticipant?.publishVideoTrack(track: track).then { publication in
        callback(callback_data, Unmanaged.passRetained(publication).toOpaque(), nil)
    }.catch { error in
        callback(callback_data, nil, error.localizedDescription as CFString)
    }
}

@_cdecl("LKRoomPublishAudioTrack")
public func LKRoomPublishAudioTrack(room: UnsafeRawPointer, track: UnsafeRawPointer, callback: @escaping @convention(c) (UnsafeRawPointer, UnsafeMutableRawPointer?, CFString?) -> Void, callback_data: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()
    let track = Unmanaged<LocalAudioTrack>.fromOpaque(track).takeUnretainedValue()
    room.localParticipant?.publishAudioTrack(track: track).then { publication in
        callback(callback_data, Unmanaged.passRetained(publication).toOpaque(), nil)
    }.catch { error in
        callback(callback_data, nil, error.localizedDescription as CFString)
    }
}


@_cdecl("LKRoomUnpublishTrack")
public func LKRoomUnpublishTrack(room: UnsafeRawPointer, publication: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()
    let publication = Unmanaged<LocalTrackPublication>.fromOpaque(publication).takeUnretainedValue()
    let _ = room.localParticipant?.unpublish(publication: publication)
}

@_cdecl("LKRoomAudioTracksForRemoteParticipant")
public func LKRoomAudioTracksForRemoteParticipant(room: UnsafeRawPointer, participantId: CFString) -> CFArray? {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()

    for (_, participant) in room.remoteParticipants {
        if participant.identity == participantId as String {
            return participant.audioTracks.compactMap { $0.track as? RemoteAudioTrack } as CFArray?
        }
    }

    return nil;
}

@_cdecl("LKRoomAudioTrackPublicationsForRemoteParticipant")
public func LKRoomAudioTrackPublicationsForRemoteParticipant(room: UnsafeRawPointer, participantId: CFString) -> CFArray? {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()

    for (_, participant) in room.remoteParticipants {
        if participant.identity == participantId as String {
            return participant.audioTracks.compactMap { $0 as? RemoteTrackPublication } as CFArray?
        }
    }

    return nil;
}

@_cdecl("LKRoomVideoTracksForRemoteParticipant")
public func LKRoomVideoTracksForRemoteParticipant(room: UnsafeRawPointer, participantId: CFString) -> CFArray? {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()

    for (_, participant) in room.remoteParticipants {
        if participant.identity == participantId as String {
            return participant.videoTracks.compactMap { $0.track as? RemoteVideoTrack } as CFArray?
        }
    }

    return nil;
}

@_cdecl("LKLocalAudioTrackCreateTrack")
public func LKLocalAudioTrackCreateTrack() -> UnsafeMutableRawPointer {
    let track = LocalAudioTrack.createTrack(options: AudioCaptureOptions(
      echoCancellation: true,
      noiseSuppression: true
    ))

    return Unmanaged.passRetained(track).toOpaque()
}


@_cdecl("LKCreateScreenShareTrackForDisplay")
public func LKCreateScreenShareTrackForDisplay(display: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    let display = Unmanaged<MacOSDisplay>.fromOpaque(display).takeUnretainedValue()
    let track = LocalVideoTrack.createMacOSScreenShareTrack(source: display, preferredMethod: .legacy)
    return Unmanaged.passRetained(track).toOpaque()
}

@_cdecl("LKVideoRendererCreate")
public func LKVideoRendererCreate(data: UnsafeRawPointer, onFrame: @escaping @convention(c) (UnsafeRawPointer, CVPixelBuffer) -> Bool, onDrop: @escaping @convention(c) (UnsafeRawPointer) -> Void) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(LKVideoRenderer(data: data, onFrame: onFrame, onDrop: onDrop)).toOpaque()
}

@_cdecl("LKVideoTrackAddRenderer")
public func LKVideoTrackAddRenderer(track: UnsafeRawPointer, renderer: UnsafeRawPointer) {
    let track = Unmanaged<Track>.fromOpaque(track).takeUnretainedValue() as! VideoTrack
    let renderer = Unmanaged<LKVideoRenderer>.fromOpaque(renderer).takeRetainedValue()
    renderer.track = track
    track.add(videoRenderer: renderer)
}

@_cdecl("LKRemoteVideoTrackGetSid")
public func LKRemoteVideoTrackGetSid(track: UnsafeRawPointer) -> CFString {
    let track = Unmanaged<RemoteVideoTrack>.fromOpaque(track).takeUnretainedValue()
    return track.sid! as CFString
}

@_cdecl("LKRemoteAudioTrackGetSid")
public func LKRemoteAudioTrackGetSid(track: UnsafeRawPointer) -> CFString {
    let track = Unmanaged<RemoteAudioTrack>.fromOpaque(track).takeUnretainedValue()
    return track.sid! as CFString
}

@_cdecl("LKRemoteAudioTrackStart")
public func LKRemoteAudioTrackStart(track: UnsafeRawPointer) {
    let track = Unmanaged<RemoteAudioTrack>.fromOpaque(track).takeUnretainedValue()
    track.start()
}

@_cdecl("LKRemoteAudioTrackStop")
public func LKRemoteAudioTrackStop(track: UnsafeRawPointer) {
    let track = Unmanaged<RemoteAudioTrack>.fromOpaque(track).takeUnretainedValue()
    track.stop()
}

@_cdecl("LKDisplaySources")
public func LKDisplaySources(data: UnsafeRawPointer, callback: @escaping @convention(c) (UnsafeRawPointer, CFArray?, CFString?) -> Void) {
    MacOSScreenCapturer.sources(for: .display, includeCurrentApplication: false, preferredMethod: .legacy).then { displaySources in
        callback(data, displaySources as CFArray, nil)
    }.catch { error in
        callback(data, nil, error.localizedDescription as CFString)
    }
}

@_cdecl("LKLocalTrackPublicationSetMute")
public func LKLocalTrackPublicationSetMute(
    publication: UnsafeRawPointer,
    muted: Bool,
    on_complete: @escaping @convention(c) (UnsafeRawPointer, CFString?) -> Void,
    callback_data: UnsafeRawPointer
) {
    let publication = Unmanaged<LocalTrackPublication>.fromOpaque(publication).takeUnretainedValue()

    if muted {
        publication.mute().then {
            on_complete(callback_data, nil)
        }.catch { error in
            on_complete(callback_data, error.localizedDescription as CFString)
        }
    } else {
        publication.unmute().then {
            on_complete(callback_data, nil)
        }.catch { error in
            on_complete(callback_data, error.localizedDescription as CFString)
        }
    }
}

@_cdecl("LKLocalTrackPublicationIsMuted")
public func LKLocalTrackPublicationIsMuted(
    publication: UnsafeRawPointer
) -> Bool {
    let publication = Unmanaged<LocalTrackPublication>.fromOpaque(publication).takeUnretainedValue()
    return publication.muted
}

@_cdecl("LKRemoteTrackPublicationSetEnabled")
public func LKRemoteTrackPublicationSetEnabled(
    publication: UnsafeRawPointer,
    enabled: Bool,
    on_complete: @escaping @convention(c) (UnsafeRawPointer, CFString?) -> Void,
    callback_data: UnsafeRawPointer
) {
    let publication = Unmanaged<RemoteTrackPublication>.fromOpaque(publication).takeUnretainedValue()

    publication.set(enabled: enabled).then {
        on_complete(callback_data, nil)
    }.catch { error in
        on_complete(callback_data, error.localizedDescription as CFString)
    }
}

@_cdecl("LKRemoteTrackPublicationIsMuted")
public func LKRemoteTrackPublicationIsMuted(
    publication: UnsafeRawPointer
) -> Bool {
    let publication = Unmanaged<RemoteTrackPublication>.fromOpaque(publication).takeUnretainedValue()

    return publication.muted
}

@_cdecl("LKRemoteTrackPublicationGetSid")
public func LKRemoteTrackPublicationGetSid(
    publication: UnsafeRawPointer
) -> CFString {
    let publication = Unmanaged<RemoteTrackPublication>.fromOpaque(publication).takeUnretainedValue()

    return publication.sid as CFString
}

@_cdecl("LKLocalTrackPublicationGetSid")
public func LKLocalTrackPublicationGetSid(
    publication: UnsafeRawPointer
) -> CFString {
    let publication = Unmanaged<LocalTrackPublication>.fromOpaque(publication).takeUnretainedValue()

    return publication.sid as CFString
}
