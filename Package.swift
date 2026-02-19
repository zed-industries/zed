// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "ZedCapture",
    platforms: [.macOS(.v14)],
    dependencies: [],
    targets: [
        .executableTarget(
            name: "ZedCapture",
            dependencies: [],
            path: "."
        )
    ]
)
