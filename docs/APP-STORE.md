# Putting Idlewright on the App Store from a Mac

A guide for the person (T17). It starts from this repository and ends with the
app in review. Steps marked **you** need your Apple ID, money or a judgement;
the rest are commands. Read each step to the end before starting it.

What is already built, in `app/`:

- **The Tauri 2 app.** It carries the web build's own files (`dist/`), so the
  game engine is the same WebAssembly file on the web and on the phone
  (IG-Q52 A). Rust in `app/src-tauri/src/lib.rs` keeps the save in a file on the
  phone and schedules the notifications the core lists.
- **The Apple plugin** (`app/src-tauri/apple/`). It handles the purchase
  (StoreKit 2), the rating prompt, Game Center and iCloud saves.
- **The core's side.** The Coast, the Peaks and the Wastes stay shut until the
  purchase. Buy and Restore are on More → Regions. A purchase sheet appears when
  the Coast is reached, rating requests come at three moments, and the
  notification times are worked out by the core.
- **The app icon:** `data/icons/app-icon-1024.png`, made by `tools/icons/app_icon.py`.

Not built or checked here, because this Mac has no Xcode: the iPhone build
itself. The Swift in `ApplePlugin.swift` has not been compiled. The first real
build is step 6, and a compile error there is expected work, not a surprise.

---

## 1. Join the Apple Developer Program (**you**, about a day)

1. Make sure your Apple ID has two-factor authentication on.
2. Enrol at <https://developer.apple.com/programs/enroll/>. Enrolling as an
   **individual** costs $99 a year, and your own name shows as the seller.
   Enrolling as an organisation needs a D-U-N-S number and takes longer.
3. Approval can take up to 48 hours.
4. In App Store Connect → **Business**, sign the **Paid Apps** agreement and
   fill in tax and banking. You need it even though the download is free,
   because the purchase is paid.
5. Optional, but worth it: apply to the **App Store Small Business Program**,
   so Apple keeps 15%, not 30%.

## 2. Install the tools (about an hour, mostly downloading)

1. **Xcode** from the Mac App Store. Since 28 April 2026, App Store Connect only
   accepts builds made with **Xcode 26 or later**
   ([Apple](https://developer.apple.com/news/upcoming-requirements/)). Open it
   once and accept the licence.
2. In Terminal:

   ```sh
   sudo xcode-select -s /Applications/Xcode.app
   xcodebuild -runFirstLaunch
   brew install xcodegen          # Tauri uses it to write the Xcode project
   ```

3. Rust's iPhone targets install themselves the first time you build in
   `app/`, because `app/rust-toolchain.toml` lists them.
4. The Tauri command-line tool is already installed (`cargo tauri --version`
   says 2.12.1).
5. In Xcode → Settings → Accounts, add your Apple ID and pick your team.

## 3. Register the app (**you**, 20 minutes)

1. At <https://developer.apple.com/account/resources/identifiers>, add an
   **App ID** with the bundle ID `io.github.sgilson7.idlewright`. This must
   match `identifier` in `app/src-tauri/tauri.conf.json`. Tick **Game Center**,
   **iCloud** and **In-App Purchase**.
2. In App Store Connect → **Apps** → **+** → New App:
   - Platform: iOS.
   - Name: **Idlewright: Idle Skilling RPG**.
   - Language: English (U.S.).
   - Bundle ID: the one above.
   - SKU: `idlewright`.
3. Under **Monetization → In-App Purchases**, add a **Non-Consumable**:
   - Product ID: `io.github.sgilson7.idlewright.regions`. This must match
     `PRODUCT` in `web/device.js`.
   - Price: $0.99.
   - Display name: “The Coast, the Peaks and the Wastes”.
   - A review screenshot of More → Regions.
4. Under **Game Center**:
   - Add a leaderboard with ID `total_level` (best score, high to low).
   - Add the achievements the core reports: `coast`, `peaks`, `wastes`,
     `first_prestige`, `legend`, `three_followers`.
   - Their names and pictures are yours to write. Game Center ignores any ID it
     does not know, so you can add these later.

## 4. Make the Xcode project (10 minutes)

```sh
cd app
export APPLE_DEVELOPMENT_TEAM=XXXXXXXXXX   # your Team ID, from developer.apple.com → Membership
cargo tauri ios init
```

This writes `app/src-tauri/gen/apple/`, which holds the Xcode project. Commit it
so you only do this once.

Then open the project with `cargo tauri ios open`, pick the `idlewright_iOS`
target, go to **Signing & Capabilities** and add three capabilities:

- **In-App Purchase**
- **Game Center**
- **iCloud**, with **Key-value storage** ticked

`app/src-tauri/Info.ios.plist` is merged in for you. It tells Apple the app
uses no encryption beyond the system's, so you skip the export-compliance
question on every upload.

## 5. Try it in the simulator (30 minutes)

```sh
cd app
cargo tauri ios dev                 # pick an iPhone simulator when asked
```

Check each of these:

- **The game itself:** it starts on the woodcutting tab, the scene animates
  and sounds play after the first tap.
- **Offline:** with the network off, it still plays. Reviewers try airplane
  mode.
- **Saves:** close and reopen the app, and your progress is still there (the
  save is a file now, not browser storage).
- **The purchase:**
  1. Create a StoreKit configuration file in Xcode: File → New → File →
     StoreKit Configuration File, synced from App Store Connect.
  2. Choose it under Product → Scheme → Edit Scheme → Run → Options.
  3. More → Regions → **Buy for $0.99** should open Apple's sheet, and after
     buying the regions open with a toast.
  4. **Restore purchase** should find it again.
- **Notifications:** start a caravan or plant a crop, then go to the home
  screen. The notification should arrive when it is due.

## 6. Try it on your iPhone (30 minutes)

1. Plug the phone in and trust the Mac.
2. Turn on Developer Mode: Settings → Privacy & Security → Developer Mode.
3. Run `cargo tauri ios dev --open` and choose the phone in Xcode.
4. Purchases on a real phone use a **Sandbox** Apple ID. Make one in App Store
   Connect → Users and Access → Sandbox, and sign in under Settings → App
   Store → Sandbox Account.

## 7. Make the store page (**you**, an afternoon)

The research behind each choice is on the journal's App stores tab.

| Field | What to put |
| --- | --- |
| Name (30) | Idlewright: Idle Skilling RPG |
| Subtitle (30) | e.g. “Chop, mine, fight, offline” (searched, so use words players search for) |
| Keywords (100) | comma-separated, no spaces, no competitors' names: `idle,skilling,rpg,incremental,offline,afk,crafting,mining,woodcutting,fishing,bard,adventure` |
| Description | what it is in the first two lines; Apple does not search it |
| Category | Games → Role Playing; second: Games → Simulation |
| Age rating | answer the questionnaire; cartoon fighting is “infrequent/mild” |
| Price | Free (the regions are the $0.99 purchase) |
| Privacy | “Data Not Collected”: the game sends nothing anywhere. Game Center and iCloud are Apple's. A privacy-policy URL is still required; a page on the GitHub Pages site saying so is enough |
| Support URL | the GitHub Pages site |

**Screenshots:** iPhone 6.9″ (1320 × 2868) and, because the app runs on iPad,
13″ iPad (2064 × 2752). Up to ten each; the first three matter most. Take them
in the simulator (an iPhone 16 Pro Max and an iPad Pro 13-inch) with
File → Save Screen. Good first three:

1. The woodcutting scene with a full skill card.
2. An advanced plan with the Now panel.
3. A Coast or Wastes fight, with the combat log.

**Preview video** (optional, 15 to 30 seconds): record the simulator with
`xcrun simctl io booted recordVideo preview.mov`.

## 8. Build and upload (30 minutes)

```sh
cd app
cargo tauri ios build --export-method app-store-connect
```

This runs `scripts/build.sh` first, so the app carries the same `dist/` that the
web has. To see that the engine matches the published web game:

```sh
./app/same-engine.sh
```

The `.ipa` lands in `app/src-tauri/gen/apple/build/`. Upload it with Apple's
free **Transporter** app (Mac App Store): sign in, drag the `.ipa` in, and press
Deliver. Processing takes 10 to 30 minutes, and then the build appears under
the app's **TestFlight** tab.

## 9. TestFlight first (**you**, a few days)

Add yourself and your co-designer as internal testers. Play a day on real
phones, buying with a sandbox account. This is also the Google-style closed
test the lit review recommended, done Apple's way.

## 10. Submit for review (**you**)

1. On the app's page, pick the build.
2. Attach the in-app purchase to this version.
3. Fill in **App Review Information**, with a note like this:

   > The Vale is free to play in full. The one in-app purchase (non-consumable)
   > opens three further regions; it is offered on More → Regions from the
   > start, with Restore beside it. The game runs offline; it uses local
   > notifications for crops, caravans, research and time away, Game Center,
   > and iCloud key-value storage to carry a save between devices. No account
   > or login.

4. Submit. A first review usually takes a day or two.

## If it is rejected

| Guideline | What it means here | What to do |
| --- | --- | --- |
| 4.2 Minimum functionality | “a repackaged website” | Point to offline play, notifications, Game Center, iCloud, the purchase; none of these is on the web |
| 2.5.2 Code | downloading code that changes the app | The engine is bundled, never downloaded; say so |
| 3.1.1 Purchases | a missing restore, or a purchase outside Apple's | Restore is on More → Regions, beside Buy |
| 2.1 Completeness | the reviewer could not find the purchase | The review note says where it is |

## Updating later

Change the game, run `./scripts/check.sh`, publish the web with
`./scripts/publish.sh`, then repeat steps 8 and 10 with a higher `version` in
`tauri.conf.json`. Both get the same engine from the same commit.
