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
