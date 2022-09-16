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
        .package(url: "https://github.com/livekit/client-sdk-swift.git", revision: "5cc3c001779ab147199ce3ea0dce465b846368b4"),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "LiveKitBridge",
            dependencies: [.product(name: "LiveKit", package: "client-sdk-swift")]),
    ]
)
