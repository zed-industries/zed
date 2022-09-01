//
//  LKRoom.swift
//  LiveKitObjC
//
//  Created by Antonio Scandurra on 01/09/22.
//

import Foundation
import LiveKit

@objc public class SLKRoom: NSObject, RoomDelegate {
    lazy var room = Room(delegate: self)
    
    @objc public func connect(
        url: String,
        token: String,
        callback: @convention(block) @escaping () -> Void
    ) {
        self.room.connect(url, token).then { room in
            callback()
        }
    }
}
