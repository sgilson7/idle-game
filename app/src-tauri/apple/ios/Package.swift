// swift-tools-version:5.5
// The App Store, Game Center and iCloud for Idlewright (T17).

import PackageDescription

let package = Package(
  name: "tauri-plugin-apple",
  platforms: [
    .iOS(.v16)
  ],
  products: [
    .library(
      name: "tauri-plugin-apple",
      type: .static,
      targets: ["tauri-plugin-apple"])
  ],
  dependencies: [
    .package(name: "Tauri", path: "../.tauri/tauri-api")
  ],
  targets: [
    .target(
      name: "tauri-plugin-apple",
      dependencies: [
        .byName(name: "Tauri")
      ],
      path: "Sources")
  ]
)
