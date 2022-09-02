//
//  LKRoom.swift
//  LiveKitObjC
//
//  Created by Antonio Scandurra on 01/09/22.
//

import Foundation
import LiveKit

public class LKRoom: RoomDelegate {
    lazy var room = Room(delegate: self)
    
    init() {
        print("INIT!\n");
    }
    
    deinit {
        print("DEINIT!\n");
    }
    
    public func connect(
        url: String,
        token: String,
        callback: @convention(block) @escaping () -> Void
    ) {
        self.room.connect(url, token).then { room in
            callback()
        }
    }
    
}


@_cdecl("LKRoomCreate")
public func LKRoomCreate() -> UnsafeMutableRawPointer  {
    Unmanaged.passRetained(LKRoom()).toOpaque()
}

@_cdecl("LKRoomDestroy")
public func LKRoomDestroy(ptr: UnsafeRawPointer)  {
    let _ = Unmanaged<LKRoom>.fromOpaque(ptr).takeRetainedValue();
}


