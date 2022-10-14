import Foundation
import LiveKit
import WebRTC

class LKRoomDelegate: RoomDelegate {
    var data: UnsafeRawPointer
    var onDidSubscribeToRemoteVideoTrack: @convention(c) (UnsafeRawPointer, UnsafeRawPointer) -> Void
    
    init(data: UnsafeRawPointer, onDidSubscribeToRemoteVideoTrack: @escaping @convention(c) (UnsafeRawPointer, UnsafeRawPointer) -> Void) {
        self.data = data
        self.onDidSubscribeToRemoteVideoTrack = onDidSubscribeToRemoteVideoTrack
    }

    func room(_ room: Room, participant: RemoteParticipant, didSubscribe publication: RemoteTrackPublication, track: Track) {
        if track.kind == .video {
            self.onDidSubscribeToRemoteVideoTrack(self.data, Unmanaged.passRetained(track).toOpaque())
        }
    }
}

class LKVideoRenderer: NSObject, VideoRenderer {
    var data: UnsafeRawPointer
    var onFrame: @convention(c) (UnsafeRawPointer, CVPixelBuffer) -> Void
    var onDrop: @convention(c) (UnsafeRawPointer) -> Void
    var adaptiveStreamIsEnabled: Bool = false
    var adaptiveStreamSize: CGSize = .zero

    init(data: UnsafeRawPointer, onFrame: @escaping @convention(c) (UnsafeRawPointer, CVPixelBuffer) -> Void, onDrop: @escaping @convention(c) (UnsafeRawPointer) -> Void) {
        self.data = data
        self.onFrame = onFrame
        self.onDrop = onDrop
    }

    deinit {
        self.onDrop(self.data)
    }

    func setSize(_ size: CGSize) {
        print("Called setSize", size);
    }

    func renderFrame(_ frame: RTCVideoFrame?) {
        let buffer = frame?.buffer as? RTCCVPixelBuffer
        if let pixelBuffer = buffer?.pixelBuffer {
            self.onFrame(self.data, pixelBuffer)
        }
    }
}

@_cdecl("LKRelease")
public func LKRelease(ptr: UnsafeRawPointer)  {
    let _ = Unmanaged<AnyObject>.fromOpaque(ptr).takeRetainedValue()
}

@_cdecl("LKRoomDelegateCreate")
public func LKRoomDelegateCreate(data: UnsafeRawPointer, onDidSubscribeToRemoteVideoTrack: @escaping @convention(c) (UnsafeRawPointer, UnsafeRawPointer) -> Void) -> UnsafeMutableRawPointer {
    let delegate = LKRoomDelegate(data: data, onDidSubscribeToRemoteVideoTrack: onDidSubscribeToRemoteVideoTrack)
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

@_cdecl("LKRoomPublishVideoTrack")
public func LKRoomPublishVideoTrack(room: UnsafeRawPointer, track: UnsafeRawPointer, callback: @escaping @convention(c) (UnsafeRawPointer, CFString?) -> Void, callback_data: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue()
    let track = Unmanaged<LocalVideoTrack>.fromOpaque(track).takeUnretainedValue()
    room.localParticipant?.publishVideoTrack(track: track).then { _ in
        callback(callback_data, UnsafeRawPointer(nil) as! CFString?)
    }.catch { error in
        callback(callback_data, error.localizedDescription as CFString)
    }
}

@_cdecl("LKCreateScreenShareTrackForDisplay")
public func LKCreateScreenShareTrackForDisplay(display: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    let display = Unmanaged<MacOSDisplay>.fromOpaque(display).takeRetainedValue()
    let track = LocalVideoTrack.createMacOSScreenShareTrack(source: display, preferredMethod: .legacy)
    return Unmanaged.passRetained(track).toOpaque()
}

@_cdecl("LKVideoRendererCreate")
public func LKVideoRendererCreate(data: UnsafeRawPointer, onFrame: @escaping @convention(c) (UnsafeRawPointer, CVPixelBuffer) -> Void, onDrop: @escaping @convention(c) (UnsafeRawPointer) -> Void) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(LKVideoRenderer(data: data, onFrame: onFrame, onDrop: onDrop)).toOpaque()
}

@_cdecl("LKVideoTrackAddRenderer")
public func LKVideoTrackAddRenderer(track: UnsafeRawPointer, renderer: UnsafeRawPointer) {
    let track = Unmanaged<Track>.fromOpaque(track).takeUnretainedValue() as! VideoTrack
    let renderer = Unmanaged<LKVideoRenderer>.fromOpaque(renderer).takeRetainedValue()
    track.add(videoRenderer: renderer)
}

@_cdecl("LKDisplaySources")
public func LKDisplaySources(data: UnsafeRawPointer, callback: @escaping @convention(c) (UnsafeRawPointer, CFArray?, CFString?) -> Void) {
    MacOSScreenCapturer.displaySources().then { displaySources in
        callback(data, displaySources as CFArray, nil)
    }.catch { error in
        callback(data, nil, error.localizedDescription as CFString)
    }
}
