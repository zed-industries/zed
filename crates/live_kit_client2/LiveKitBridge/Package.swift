// swift-tools-version: 5.5

import PackageDescription

let package = Package(
    name: "LiveKitBridge",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "LiveKitBridge",
            type: .static,
            targets: ["LiveKitBridge"]),
    ],
    dependencies: [
        .package(url: "https://github.com/livekit/client-sdk-swift.git", .exact("1.0.12")),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "LiveKitBridge",
            dependencies: [.product(name: "LiveKit", package: "client-sdk-swift")]),
    ]
)
