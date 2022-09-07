import Foundation
import LiveKit

class LKRoomDelegate: RoomDelegate {
    var data: UnsafeRawPointer
    var onDidSubscribeToRemoteTrack: @convention(c) (UnsafeRawPointer, UnsafeRawPointer) -> Void
    
    init(data: UnsafeRawPointer, onDidSubscribeToRemoteTrack: @escaping @convention(c) (UnsafeRawPointer, UnsafeRawPointer) -> Void) {
        self.data = data
        self.onDidSubscribeToRemoteTrack = onDidSubscribeToRemoteTrack
    }
    
    func room(_ room: Room, participant: RemoteParticipant, didSubscribe publication: RemoteTrackPublication, track: Track) {
        self.onDidSubscribeToRemoteTrack(self.data, Unmanaged.passRetained(track).toOpaque())
    }
}

@_cdecl("LKRelease")
public func LKRelease(ptr: UnsafeRawPointer)  {
    let _ = Unmanaged<AnyObject>.fromOpaque(ptr).takeRetainedValue()
}

@_cdecl("LKRoomDelegateCreate")
public func LKRoomDelegateCreate(data: UnsafeRawPointer, onDidSubscribeToRemoteTrack: @escaping @convention(c) (UnsafeRawPointer, UnsafeRawPointer) -> Void) -> UnsafeMutableRawPointer {
    let delegate = LKRoomDelegate(data: data, onDidSubscribeToRemoteTrack: onDidSubscribeToRemoteTrack)
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

@_cdecl("LKCreateScreenShareTrackForWindow")
public func LKCreateScreenShareTrackForWindow(windowId: uint32) -> UnsafeMutableRawPointer {
    let track = LocalVideoTrack.createMacOSScreenShareTrack(source: .window(id: windowId))
    return Unmanaged.passRetained(track).toOpaque()
}
