import Foundation
import LiveKit

@_cdecl("LKRoomCreate")
public func LKRoomCreate() -> UnsafeMutableRawPointer  {
    Unmanaged.passRetained(Room()).toOpaque()
}

@_cdecl("LKRoomDestroy")
public func LKRoomDestroy(ptr: UnsafeRawPointer)  {
    let _ = Unmanaged<Room>.fromOpaque(ptr).takeRetainedValue();
}

@_cdecl("LKRoomConnect")
public func LKRoomConnect(room: UnsafeRawPointer, url: CFString, token: CFString, callback: @escaping @convention(c) (UnsafeRawPointer) -> Void, callback_data: UnsafeRawPointer) {
    let room = Unmanaged<Room>.fromOpaque(room).takeUnretainedValue();

    room.connect(url as String, token as String).then { _ in
        callback(callback_data);
    }.catch { error in
        print(error);
    };
}
