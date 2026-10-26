// The App Store, Game Center and iCloud for Idlewright (T17, IG-Q46 B, IG-Q50 B).
//
// Each method answers one call from the page, through the Rust bridge in
// ../src/lib.rs. The game's rules stay in the core: this file reports what
// Apple's services say (a price, whether the purchase is owned) and does what
// the core asks (show the rating prompt, report a score, keep a save in iCloud).

import GameKit
import StoreKit
import SwiftRs
import Tauri
import UIKit
import WebKit

class ProductArgs: Decodable {
  let id: String
}

class ScoreArgs: Decodable {
  let board: String
  let value: Int
}

class AchievementArgs: Decodable {
  let ids: [String]
}

class KeyArgs: Decodable {
  let key: String
}

class KeyValueArgs: Decodable {
  let key: String
  let value: String
}

class ApplePlugin: Plugin {
  // ---------- the purchase (StoreKit 2) ----------

  /// The product's price as the store shows it to this player, e.g. "$0.99".
  @objc public func price(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ProductArgs.self)
    Task {
      do {
        let found = try await Product.products(for: [args.id])
        invoke.resolve(["price": found.first?.displayPrice ?? ""])
      } catch {
        invoke.resolve(["price": ""])
      }
    }
  }

  /// Whether this Apple ID owns the purchase now; a refund removes it.
  @objc public func owned(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ProductArgs.self)
    Task {
      invoke.resolve(["owned": await ApplePlugin.owns(args.id)])
    }
  }

  static func owns(_ id: String) async -> Bool {
    for await result in Transaction.currentEntitlements {
      if case .verified(let t) = result, t.productID == id, t.revocationDate == nil {
        return true
      }
    }
    return false
  }

  /// Buys the purchase: "owned", "cancelled", "pending" (Ask to Buy) or "failed".
  @objc public func buy(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ProductArgs.self)
    Task {
      do {
        guard let product = try await Product.products(for: [args.id]).first else {
          invoke.resolve(["result": "failed"])
          return
        }
        switch try await product.purchase() {
        case .success(let verification):
          if case .verified(let t) = verification {
            await t.finish()
            invoke.resolve(["result": "owned"])
          } else {
            invoke.resolve(["result": "failed"])
          }
        case .userCancelled:
          invoke.resolve(["result": "cancelled"])
        case .pending:
          invoke.resolve(["result": "pending"])
        @unknown default:
          invoke.resolve(["result": "failed"])
        }
      } catch {
        invoke.resolve(["result": "failed"])
      }
    }
  }

  /// Restore: asks the App Store for this Apple ID's purchases, then checks.
  @objc public func restore(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ProductArgs.self)
    Task {
      try? await AppStore.sync()
      invoke.resolve(["result": await ApplePlugin.owns(args.id) ? "owned" : "none"])
    }
  }

  // ---------- the rating prompt ----------

  /// Apple decides whether the prompt appears; it shows at most three times a year.
  @objc public func review(_ invoke: Invoke) throws {
    Task { @MainActor in
      if let scene = ApplePlugin.scene() {
        AppStore.requestReview(in: scene)
      }
      invoke.resolve()
    }
  }

  @MainActor static func scene() -> UIWindowScene? {
    UIApplication.shared.connectedScenes
      .compactMap { $0 as? UIWindowScene }
      .first { $0.activationState == .foregroundActive }
  }

  // ---------- Game Center ----------

  /// Signs the player in; Apple shows its own sheet the first time.
  @objc public func gameCenter(_ invoke: Invoke) throws {
    Task { @MainActor in
      GKLocalPlayer.local.authenticateHandler = { sheet, _ in
        if let sheet = sheet,
          let root = ApplePlugin.scene()?.windows.first(where: { $0.isKeyWindow })?.rootViewController
        {
          root.present(sheet, animated: true)
        }
      }
      invoke.resolve(["signedIn": GKLocalPlayer.local.isAuthenticated])
    }
  }

  @objc public func score(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ScoreArgs.self)
    Task {
      if GKLocalPlayer.local.isAuthenticated {
        try? await GKLeaderboard.submitScore(
          args.value, context: 0, player: GKLocalPlayer.local, leaderboardIDs: [args.board])
      }
      invoke.resolve()
    }
  }

  /// Reports achievements as complete; Game Center ignores ones already reported.
  @objc public func achieve(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(AchievementArgs.self)
    Task {
      if GKLocalPlayer.local.isAuthenticated && !args.ids.isEmpty {
        let list = args.ids.map { id -> GKAchievement in
          let a = GKAchievement(identifier: id)
          a.percentComplete = 100
          a.showsCompletionBanner = true
          return a
        }
        try? await GKAchievement.report(list)
      }
      invoke.resolve()
    }
  }

  // ---------- iCloud (the key-value store) ----------

  @objc public func cloudGet(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(KeyArgs.self)
    let store = NSUbiquitousKeyValueStore.default
    store.synchronize()
    invoke.resolve(["value": store.string(forKey: args.key) ?? ""])
  }

  @objc public func cloudSet(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(KeyValueArgs.self)
    let store = NSUbiquitousKeyValueStore.default
    store.set(args.value, forKey: args.key)
    store.synchronize()
    invoke.resolve()
  }
}

@_cdecl("init_plugin_apple")
func initPlugin() -> Plugin {
  return ApplePlugin()
}
