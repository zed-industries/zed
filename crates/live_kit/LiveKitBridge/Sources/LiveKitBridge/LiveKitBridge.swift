import Foundation
import LiveKit

@_cdecl("LKRelease")
public func LKRelease(ptr: UnsafeRawPointer)  {
    let _ = Unmanaged<AnyObject>.fromOpaque(ptr).takeRetainedValue();
}

@_cdecl("LKRoomCreate")
public func LKRoomCreate() -> UnsafeMutableRawPointer  {
    Unmanaged.passRetained(Room()).toOpaque()
}

@_cdecl("LKRoomConnect")
public func LKRoomConnect(room: UnsafeRawPointer, url: CFString, token: CFString, callback: @escaping @convention(c) (UnsafeRawPointer, CFString?) -> Void, callback_data: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue();

    room.connect(url as String, token as String).then { _ in
        callback(callback_data, UnsafeRawPointer(nil) as! CFString?);
    }.catch { error in
        callback(callback_data, error.localizedDescription as CFString);
    };
}

@_cdecl("LKRoomPublishVideoTrack")
public func LKRoomPublishVideoTrack(room: UnsafeRawPointer, track: UnsafeRawPointer, callback: @escaping @convention(c) (UnsafeRawPointer, CFString?) -> Void, callback_data: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue();
    let track = Unmanaged<LocalVideoTrack>.fromOpaque(track).takeUnretainedValue();
    room.localParticipant?.publishVideoTrack(track: track).then { _ in
        callback(callback_data, UnsafeRawPointer(nil) as! CFString?);
    }.catch { error in
        callback(callback_data, error.localizedDescription as CFString);
    };
}

@_cdecl("LKCreateScreenShareTrackForWindow")
public func LKCreateScreenShareTrackForWindow(windowId: uint32) -> UnsafeMutableRawPointer {
    let track = LocalVideoTrack.createMacOSScreenShareTrack(source: .window(id: windowId));
    return Unmanaged.passRetained(track).toOpaque()
}
